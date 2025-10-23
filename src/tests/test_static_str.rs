#[test]
fn test_bar() {
    let result = crate::lang_features::static_str::bar();
    assert_eq!(result, "bar finished running!");
}
