//! Database layer for **nativedoctor**: SQLite-backed persistence for projects, requests,
//! folders, tests, environments, and execution history.
//!
//! All database files live under `~/.nativedoctor/`. Each project is a separate `.db` file.
//! The `nd_db::store::Store` struct provides the main entry point for all CRUD operations.

pub mod migrate;
pub mod models;
pub mod store;
