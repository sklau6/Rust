# Simple Blockchain in Rust (v2)
This repository contains a simple implementation of a blockchain in Rust. It is designed for educational purposes to provide a clear understanding of the basic principles behind blockchain technology.

## Project Structure
This blockchain implementation uses two main structures:
- `Block`: This represents a block in the blockchain. Each block has an ID, a nonce, some data, a hash, the previous block's hash, and a timestamp.
- `BlockChain`: This is a simple blockchain consisting of a vector of blocks.

## Features
The codebase contains the following features:
- `Block` and `BlockChain` structs that represent a blockchain and the blocks in it.
- A method for mining a new block.
- A method for validating the integrity of a new block and checking whether it can be added to the blockchain.
- A method for validating the integrity of the entire blockchain.
- A method for determining whether to use a local copy of the blockchain or a remote one based on their validity and length.

## Usage
1. Install [Rust](https://www.rust-lang.org/tools/install) on your machine.
2. Clone this repository
