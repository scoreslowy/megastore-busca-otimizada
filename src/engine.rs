use crate::graph::RecGraph;
use crate::index::InvertedIndex;
use crate::models::{Product, ProductId};
use crate::search::{search, SearchParams};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Engine {
    pub index: InvertedIndex,
    pub graph: RecGraph,
}

impl Engine {
    pub fn from_products(products: Vec<Product>) -> Self {
        let index = InvertedIndex::build(products);
        let graph = RecGraph::build(&index, 10);
        Self { index, graph }
    }

    pub fn search(&self, params: SearchParams) -> Vec<(ProductId, f32)> {
        search(&self.index, &params)
    }

    pub fn recommend(&self, id: ProductId, top_k: usize) -> Vec<(ProductId, f32)> {
        self.graph.recommend(id, top_k)
    }

    pub fn product(&self, id: ProductId) -> Option<&Product> {
        self.index.get(id)
    }

    /// Pequeno benchmark end-to-end com dados sinteticos
    pub fn bench(n: usize, query: &str, top_k: usize) -> Result<BenchReport> {
        let products = crate::storage::generate_synthetic(n, 42);
        let t0 = std::time::Instant::now();
        let index = InvertedIndex::build(products);
        let t_index = t0.elapsed();

        let t1 = std::time::Instant::now();
        let graph = RecGraph::build(&index, 10);
        let t_graph = t1.elapsed();

        let engine = Engine { index, graph };

        let t2 = std::time::Instant::now();
        let _ = engine.search(SearchParams {
            q: query.to_string(),
            brand: None,
            category: None,
            top_k,
        });
        let t_search = t2.elapsed();

        Ok(BenchReport { n, t_index, t_graph, t_search })
    }
}

#[derive(Debug, Clone)]
pub struct BenchReport {
    pub n: usize,
    pub t_index: std::time::Duration,
    pub t_graph: std::time::Duration,
    pub t_search: std::time::Duration,
}
