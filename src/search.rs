use crate::index::InvertedIndex;
use crate::models::{ProductId};
use crate::text::tokenize;
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub q: String,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub top_k: usize,
}

/// BM25 simplificado
fn bm25(tf: f32, idf: f32, doc_len: f32, avg_doc_len: f32) -> f32 {
    let k1 = 1.2_f32;
    let b = 0.75_f32;
    let denom = tf + k1 * (1.0 - b + b * (doc_len / avg_doc_len.max(1e-6)));
    idf * ((tf * (k1 + 1.0)) / denom.max(1e-6))
}

pub fn search(index: &InvertedIndex, params: &SearchParams) -> Vec<(ProductId, f32)> {
    let terms = tokenize(&params.q);
    if terms.is_empty() || index.doc_count == 0 {
        return vec![];
    }

    let mut scores: HashMap<ProductId, f32> = HashMap::new();

    for term in terms {
        let idf = *index.idf.get(&term).unwrap_or(&0.0);
        if let Some(plist) = index.postings.get(&term) {
            for p in plist {
                let doc_len = *index.doc_len.get(&p.doc).unwrap_or(&0) as f32;
                let tf = p.tf as f32;
                let s = bm25(tf, idf, doc_len, index.avg_doc_len);
                *scores.entry(p.doc).or_insert(0.0) += s;
            }
        }
    }

    // aplica filtros (brand/category)
    let mut results: Vec<(ProductId, f32)> = scores.into_iter().collect();
    if params.brand.is_some() || params.category.is_some() {
        results.retain(|(id, _)| {
            let Some(prod) = index.get(*id) else { return false; };
            if let Some(b) = &params.brand {
                if &prod.brand != b { return false; }
            }
            if let Some(c) = &params.category {
                if &prod.category != c { return false; }
            }
            true
        });
    }

    // ordena por score desc
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    results.truncate(params.top_k.max(1));
    results
}
