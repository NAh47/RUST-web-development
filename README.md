# RUST-web-development

##Nahom Kiros
This is a repo for CS410P:RUST web development homeworks and assignments. 

This Rust project uses Axum to build a RESTful API for managing a list of questions. It features an in-memory storage system and supports basic CRUD operations. Below is an overview of the key components of the project:

## Files Overview

- **`main.rs`**:
  - Initializes and runs the Axum web server.
  - Sets up routes and manages shared state for question data.

- **`api.rs`**:
  - Handles CRUD operations for questions through HTTP endpoints.

- **`question_list.rs`**:
  - Manages the `Question` data structure and in-memory storage.
  - Provides methods for initializing data and performing CRUD operations.

## Getting Started

To run this project locally, ensure you have Rust and Cargo installed. Clone the repository, navigate to the directory containing `Cargo.toml`, and run the following command:

```bash
cargo run
