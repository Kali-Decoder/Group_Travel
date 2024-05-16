# Decentralized Group Travel Planning

This repository contains the code for a decentralized group travel planning platform, implemented as a smart contract in Rust using `#![no_std]`. The contract is designed to run on the Soroban blockchain platform and facilitates transparent management of group travel expenses, itineraries, and contributions.

## Overview

The Decentralized Group Travel Planning smart contract allows users to create and manage travel groups, track expenses, contribute funds, and share itineraries. The contract ensures transparency and automation in managing group travel activities.

## Features

- **Group Creation**: Users can create travel groups with an initial itinerary and become the organizer.
  
- **Joining Groups**: Participants can join existing travel groups to collaborate on travel plans.
  
- **Expense Management**: Organizers and participants can add and track expenses for the group.
  
- **Contributions**: Participants can contribute funds towards group expenses.
  
- **Itinerary Sharing**: The group itinerary is stored and can be retrieved by participants.

## Contract Functions

- `create_group`: Creates a new travel group with the specified title and initial itinerary.
  
- `join_group`: Allows a user to join an existing travel group.
  
- `add_expense`: Adds an expense to the specified travel group.
  
- `contribute`: Allows a user to contribute funds to the specified travel group.
  
- `get_itinerary`: Retrieves the itinerary for the specified travel group.

## Usage

To deploy this smart contract on the Soroban blockchain platform, compile the code and deploy the resulting bytecode. Once deployed, interact with the contract using Soroban-compatible tools or libraries.

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── DecentralizedGroupTravel
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
