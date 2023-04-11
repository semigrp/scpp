// tests/test_integration.rs
use safecpp::analyzer::{pointer_analyzer::PointerAnalyzer, memory_analyzer::MemoryAnalyzer};
use safecpp::parser::cpp_lexer::{Lexer};
use safecpp::parser::cpp_parser::{Declaration, Parser};
use safecpp::error::error::MemoryError;

#[test]
fn test_integration_pointer_and_memory_analyzer() {
    let input = r#"
    int *x = new int;
    *x = 42;
    int y = *x;
    delete x;
    "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let declarations = parser.parse().expect("Failed to parse");

    let mut pointer_analyzer = PointerAnalyzer::new();
    let pointer_result = pointer_analyzer.analyze(&declarations);

    assert!(pointer_result.is_ok());

    let mut memory_analyzer = MemoryAnalyzer::new();
    let memory_result = memory_analyzer.analyze(&declarations);

    assert!(memory_result.is_ok());
}

#[test]
fn test_integration_pointer_and_memory_analyzer_with_error() {
    let input = r#"
    int *x = new int;
    int y = *x;
    delete x;
    delete x;
    "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let declarations = parser.parse().expect("Failed to parse");

    let mut pointer_analyzer = PointerAnalyzer::new();
    let pointer_result = pointer_analyzer.analyze(&declarations);

    assert!(pointer_result.is_ok());

    let mut memory_analyzer = MemoryAnalyzer::new();
    let memory_result = memory_analyzer.analyze(&declarations);

    assert!(matches!(memory_result, Err(MemoryError::DoubleFree {..})));
}
