# zk-storage

# Verifiable Data Storage with RISC Zero and Merkle Tree

## Overview

This project aims to create a verifiable data storage system where data integrity can be proven using zero-knowledge proofs (RISC Zero) and Merkle trees.

## Components

- Data Storage: A simple vector storage.
- Merkle Tree: To ensure data integrity.
- RISC Zero: To generate and verify zero-knowledge proofs.

## Why not just merkle tree?

It's because my goal was to create a proof of data integrity without revealing any information about the stored elements, not even their hashes. What's more, in the future, the storage can be extended with some features such as history of operations, proof of the same data properties, etc.