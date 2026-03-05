use crate::models::Product;
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn load_products<P: AsRef<Path>>(path: P) -> Result<Vec<Product>> {
    let path = path.as_ref();
    let f = File::open(path).with_context(|| format!("falha ao abrir arquivo: {}", path.display()))?;
    let mut reader = BufReader::new(f);

    // detecta se é JSON array ou JSONL
    let mut first = String::new();
    reader.read_line(&mut first)?;
    let first_trim = first.trim_start();

    // reabre 
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    if first_trim.starts_with('[') {
        let v: Value = serde_json::from_reader(reader)?;
        let prods: Vec<Product> = serde_json::from_value(v)?;
        Ok(prods)
    } else {
        let mut out = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() { continue; }
            let p: Product = serde_json::from_str(&line)?;
            out.push(p);
        }
        Ok(out)
    }
}

/// Gera um dataset sintetico (para benchmark local e testes de performance)
pub fn generate_synthetic(n: usize, seed: u64) -> Vec<Product> {
    use rand::prelude::*;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let brands = ["Apple", "Samsung", "MegaWear", "HomePro", "MegaCompute", "Foodies"];
    let cats = ["Eletrônicos", "Vestuário", "Casa", "Informática", "Alimentos"];
    let nouns = ["smartphone", "notebook", "camiseta", "cafeteira", "fone", "tv", "mouse", "teclado", "calca", "sofa", "arroz", "feijao"];
    let attrs = ["premium", "pro", "max", "lite", "ultra", "classic", "2026", "novo", "oficial", "importado"];

    let mut prods = Vec::with_capacity(n);
    for i in 0..n {
        let brand = brands[rng.gen_range(0..brands.len())].to_string();
        let category = cats[rng.gen_range(0..cats.len())].to_string();
        let a = nouns[rng.gen_range(0..nouns.len())];
        let b = attrs[rng.gen_range(0..attrs.len())];
        let c = attrs[rng.gen_range(0..attrs.len())];
        let name = format!("{} {} {}", brand, a, b);
        let description = format!("Produto {} com caracteristicas {} e {} para uso diario.", a, b, c);
        let tags = vec![a.to_string(), b.to_string(), c.to_string()];
        prods.push(Product {
            id: (i as u64) + 1,
            name,
            brand,
            category,
            description,
            tags,
        });
    }
    prods
}
