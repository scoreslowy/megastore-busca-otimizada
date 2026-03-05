# Sistema de Busca Otimizado para Catálogo de Produtos – MegaStore

> Trabalho acadêmico – disciplina **Data Structure Strategy and Implementation** (FECAF)

## 1 O que é este projeto
A proposta é implementar um **sistema de busca** para um catálogo grande (cenário: milhões de itens), evitando varredura completa do catálogo a cada consulta.
Além da busca, o projeto inclui **recomendação de produtos** baseada em **grafo de similaridade** (conteúdo/tokens).

O foco é **estrutura de dados + estratégia**, então a implementação foi mantida enxuta, mas com:
- índice invertido em memória (HashMap);
- ranking de relevância (BM25 simplificado);
- filtros por marca e categoria;
- recomendação por grafo (similaridade de tokens);
- CLI para demonstrar execução;
- testes automatizados.

## 2 Como eu abordei o desafio (metodologia)
### 2.1 Entendimento do problema
O problema original era “busca lenta e imprecisa”. Para atacar isso, foquei em duas frentes:
1) **Indexar** (pré-processar) o catálogo para não precisar varrer tudo em cada consulta;
2) **Rankear** os resultados por relevância em vez de devolver qualquer match.

### 2.2 Pesquisa (exemplos do mercado)
Antes de implementar, consultei soluções usadas no mercado e os conceitos por trás:
- **Elasticsearch/Lucene**: usa índice invertido e modelos de relevância como **BM25**.
  - Elastic (BM25): https://www.elastic.co/blog/practical-bm25-part-2-the-bm25-algorithm-and-its-variables
- **OpenSearch**: alternativa open-source/gerenciada com foco em busca e analytics.
  - OpenSearch Docs: https://docs.opensearch.org/latest/
  - AWS OpenSearch: https://docs.aws.amazon.com/opensearch-service/

Este repositório não é um clone de Elasticsearch; ele aplica a mesma base (índice invertido + ranking) em uma implementação educacional em Rust.

### 2.3 Entrega
O repositório contém:
- `src/`: implementação em Rust (módulos de indexação, busca, grafo e CLI);
- `tests/`: testes automatizados (integração/funcionais);
- `Cargo.toml` / `Cargo.lock`: dependências;
- `docs/`: PDF da documentação (padrão solicitado).

## 3 Tecnologias e bibliotecas
- Rust (edition 2021)
- crates principais:
  - `hashbrown` (HashMap/HashSet)
  - `rayon` (paralelismo)
  - `serde` / `serde_json` (carregamento do catálogo)
  - `clap` (CLI)
  - `anyhow` / `thiserror` (erros)
- testes: `cargo test`, `assert_cmd`, `predicates`

## 4 Como executar (Windows)
### 4.1 Pré-requisitos
- Rust toolchain (rustup).
- No Windows: foi necessário instalar **Visual Studio Build Tools** com **Desktop development with C++** para disponibilizar o `link.exe` (sem isso o projeto não compila).

### 4.2 Compilar
```bash
cargo build --release
```

### 4.3 Rodar busca (dataset de exemplo)
```bash
cargo run --release -- search --data data/sample_products.jsonl --q "iphone 128gb" --top-k 5
```

### 4.4 Rodar recomendação (grafo)
```bash
cargo run --release -- recommend --data data/sample_products.jsonl --id 1 --top-k 5
```

### 4.5 Benchmark (sintético)
O comando `bench` gera um catálogo sintético (N produtos), monta o índice e constrói o grafo para medir tempos.

```bash
cargo run --release -- bench --n 5000 --q "smartphone camera" --top-k 10
cargo run --release -- bench --n 10000 --q "smartphone camera" --top-k 10
```

Observação real do meu ambiente: valores muito altos (ex.: `n=50000`) podem falhar por falta de memória.

## 5 Testes
Executar testes automatizados:
```bash
cargo test
```

## 6 Arquitetura (visão rápida)
- `models`: domínio (Product, ProductId)
- `text`: tokenização/normalização
- `index`: índice invertido e filtros (marca/categoria)
- `search`: ranking (BM25 simplificado)
- `graph`: grafo de similaridade (recomendação)
- `engine`: fachada (busca + recomendação)
- `main`: CLI

## 7 Resultados (execução local)
### 7.1 Testes
- `cargo test`: 3 testes aprovados (2 de CLI e 1 de busca).

### 7.2 Benchmark (saída real do terminal)
Build: `cargo run --release`

N=5000  
- indexação: 30.8898ms  
- grafo: 664.1946ms  
- busca: 52.1µs  

N=10000  
- indexação: 48.339ms  
- grafo: 9.4451769s  
- busca: 157.9µs  

Interpretação curta: busca/indexação ficaram rápidas; a construção do grafo cresce mais rápido com N e vira o gargalo. Em produção, faria construção incremental e/ou limitaria candidatos e/ou geraria recomendação sob demanda.

## 8 Documentação (PDF)
- `docs/MegaStore_Documentacao.pdf`

## 9 Referências (links)
- Elastic. *Practical BM25 – Part 2*. https://www.elastic.co/blog/practical-bm25-part-2-the-bm25-algorithm-and-its-variables (acesso em 04/03/2026)
- OpenSearch. *Documentation*. https://docs.opensearch.org/latest/ (acesso em 04/03/2026)
- AWS. *Amazon OpenSearch Service Documentation*. https://docs.aws.amazon.com/opensearch-service/ (acesso em 04/03/2026)
