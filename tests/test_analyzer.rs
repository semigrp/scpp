// tests/test_analyzer.rs
use safecpp::analyzer::pointer_analyzer::PointerAnalyzer;
use safecpp::parser::cpp_lexer::{Lexer};
use safecpp::parser::cpp_parser::{Declaration, Parser};
use safecpp::error::error::MemoryError;

#[test]
fn test_analyze_pointer_declaration() {
    let input = "int *x;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let declarations = parser.parse().expect("Failed to parse");

    let mut pointer_analyzer = PointerAnalyzer::new();
    let result = pointer_analyzer.analyze(&declarations);

    assert!(result.is_ok());
}

#[test]
fn test_analyze_null_pointer_dereference() {
    let input = r#"
    int *x = nullptr;
    int y = *x;
    "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let declarations = parser.parse().expect("Failed to parse");

    let mut pointer_analyzer = PointerAnalyzer::new();
    let result = pointer_analyzer.analyze(&declarations);

    assert!(matches!(result, Err(MemoryError::NullPointerDereference {..})));
}

#[test]
fn test_analyze_double_free() {
    let input = r#"
    int *x = new int;
    delete x;
    delete x;
    "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let declarations = parser.parse().expect("Failed to parse");

    let mut pointer_analyzer = PointerAnalyzer::new();
    let result = pointer_analyzer.analyze(&declarations);

    assert!(matches!(result, Err(MemoryError::DoubleFree {..})));
}
