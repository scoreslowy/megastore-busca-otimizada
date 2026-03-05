use assert_cmd::Command;
use predicates::str::contains;
// Dataset pequeno só para validar o fluxo da CLI na correção.
#[test]
fn cli_search_returns_iphone() {
    let mut cmd = Command::cargo_bin("megastore_search").unwrap();
    cmd.args([
        "search",
        "--data", "data/sample_products.jsonl",
        "--q", "iphone 128gb",
        "--top-k", "3"
    ]);
    cmd.assert()
        .success()
        .stdout(contains("iPhone 14 128GB"));
}

#[test]
fn cli_recommend_runs() {
    let mut cmd = Command::cargo_bin("megastore_search").unwrap();
    cmd.args([
        "recommend",
        "--data", "data/sample_products.jsonl",
        "--id", "1",
        "--top-k", "3"
    ]);
    cmd.assert().success().stdout(contains("Base: #1"));
}
