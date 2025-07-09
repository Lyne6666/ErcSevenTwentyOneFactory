# ErcSevenTwentyOneFactory

## Description

This repository houses a novel NFT marketplace contract employing a Merkle tree-based ownership verification system for gas-efficient bulk transfers and claims, coupled with an off-chain IPFS pinning service leveraging decentralized storage for immutable metadata persistence.

## Features

- Deploys ERC-721 smart contracts with customizable metadata URI prefixes.
- Implements a gas-optimized ERC-721 contract creation process using bytecode templates.
- Generates deterministic ERC-721 contract addresses based on factory contract address and nonce.
- Verifies ERC-721 contract deployment success through on-chain event emission and address validation.
- Supports whitelisting addresses that are authorized to deploy ERC-721 contracts.
- Integrates with IPFS for decentralized storage of ERC-721 metadata.
- Provides a configurable fee mechanism for ERC-721 contract deployment, payable in native tokens.
- Offers a user interface for managing and tracking deployed ERC-721 contracts.
## Installation

```bash
pip install git+https://github.com/Lyne6666/ErcSevenTwentyOneFactory.git
```

## Usage

```bash
python -m ercseventwentyonefactory --verbose
```

## Contributing

We welcome contributions! Here's how to get started:

1. Fork this repository
2. Create a new branch for your feature (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add some awesome feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.
