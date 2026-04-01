use nd_core::execute::types::ExecutionResult;

fn redact_headers(headers: &[(String, String)]) -> Vec<(String, String)> {
    headers
        .iter()
        .map(|(k, v)| {
            if k.eq_ignore_ascii_case("authorization") {
                (k.clone(), "<redacted>".to_string())
            } else {
                (k.clone(), v.clone())
            }
        })
        .collect()
}

/// Status line always; verbose adds headers; body is pretty-printed JSON when valid UTF-8 JSON.
pub fn print_result(result: &ExecutionResult, verbose: bool) -> Result<(), String> {
    let label = result
        .request_name
        .as_deref()
        .map(|s| format!(" [{}]", s))
        .unwrap_or_default();

    println!(
        "{}{} {} -> {} ({:?})",
        result.method, label, result.final_url, result.status, result.duration
    );

    if verbose {
        let hdrs = redact_headers(&result.headers);

        for (k, v) in hdrs {
            println!("{k}: {v}");
        }

        println!();
    }

    let body = &result.body;
    if body.is_empty() {
        return Ok(());
    }

    if let Ok(text) = std::str::from_utf8(body) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
            println!(
                "{}",
                serde_json::to_string_pretty(&v).unwrap_or_else(|_| text.to_string())
            );
        } else {
            print!("{text}");
            if !text.ends_with('\n') {
                println!();
            }
        }
    } else {
        println!("<{} bytes binary>", body.len());
    }

    return Ok(());
}
