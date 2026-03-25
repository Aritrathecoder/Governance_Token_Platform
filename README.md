# Governance_Token_Platform

<img width="1919" height="957" alt="image" src="https://github.com/user-attachments/assets/a999cfb0-ac3b-4c71-b358-04adf1f4f521" />

## Project Descriptionnnsss
Governance Token Platform is a basic Soroban smart contract built on Stellar for managing community proposals, token-based voting, and proposal execution. It provides a simple on-chain governance flow where token holders can participate in decision-making.

## What it does
This contract lets an admin initialize the platform, mint governance tokens, transfer balances, create proposals, vote on proposals, and execute proposals after the voting period ends.

## Features
- Admin initialization
- Governance token minting
- Token transfers
- Proposal creation
- Yes / No voting
- Voting power based on token balance
- Proposal execution after voting ends
- On-chain balance and proposal lookup

## Deployed Smart Contract Link
Governance Token Platform: Transaction b1f011ab7874e701bdd0c38ea63b0dfa6bb3e5150ce6186170bd79cdfdbbd9a1
https://stellar.expert/explorer/testnet/tx/7148771200733184#7148771200733185
<img width="1919" height="959" alt="image" src="https://github.com/user-attachments/assets/5cccf3ab-1c04-4dbd-ba1a-0ce9e78755b7" />





Summary

## Basic Usage
1. Deploy the contract to Stellar using Soroban CLI.
2. Initialize it with the admin address.
3. Mint governance tokens to eligible voters.
4. Create proposals.
5. Cast votes.
6. Execute the proposal after the deadline.

## Notes
This is a starter template and is not production-ready. For a real governance system, add proposal snapshots, quorum rules, timelocks, delegation, and stronger role controls.
