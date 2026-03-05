use serde::{Deserialize, Serialize};

pub type ProductId = u64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Product {
    pub fn full_text(&self) -> String {
        // Texto usado para indexação
        let mut s = String::new();
        s.push_str(&self.name);
        s.push(' ');
        s.push_str(&self.brand);
        s.push(' ');
        s.push_str(&self.category);
        s.push(' ');
        s.push_str(&self.description);
        for t in &self.tags {
            s.push(' ');
            s.push_str(t);
        }
        s
    }
}
