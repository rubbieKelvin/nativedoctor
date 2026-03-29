//! `nativedoctor generate`: OpenAPI 3.0.x → request files.

use std::path::Path;

use ng_generate::{generate_from_openapi_path, GenerateOptions, OutputFormat};

pub fn run_generate(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    format: OutputFormat,
) -> Result<(), String> {
    let report =
        generate_from_openapi_path(input.as_ref(), output.as_ref(), GenerateOptions { format })
            .map_err(|e| e.to_string())?;
    let n = report.files_written.len();
    println!(
        "Generated {n} request file(s) under {}",
        output.as_ref().display()
    );
    for p in &report.files_written {
        println!("  {}", p.display());
    }
    Ok(())
}
