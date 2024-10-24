#[test]
fn test_macros() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/message_macro.rs");
    t.pass("tests/pass/reply_macro.rs");
    // TODO: Make it pass
    t.compile_fail("tests/fail/extract/*.rs");
}
