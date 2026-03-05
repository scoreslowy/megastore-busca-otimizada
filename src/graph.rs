use crate::index::InvertedIndex;
use crate::models::ProductId;
use hashbrown::HashMap;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct RecGraph {
    // lista de adjacencia: produto -> (produto similar, score)
    pub adj: HashMap<ProductId, Vec<(ProductId, f32)>>,
}

impl RecGraph {
    /// Constroi um grafo de similaridade usando Jaccard sobre conjuntos de tokens.
    ///
    /// Importante: evita O(n^2) gerando candidatos via inverted lists:
    /// para cada termo do produto A, considera documentos que compartilham o termo.
    pub fn build(index: &InvertedIndex, top_n_per_node: usize) -> Self {
        let top_n = top_n_per_node.max(1);

        // termo -> lista de docs (reaproveita index.postings, mas transforma em ids)
        let term_docs: HashMap<&str, Vec<ProductId>> = index.postings.iter()
            .map(|(t, plist)| (t.as_str(), plist.iter().map(|p| p.doc).collect()))
            .collect();

        let ids: Vec<ProductId> = index.products.keys().copied().collect();

        let pairs: Vec<(ProductId, Vec<(ProductId, f32)>)> = ids.par_iter().map(|&id_a| {
            let set_a = index.token_sets.get(&id_a).expect("token set");
            let len_a = set_a.len() as f32;

            // conta intersecoes com candidatos
            let mut inter: HashMap<ProductId, u32> = HashMap::new();
            for term in set_a.iter() {
                if let Some(docs) = term_docs.get(term.as_str()) {
                    for &id_b in docs {
                        if id_b == id_a { continue; }
                        *inter.entry(id_b).or_insert(0) += 1;
                    }
                }
            }

            // calcula jaccard e pega top_n
            let mut scored: Vec<(ProductId, f32)> = Vec::new();
            for (id_b, inter_cnt) in inter {
                // Evita "temporary value dropped while borrowed" ao nao retornar referencia
                // para um HashSet temporario.
                let len_b = index
                    .token_sets
                    .get(&id_b)
                    .map(|s| s.len() as f32)
                    .unwrap_or(0.0);
                let inter_f = inter_cnt as f32;
                let union = (len_a + len_b - inter_f).max(1.0);
                let j = inter_f / union;
                if j > 0.0 {
                    scored.push((id_b, j));
                }
            }
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            scored.truncate(top_n);
            (id_a, scored)
        }).collect();

        let mut adj = HashMap::new();
        for (id, list) in pairs {
            adj.insert(id, list);
        }
        Self { adj }
    }

    pub fn recommend(&self, id: ProductId, top_k: usize) -> Vec<(ProductId, f32)> {
        self.adj.get(&id).cloned().unwrap_or_default().into_iter().take(top_k.max(1)).collect()
    }
}
