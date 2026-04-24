# Project Title

Patent Royalties – IP Royalty Distribution on Stellar

## Project Vision

This project demonstrates how to build a Soroban smart contract for managing **Intellectual Property (IP) royalties on the Stellar blockchain**. It provides a transparent, trustless system where:
- IP owners can register patents
- Licensors can issue licenses with royalty terms
- Licensees can pay royalties on-chain
- Patent owners can claim accumulated royalties

This serves as a foundation for royalty management systems, IP licensing platforms, and creator-economy dApps on Stellar.

---

## Description

A Soroban smart contract dApp that enables **IP royalty distribution on Stellar**. The contract manages patent registration, license creation with royalty amounts, royalty payments, and royalty claims — all stored permanently on-chain with full transparency.

---

## Features

### 1. Patent Registration
- IP owner registers a patent with a unique ID and title
- On-chain ownership tracking
- Immutable record of IP rights

### 2. License Management
- Licensor creates licenses for registered patents
- Royalty amount set per license
- Track paid vs unpaid royalties per licensee

### 3. Royalty Payment
- Licensee pays royalty for a patent
- On-chain payment tracking
- Accumulated royalties tracked per patent owner

### 4. Royalty Claims
- Patent owner claims accumulated royalties
- Transparent accounting of all transactions

### 5. On-chain Transparency
- All patents, licenses, and payments stored permanently on Stellar
- Anyone can verify ownership and payment history

---

## Contract Functions

| Function | Description |
|----------|-------------|
| `register_patent(patent_id, owner, title)` | IP owner registers a patent |
| `create_license(patent_id, licensee, royalty_amount)` | Licensor creates a license |
| `pay_royalty(patent_id, licensee)` | Licensee pays royalty |
| `claim_royalties(patent_id)` | Owner claims accumulated royalties |
| `get_patent(patent_id)` | Returns (owner, title) |
| `get_license(patent_id, licensee)` | Returns (royalty_amount, paid) |

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CBFJGBFP365IMSPGYZEDGNCH2VNAKM2NY62ZXNHKQOSQWZAUVRTV6V4Y](https://stellar.expert/explorer/testnet/tx/00c848061b04752a30a3f585e1d28e6d4e93e5f2e307bf207e1e4bca611dba64)

![screenshot](https://i.ibb.co/jkCWY0Fs/image.png)

---


## Future Scopes

### 1. Royalty Tokenization
- Issue ERC-20 style tokens representing IP shares
- Enable secondary trading of IP royalty rights

### 2. Automated Royalty Distribution
- Smart contract splits royalties automatically among multiple IP holders
- Support for royalty percentage splits

### 3. Time-based Licensing
- Recurring royalty payments on a schedule
- Subscription-style licensing model

### 4. IP Metadata NFT
- Mint NFTs representing the patent itself
- Attach IP metadata (filing date, jurisdiction, documents hash)

### 5. Dispute Resolution
- On-chain arbitration mechanism
- Slashing for invalid claims

### 6. Frontend Dashboard
- React/web interface for managing patents and licenses
- Analytics for royalty earnings

### 7. Cross-chain Bridges
- Bridge Stellar IP rights to other ecosystems
- Enable multi-chain IP licensing

---

## Technology Stack

- **Blockchain**: Stellar Testnet
- **Smart Contract**: Soroban (Rust)
- **SDK**: soroban-sdk v25
- **Build**: stellar-cli

---

## Profile

- **Name:** mundschau

