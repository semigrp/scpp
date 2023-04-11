// tests/test_parser.rs
use safecpp::parser::cpp_lexer::{Lexer, Token};
use safecpp::parser::cpp_parser::{Declaration, Expression, Parser, Statement};

#[test]
fn test_parse_variable_declaration() {
    let input = "int x = 10;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse().expect("Failed to parse");

    assert_eq!(
        result,
        vec![Declaration::Variable(
            "x".to_string(),
            Expression::Integer(10)
        )]
    );
}

#[test]
fn test_parse_function_declaration() {
    let input = "int sum(int a, int b) { return a + b; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse().expect("Failed to parse");

    assert_eq!(
        result,
        vec![Declaration::Function(
            "sum".to_string(),
            vec!["a".to_string(), "b".to_string()],
            Box::new(Statement::Return(Expression::Identifier("a + b".to_string())))
        )]
    );
}

#[test]
fn test_parse_invalid_input() {
    let input = "int x = 10; invalid";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(result.is_err());
}
