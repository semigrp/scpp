use crate::parser::cpp_lexer::{Lexer, Token};
use std::borrow::Borrow;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    FunctionCall(String, Vec<Expression>),
    Variable(String),
    Dereference(Box<Expression>),
    BinaryOperation(String, Box<Expression>, Box<Expression>),
    Assignment(Box<Expression>, Box<Expression>),
    ArrayAccess(String, Box<Expression>),
    ArrayDeclaration(String, Box<Expression>),
}

impl Borrow<String> for Expression {
    fn borrow(&self) -> &String {
        match self {
            Expression::Identifier(s) => s,
            Expression::FunctionCall(s, _) => s,
            Expression::Variable(s) => s,
            _ => panic!("Not a string expression"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperator {
    pub fn requires_pointer(&self) -> bool {
        match self {
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Declaration(String, Expression),
    If(Expression, Box<Statement>, Box<Statement>),
    While(Expression, Box<Statement>),
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Declaration {
    Function(String, Vec<String>, Box<Statement>),
    Variable(String, Expression),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

#[derive(Debug)]
pub struct ParserError {
    details: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}


impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser { lexer }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        if let Some(token) = self.lexer.next_token()? {
            match token {
                Token::Identifier(identifier) => Ok(Expression::Identifier(identifier)),
                Token::Integer(value) => Ok(Expression::Integer(value)),
                _ => Err(ParserError {
                    details: String::from("Unexpected token in expression"),
                }),
            }
        } else {
            Err(ParserError {
                details: String::from("Unexpected end of input"),
            })
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        if let Some(token) = self.lexer.next_token()? {
            match token {
                Token::Keyword(keyword) if keyword == "return" => {
                    let expr = self.parse_expression()?;
                    Ok(Statement::Return(expr))
                }
                _ => Err(ParserError {
                    details: String::from("Unexpected token in statement"),
                }),
            }
        } else {
            Err(ParserError {
                details: String::from("Unexpected end of input"),
            })
        }
    }

    fn parse_declaration(&mut self) -> Result<Declaration, ParserError> {
        if let Some(token) = self.lexer.next_token()? {
            match token {
                Token::Keyword(keyword) => match keyword.as_str() {
                    "int" => {
                        if let Some(Token::Identifier(identifier)) = self.lexer.next_token()? {
                            if let Some(Token::Symbol('(')) = self.lexer.next_token()? {
                                let mut params = Vec::new();
                                while let Some(Token::Identifier(param)) = self.lexer.next_token()? {
                                    params.push(param);
                                    if let Some(Token::Symbol(',')) = self.lexer.next_token()? {
                                        continue;
                                    } else {
                                        break;
                                    }
                                }
                                if let Some(Token::Symbol(')')) = self.lexer.next_token()? {
                                    let stmt = self.parse_statement()?;
                                    Ok(Declaration::Function(identifier, params, Box::new(stmt)))
                                } else {
                                    Err(ParserError {
                                        details: String::from("Expected ')'"),
                                    })
                                }
                            } else {
                                let value = self.parse_expression()?;
                                Ok(Declaration::Variable(identifier, value))
                            }
                        } else {
                            Err(ParserError {
                                details: String::from("Expected identifier"),
                            })
                        }
                    }
                    
                    _ => Err(ParserError {
                        details: String::from("Unexpected keyword in declaration"),
                    }),
                },
                _ => Err(ParserError {
                    details: String::from("Unexpected token in declaration"),
                }),
            }
        } else {
            Err(ParserError {
                details: String::from("Unexpected end of input"),
            })
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Declaration>, ParserError> {
        let mut declarations = Vec::new();

        while let Ok(declaration) = self.parse_declaration() {
            declarations.push(declaration);
        }

        if let Some(token) = self.lexer.next_token()? {
            Err(ParserError {
                details: format!("Unexpected token {:?}", token),
            })
        } else {
            Ok(declarations)
        }
    }
}

pub fn parse_cpp_code(source_code: &str) -> Result<Vec<Declaration>, ParserError> {
    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);
    parser.parse()
}

