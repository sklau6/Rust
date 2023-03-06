use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Error getting timestamp")
            .as_secs();
        let hash = Block::calculate_hash(index, &previous_hash, timestamp, &data);
        
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
    
    fn calculate_hash(index: u32, previous_hash: &str, timestamp: u64, data: &str) -> String {
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}{}", index, previous_hash, timestamp, data);
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis block".to_owned(), "0".to_owned());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }
    
    fn add_block(&mut self, data: String) {
        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let index = self.blocks.len() as u32;
        let block = Block::new(index, data, previous_hash);
        self.blocks.push(block);
    }
    
    fn is_valid(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.hash != Block::calculate_hash(block.index, &block.previous_hash, block.timestamp, &block.data) {
                return false;
            }
            if i > 0 && block.previous_hash != self.blocks[i - 1].hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Hello, world!".to_owned());
    blockchain.add_block("This is a blockchain implemented in Rust".to_owned());
    
    println!("{:#?}", blockchain);
    println!("Is valid? {}", blockchain.is_valid());
}
