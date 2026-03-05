use crate::models::{Product, ProductId};
use crate::text::tokenize;
use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Posting {
    pub doc: ProductId,
    pub tf: u32,
}

#[derive(Debug, Clone)]
pub struct InvertedIndex {
    pub products: HashMap<ProductId, Product>,
    pub postings: HashMap<String, Vec<Posting>>, // termo -> lista de (doc, tf)
    pub doc_len: HashMap<ProductId, usize>,
    pub doc_count: usize,
    pub avg_doc_len: f32,
    pub idf: HashMap<String, f32>,
    pub by_brand: HashMap<String, Vec<ProductId>>,
    pub by_category: HashMap<String, Vec<ProductId>>,
    pub token_sets: HashMap<ProductId, HashSet<String>>, // usado pelo grafo de recomendacao
}

impl InvertedIndex {
    pub fn build(products: Vec<Product>) -> Self {
        let mut prod_map: HashMap<ProductId, Product> = HashMap::new();
        for p in products {
            prod_map.insert(p.id, p);
        }

        // Constrói índices em paralelo
        let docs: Vec<ProductId> = prod_map.keys().copied().collect();

        let doc_tokens: Vec<(ProductId, Vec<String>)> = docs
            .par_iter()
            .map(|&id| {
                let p = prod_map.get(&id).expect("product exists");
                let tokens = tokenize(&p.full_text());
                (id, tokens)
            })
            .collect();

        let mut postings: HashMap<String, Vec<Posting>> = HashMap::new();
        let mut doc_len: HashMap<ProductId, usize> = HashMap::new();
        let mut token_sets: HashMap<ProductId, HashSet<String>> = HashMap::new();

        for (doc, tokens) in &doc_tokens {
            doc_len.insert(*doc, tokens.len());
            token_sets.insert(*doc, tokens.iter().cloned().collect());

            // conta TF por termo no documento
            let mut tf_map: HashMap<&str, u32> = HashMap::new();
            for t in tokens {
                *tf_map.entry(t.as_str()).or_insert(0) += 1;
            }
            for (term, tf) in tf_map {
                postings.entry(term.to_string()).or_default().push(Posting { doc: *doc, tf });
            }
        }

        // Índices auxiliares
        let mut by_brand: HashMap<String, Vec<ProductId>> = HashMap::new();
        let mut by_category: HashMap<String, Vec<ProductId>> = HashMap::new();
        for (id, p) in &prod_map {
            by_brand.entry(p.brand.clone()).or_default().push(*id);
            by_category.entry(p.category.clone()).or_default().push(*id);
        }

        let doc_count = prod_map.len();
        let total_len: usize = doc_len.values().sum();
        let avg_doc_len = if doc_count == 0 { 0.0 } else { (total_len as f32) / (doc_count as f32) };

        // IDF por termo
        let mut idf: HashMap<String, f32> = HashMap::new();
        for (term, plist) in &postings {
            let df = plist.len() as f32;
            let n = doc_count as f32;
            // idf BM25: ln(1 + (N - df + 0.5)/(df + 0.5))
            let val = ((n - df + 0.5) / (df + 0.5) + 1.0).ln();
            idf.insert(term.clone(), val);
        }

        Self {
            products: prod_map,
            postings,
            doc_len,
            doc_count,
            avg_doc_len,
            idf,
            by_brand,
            by_category,
            token_sets,
        }
    }

    pub fn get(&self, id: ProductId) -> Option<&Product> {
        self.products.get(&id)
    }
}
