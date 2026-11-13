use super::quote_identifier;

#[test]
fn quotes_sql_identifiers_without_treating_dots_as_qualification() {
    assert_eq!(quote_identifier("cpu"), "\"cpu\"");
    assert_eq!(quote_identifier("a.b"), "\"a.b\"");
    assert_eq!(quote_identifier("a\"b"), "\"a\"\"b\"");
    assert_eq!(quote_identifier(""), "\"\"");
    assert_eq!(quote_identifier("温度"), "\"温度\"");
}
