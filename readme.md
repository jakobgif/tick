[![Rust Backend Unit Tests](https://github.com/jakobgif/tick/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/jakobgif/tick/actions/workflows/rust.yml)
[![Build Backend Docker Image](https://github.com/jakobgif/tick/actions/workflows/docker-build.yml/badge.svg?branch=main)](https://github.com/jakobgif/tick/actions/workflows/docker-build.yml)

# Tick

Tick is a Todo application build in Rust. The application consists of a backend that stores items in a database and a frontend that exposes CRUD features to the user. The frontend communicates with the backend via a REST API.

## Backend
The application uses the following techstack:
- [Axum](https://docs.rs/axum/latest/axum/) framework for the REST API
- [SQLx](https://docs.rs/sqlx_wasi/latest/sqlx/) for database access

- SQLite database

