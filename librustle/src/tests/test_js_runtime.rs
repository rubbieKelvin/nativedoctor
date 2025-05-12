use crate::executor::scripting::JavascriptRunner;

#[test]
fn test_js_script() {
    let mut runtime = JavascriptRunner::new().unwrap();
    runtime.run("let x = 5;").unwrap();
}

#[test]
#[should_panic(expected = "Script execution failed: Error: Fuck yes!")]
fn test_js_script_throwing_error() {
    let mut runtime = JavascriptRunner::new().unwrap();
    runtime.run("throw new Error('Fuck yes!')").unwrap();
}

#[test]
// #[should_panic(expected = "")]
fn test_js_script_asserted() {
    let mut runtime = JavascriptRunner::new().unwrap();
    runtime.run("console.assert(false, '!Yes')").unwrap();
    assert!(false);
}
