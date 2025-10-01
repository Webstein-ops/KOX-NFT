use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use uuid::Uuid;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
    pub rarity: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NFT {
    pub id: String,
    pub owner: String,
    pub metadata: Metadata,
    pub hash: String,
}

impl NFT {
    pub fn new(owner: &str, metadata: Metadata) -> Self {
        let id = Uuid::new_v4().to_string();
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{:?}", id, owner, metadata));
        let hash = format!("{:x}", hasher.finalize());

        NFT {
            id,
            owner: owner.to_string(),
            metadata,
            hash,
        }
    }

    pub fn transfer(&mut self, new_owner: &str) {
        self.owner = new_owner.to_string();
    }

    pub fn save_metadata(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self.metadata).unwrap();
        fs::write(path, json).unwrap();
    }
}
