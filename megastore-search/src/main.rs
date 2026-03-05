use anyhow::Result;
use clap::{Parser, Subcommand};
use megastore_search::engine::Engine;
use megastore_search::search::SearchParams;
use megastore_search::storage::load_products;

#[derive(Parser, Debug)]
#[command(name="megastore-search", version, about="Sistema de busca + recomendacao (MegaStore)")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Executa uma busca no catalogo
    Search {
        /// Caminho do arquivo JSON array ou JSONL
        #[arg(long)]
        data: String,
        /// Texto da consulta
        #[arg(long, short='q')]
        q: String,
        /// Filtro por marca 
        #[arg(long)]
        brand: Option<String>,
        /// Filtro por categoria 
        #[arg(long)]
        category: Option<String>,
        /// Quantidade de resultados
        #[arg(long, default_value_t=10)]
        top_k: usize,
    },
    /// Recomenda produtos similares a partir de um ID
    Recommend {
        #[arg(long)]
        data: String,
        #[arg(long)]
        id: u64,
        #[arg(long, default_value_t=10)]
        top_k: usize,
    },
    /// Benchmark simples com dados sinteticos
    Bench {
        #[arg(long, default_value_t=50000)]
        n: usize,
        #[arg(long, short='q', default_value="smartphone camera")]
        q: String,
        #[arg(long, default_value_t=10)]
        top_k: usize,
    },
    /// Mostra estatisticas da indexacao
    Stats {
        #[arg(long)]
        data: String,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Search { data, q, brand, category, top_k } => {
            let products = load_products(data)?;
            let engine = Engine::from_products(products);
            let results = engine.search(SearchParams { q, brand, category, top_k });
            for (id, score) in results {
                let p = engine.product(id).unwrap();
                println!("#{id} | score={score:.4} | {} | {} | {}", p.name, p.brand, p.category);
            }
        }
        Commands::Recommend { data, id, top_k } => {
            let products = load_products(data)?;
            let engine = Engine::from_products(products);
            let base = engine.product(id);
            if let Some(p) = base {
                println!("Base: #{id} | {} | {} | {}", p.name, p.brand, p.category);
            } else {
                println!("Produto #{id} nao encontrado.");
                return Ok(());
            }
            let recs = engine.recommend(id, top_k);
            for (rid, score) in recs {
                if let Some(p) = engine.product(rid) {
                    println!("- #{rid} | sim={score:.3} | {} | {} | {}", p.name, p.brand, p.category);
                }
            }
        }
        Commands::Bench { n, q, top_k } => {
            let report = Engine::bench(n, &q, top_k)?;
            println!("Benchmark (sintetico)");
            println!("- n: {}", report.n);
            println!("- indexacao: {:?}", report.t_index);
            println!("- grafo: {:?}", report.t_graph);
            println!("- busca: {:?}", report.t_search);
        }
        Commands::Stats { data } => {
            let products = load_products(data)?;
            let engine = Engine::from_products(products);
            println!("Docs: {}", engine.index.doc_count);
            println!("Termos distintos: {}", engine.index.postings.len());
            println!("Avg doc len: {:.2}", engine.index.avg_doc_len);
        }
    }

    Ok(())
}
