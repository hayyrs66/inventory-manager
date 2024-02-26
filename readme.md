# Rust Inventory Management Project

This project is a robust inventory management system developed in Rust. It provides functionalities for adding, updating, and selling products, as well as user management features. The system is designed to efficiently handle inventory tracking and ensure optimal stock levels.

## Features

- **Product Management:** Easily add new products, update product details, and track available quantities.
- **Inventory Control:** Maintain optimal stock levels by setting minimum quantity thresholds for each product.
- **User Management:** Administer user accounts with options to create, update, and delete user profiles.
- **Intuitive Interface:** User-friendly command-line interface for seamless interaction.

## Getting Started

To get started with the Rust Inventory Management Project, follow these steps:

1. Clone the repository to your local machine.
2. Ensure you have Rust installed. If not, follow the [Rust installation instructions](https://www.rust-lang.org/tools/install).
3. Navigate to the project directory and run `cargo run` to compile and execute the program.
4. Follow the on-screen instructions to interact with the inventory management system.

## Dependencies

- [Rust](https://www.rust-lang.org/) - The programming language used to develop the project.
- [web-view](https://crates.io/crates/web-view) - Crate for creating cross-platform web-based GUI applications.
- [rusqlite](https://crates.io/crates/rusqlite) - SQLite3 bindings for Rust, used for database operations.

## User Management

The Rust Inventory Management Project includes user management features, allowing administrators to create, update, and delete user accounts. By default, the SQLite3 database contains a pre-configured user with the following credentials:
- Email: rayrtsx@proton.me
- Password: "admin"

## Database

This project uses SQLite3 as its database system. The database file (`users.db`) is included in the repository and contains tables for storing product and user information.

## Usage

```bash
cargo run

```
## Media
![Screenshot](/img.png)