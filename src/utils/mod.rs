use std::{env, path::PathBuf};

use anyhow::Context;

/// Converts a string into a URL-friendly "slug".
pub fn slugify<S: AsRef<str>>(input: S) -> String {
    let mut slug = String::new();

    // prevents leading hyphen
    let mut last_was_hyphen = true;

    for c in input.as_ref().to_lowercase().chars() {
        if c.is_alphanumeric() {
            slug.push(c);
            last_was_hyphen = false;
        } else if !last_was_hyphen {
            slug.push('-');
            last_was_hyphen = true;
        }
    }

    // remove  trailing hyphen that may have been added at the end
    while slug.ends_with('-') {
        slug.pop();
    }

    return slug;
}

/// Gets the current directory
pub fn get_current_directory() -> Result<PathBuf, anyhow::Error> {
    env::current_dir().context("Could not access the current directory")
}
