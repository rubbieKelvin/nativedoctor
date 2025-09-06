use crate::utils::slugify;

#[test]
fn test_simple_case() {
    assert_eq!(slugify("Hello, World!"), "hello-world");
}

#[test]
fn test_with_numbers() {
    assert_eq!(slugify("Version 1.2.3"), "version-1-2-3");
}

#[test]
fn test_with_multiple_spaces() {
    assert_eq!(slugify("  multiple   spaces  "), "multiple-spaces");
}

#[test]
fn test_with_symbols() {
    assert_eq!(
        slugify("Rust is the #1 programming language!"),
        "rust-is-the-1-programming-language"
    );
}

#[test]
fn test_leading_and_trailing_symbols() {
    assert_eq!(slugify("!leading-and-trailing!"), "leading-and-trailing");
}

#[test]
fn test_hyphens() {
    assert_eq!(slugify("a--b---c"), "a-b-c");
}

#[test]
fn test_mixed_case() {
    assert_eq!(slugify("MixedCaseString"), "mixedcasestring");
}

#[test]
fn test_empty_string() {
    assert_eq!(slugify(""), "");
}

#[test]
fn test_only_symbols() {
    assert_eq!(slugify("!@#$%^&*()"), "");
}
