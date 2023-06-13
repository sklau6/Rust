// Import necessary modules and libraries
use chrono::prelude::*;
use crypto_hash::{Algorithm, hex_digest};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt::{self, Debug, Formatter};

// Define the Block structure
#[derive(Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

// Implement methods for the Block structure
impl Block {
    // Create a new block with index, timestamp, data, and the hash of the previous block
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

    // Calculate the hash of a block
    fn hash(index: u64, timestamp: i64, data: &str, previous_hash: &str) -> String {
        let content = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        hex_digest(Algorithm::SHA256, content.as_bytes())
    }

    // Check if a block is valid by comparing its index and hash with the previous block
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

// Define the Blockchain structure
struct Blockchain {
    blocks: Vec<Block>,
}

// Implement methods for the Blockchain structure
impl Blockchain {
    // Create a new blockchain with a genesis block
    fn new() -> Self {
        let genesis_block = Block::new(0, Utc::now().timestamp(), "Genesis Block", "0");
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    // Add a block to the blockchain
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

    // Check if the blockchain is valid
    fn is_valid(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate().skip(1) {
            if !block.is_valid(&self.blocks[i - 1]) {
                return false;
            }
        }
        true
    }
}

// Implement debug trait for the Blockchain structure
impl Debug for Blockchain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", serde_json::to_string_pretty(&self.blocks).unwrap())
    }
}

// Main function to drive the program
fn main() {
    let mut blockchain = Blockchain::new();

    // Print the initial blockchain
    println!("Initial blockchain: {:?}", blockchain);

    // Add blocks to the blockchain
    blockchain.add_block("First block");
    blockchain.add_block("Second block");
    blockchain.add_block("Third block");

    // Print the blockchain after adding blocks
    println!("After adding blocks: {:?}", blockchain);
}
   
