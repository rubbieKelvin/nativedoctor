use nd_core::rhai::definition_export::rhai_definitions_single_file;

#[test]
fn rhai_definitions_single_file_includes_globals_and_supplement() {
    let s = rhai_definitions_single_file();
    assert!(
        s.contains("fn env("),
        "expected env() in definitions: {s:?}"
    );
    assert!(
        s.contains("fn invoke("),
        "expected request import invoke() stub"
    );
    assert!(
        s.contains("Look up a runtime variable"),
        "expected doc comment from FuncRegistration"
    );
}
