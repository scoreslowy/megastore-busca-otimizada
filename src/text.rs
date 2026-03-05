use hashbrown::HashSet;

/// Tokeniza texto em termos normalizados.
/// Regras:
/// lowercase
///  split por caracteres nao alfanumericos
///  remove tokens muito curtos (len < 2)
pub fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.len() >= 2)
        .map(|t| t.to_string())
        .collect()
}

pub fn to_token_set(text: &str) -> HashSet<String> {
    tokenize(text).into_iter().collect()
}
