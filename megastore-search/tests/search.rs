use megastore_search::{engine::Engine, search::SearchParams};
use megastore_search::storage::load_products;

#[test]
fn search_prefers_matching_terms() {
    let products = load_products("data/sample_products.jsonl").expect("falha ao carregar");
    let engine = Engine::from_products(products);

    let res = engine.search(SearchParams {
        q: "notebook i7".to_string(),
        brand: None,
        category: None,
        top_k: 3,
    });

    assert!(!res.is_empty(), "resultado vazio");

    let top = engine.product(res[0].0).unwrap();
    assert!(top.name.to_lowercase().contains("notebook"));
}