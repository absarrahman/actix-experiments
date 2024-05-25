# Simple kickstart Rust backend app using Actix

Starter project for backend app written in Rust. It is designed to reduce the time spent on setting up 
the initial project structure, which allows us to focus on implementing core features and functionality.

## Features:
**1. RestAPI:** Used Actix for implementing endpoints and handling HTTP requests
**2. DB integration:** Used Vercel's Postgres to store and retrieve data

## Usage:
1. Clone the repo `git clone https://github.com/absarrahman/actix-experiments.git`
2. Open the directory `cd actix-experiments`
3. Create `.env` file and configure it
4. For running the project use `cargo run`
5. If you want to auto reload like [Nodemon](https://www.npmjs.com/package/nodemon) use `cargo watch -x run`

## Articles
1. [Actix documentation](https://actix.rs/docs)
2. [Build a Simple API with Rust and Actix Web](https://codevoweb.com/build-a-simple-api-with-rust-and-actix-web)
3. [Vercel Postgres](https://vercel.com/storage/postgres)
4. [Rust book](https://doc.rust-lang.org/book)

## Future goals:
1. Implement authentication
2. Organize project structure
