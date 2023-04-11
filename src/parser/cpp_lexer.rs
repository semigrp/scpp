use std::iter::Peekable;
use std::str::Chars;
use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Symbol(char),
    Integer(i64),
    Float(f64),
    StringLiteral(String),
    Whitespace,
    Newline,
}

#[derive(Debug)]
pub struct LexerError {
    details: String,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for LexerError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn read_identifier(&mut self) -> Result<String, LexerError> {
        let mut identifier = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        Ok(identifier)
    }

    fn read_number(&mut self) -> Result<String, LexerError> {
        let mut number = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_digit(10) || c == '.' {
                number.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        Ok(number)
    }

    fn read_string_literal(&mut self) -> Result<String, LexerError> {
        let mut string_literal = String::new();
        self.input.next(); // Skip opening quote

        while let Some(c) = self.input.next() {
            if c == '\\' {
                if let Some(escaped_char) = self.input.next() {
                    string_literal.push(escaped_char);
                } else {
                    return Err(LexerError {
                        details: String::from("Incomplete escape sequence"),
                    });
                }
            } else if c == '"' {
                break;
            } else {
                string_literal.push(c);
            }
        }

        Ok(string_literal)
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        let next_char = match self.input.next() {
            Some(c) => c,
            None => return Ok(None),
        };

        let token = match next_char {
            c if c.is_whitespace() => {
                if c == '\n' {
                    Token::Newline
                } else {
                    Token::Whitespace
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                let identifier = self.read_identifier()?;
                if is_keyword(&identifier) {
                    Token::Keyword(identifier)
                } else {
                    Token::Identifier(identifier)
                }
            }
            c if c.is_digit(10) => {
                let number = self.read_number()?;
                if number.contains('.') {
                    Token::Float(number.parse().map_err(|_| LexerError {
                        details: String::from("Invalid float"),
                    })?)
                } else {
                    Token::Integer(number.parse().map_err(|_| LexerError {
                                                details: String::from("Invalid integer"),
                    })?)
                }
            }
            '"' => {
                let string_literal = self.read_string_literal()?;
                Token::StringLiteral(string_literal)
            }
            c => Token::Symbol(c),
        };

        Ok(Some(token))
    }
}

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "if" | "else" | "for" | "while" | "do" | "int" | "float" | "double" | "char" | "bool"
            | "void" | "true" | "false" | "const" | "static" | "class" | "struct" | "public"
            | "private" | "protected" | "return" | "break" | "continue" | "switch" | "case"
            | "default" | "enum" | "typedef" | "sizeof" | "unsigned" | "signed" | "short"
            | "long" | "namespace" | "using" | "try" | "catch" | "throw" | "new" | "delete"
            | "template" | "explicit" | "virtual" | "friend" | "inline" | "operator"
            | "typeid" | "constexpr" | "decltype" | "alignas" | "alignof" | "char8_t"
            | "char16_t" | "char32_t" | "concept" | "consteval" | "constinit" | "co_await"
            | "co_return" | "co_yield" | "export" | "import" | "module" | "requires"
            | "noexcept" | "static_assert" | "thread_local" | "nullptr" | "auto" | "decltype(auto)"
            | "final" | "override"
    )
}