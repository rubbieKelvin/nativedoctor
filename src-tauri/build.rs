//! Build script: runs Tauri build, then embeds SQL migration files into the crate.
//!
//! We read all `*.sql` files from `migrations/` (sorted by filename), escape their contents,
//! and write a generated Rust file that defines `MIGRATIONS: &[(&str, &str)]`. Each pair is
//! (filename, sql_content). The app then includes this file and runs unapplied migrations in
//! `init_db`. This way migrations are compiled into the binary and no runtime path is needed.

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    tauri_build::build();

    // Paths: crate root (src-tauri) and the migrations folder next to Cargo.toml.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations_dir = Path::new(&manifest_dir).join("migrations");
    // OUT_DIR is where Cargo lets us write build outputs; the crate can include files from here.
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("migrations.rs");

    // Collect all .sql files in migrations/, then sort by filename so order is deterministic
    // (e.g. 001_initial.sql before 002_add_foo.sql).
    let mut entries: Vec<_> = fs::read_dir(&migrations_dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "sql"))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut out = fs::File::create(&out_path).expect("create migrations.rs");

    // Emit a Rust constant: array of (name, sql) string pairs. The SQL is embedded as a normal
    // string literal so the binary is self-contained.
    writeln!(out, "pub const MIGRATIONS: &[(&str, &str)] = &[").unwrap();
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        let content =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
        // Escape content so it can be used inside a Rust double-quoted string (backslashes and
        // quotes escaped; newlines turned into \\n so they become real newlines in the generated code).
        let content_escaped = content
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace("\r\n", "\\n")
            .replace('\n', "\\n")
            .replace('\r', "\\n");
        writeln!(out, "    (\"{}\", \"{}\"),", name, content_escaped).unwrap();
    }
    writeln!(out, "];").unwrap();
}
