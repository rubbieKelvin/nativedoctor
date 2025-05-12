use crate::executor::scripting::JavascriptExec;

#[test]
fn test_js_script() {
    let mut runtime = JavascriptExec::new().unwrap();
    runtime.run("let x = 5;", None).unwrap();
}

#[test]
#[should_panic(expected = "Script execution failed: Error: Fuck yes!")]
fn test_js_script_throwing_error() {
    let mut runtime = JavascriptExec::new().unwrap();
    runtime.run("throw new Error('Fuck yes!')", None).unwrap();
}

#[test]
#[should_panic(expected = "Script execution failed: AssertationError: Assertation failed")]
fn test_js_script_asserted() {
    let mut runtime = JavascriptExec::new().unwrap();
    runtime.run("assert(false)", None).unwrap();
}

#[test]
fn test_js_script_log() {
    let mut runtime = JavascriptExec::new().unwrap();
    runtime.run("log('Hello rubbie')", None).unwrap();
    assert!(false);
}
