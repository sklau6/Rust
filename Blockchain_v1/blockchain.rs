use chrono::prelude::*;
use crypto_hash::{Algorithm, hex_digest};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt::{self, Debug, Formatter};

#[derive(Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, timestamp: i64, data: &str, previous_hash: &str) -> Self {
        let hash = Block::hash(index, timestamp, data, previous_hash);
        Block {
            index,
            timestamp,
            data: data.to_string(),
            previous_hash: previous_hash.to_string(),
            hash,
        }
    }

    fn hash(index: u64, timestamp: i64, data: &str, previous_hash: &str) -> String {
        let content = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        hex_digest(Algorithm::SHA256, content.as_bytes())
    }

    fn is_valid(&self, previous_block: &Block) -> bool {
        if self.index != previous_block.index + 1 {
            return false;
        }
        if self.previous_hash != previous_block.hash {
            return false;
        }
        if self.hash != Block::hash(self.index, self.timestamp, &self.data, &self.previous_hash) {
            return false;
        }
        true
    }
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, Utc::now().timestamp(), "Genesis Block", "0");
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: &str) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            Utc::now().timestamp(),
            data,
            &previous_block.hash,
        );
        self.blocks.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate().skip(1) {
            if !block.is_valid(&self.blocks[i - 1]) {
                return false;
            }
        }
        true
    }
}

impl Debug for Blockchain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", serde_json::to_string_pretty(&self.blocks).unwrap())
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    println!("Initial blockchain: {:?}", blockchain);

    blockchain.add_block("First block");
    blockchain.add_block("Second block");
    blockchain.add_block("Third block");

    println!("After adding blocks: {:?}", blockchain);

    assert!(blockchain.is_valid(), "Blockchain should be valid");

    println!("Blockchain is valid: {}", blockchain.is_valid());

    // Tampering with the data
    blockchain.blocks[2].data = "Tampered data".to_string();

    println!("After tampering: {:?}", blockchain);

    assert!(!blockchain.is_valid(), "Blockchain should be invalid after tampering");

    println!("Blockchain is valid: {}", blockchain.is_valid());
}
