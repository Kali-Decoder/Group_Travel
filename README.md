# Automatic Charity Donations Smart Contract

This is a smart contract written in Rust using `#![no_std]` and designed to run on the Soroban blockchain platform. The contract facilitates automatic charity donations through token transfers.

## Overview

The Automatic Charity Donations smart contract allows users to make token transfers while automatically donating a percentage (2%) of the transferred amount to a designated charity account. The contract is managed by an admin who has the authority to set the admin address, add or remove charity accounts, and view donation amounts.

## Features

- **Admin Management**: The admin address can be set by the contract creator. The admin has special privileges to manage charity accounts.
  
- **Charity Account Management**: Only the admin can add or remove charity accounts. Each charity account is associated with a unique identifier (`id`) and an address.
  
- **Token Transfer with Donation**: Users can initiate token transfers, specifying the recipient's address and the charity account ID. The contract automatically calculates and donates 2% of the transferred token amount to the designated charity account.
  
- **Donation Tracking**: The contract keeps track of donation amounts for each user and each charity account.

## Contract Functions

- `set_admin`: Sets the admin address.
  
- `add_charity_account`: Adds a charity account with a specified ID and address.
  
- `remove_charity_account`: Removes a charity account based on its ID.
  
- `transact`: Initiates a token transfer from one address to another, automatically donating 2% of the transferred amount to the specified charity account.
  
- `get_user_donation_amount`: Retrieves the donation amount made by a specific user for a particular token.
  
- `get_charity_donation_amount`: Retrieves the total donation amount received by a charity account for a particular token.

## Usage

To deploy this smart contract on the Soroban blockchain platform, compile the code and deploy the resulting bytecode. Once deployed, interact with the contract using Soroban-compatible tools or libraries. Smart Contract Address: "CCTZNU7HPJIC6HBN4FL5SLHSYJ2SPFOHFXGAGTATS6XRM3PJNNTM5WWG"

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── AutomaticCharityDonations
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

