use crate::parser::cpp_parser::{Expression, Statement};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub struct ArrayAnalyzer<'a> {
    program: &'a [Statement],
    array_sizes: HashMap<String, usize>,
}

#[derive(Debug, PartialEq)]
pub struct ArrayError {
    message: String,
}

impl fmt::Display for ArrayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}


impl Error for ArrayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl<'a> ArrayAnalyzer<'a> {
    pub fn new(program: &'a [Statement]) -> Self {
        ArrayAnalyzer {
            program,
            array_sizes: HashMap::new(),
        }
    }

    fn is_out_of_bounds_access(&self, expr: &Expression) -> bool {
        if let Expression::ArrayAccess(id, index_expr) = expr {
            if let Some(size) = self.array_sizes.get(id) {
                if let Expression::Integer(index) = **index_expr {
                    return index < 0 || index as usize >= *size;
                }
            }
        }
        false
    }
    

    fn handle_array_declaration(&mut self, id: &str, size: usize) {
        self.array_sizes.insert(id.to_string(), size);
    }

    fn handle_array_access(&mut self, id: &str, index: &Expression) -> Result<(), ArrayError> {
        if self.is_out_of_bounds_access(&Expression::ArrayAccess(id.to_string(), Box::new(index.clone()))) {
            Err(ArrayError {
                message: format!("Array access out of bounds for '{}'", id),
            })
        } else {
            Ok(())
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) -> Result<(), ArrayError> {
        match expr {
            Expression::ArrayAccess(id, index) => self.handle_array_access(id, index),
            Expression::ArrayDeclaration(id, index) => {
                if let Expression::Integer(size) = **index {
                    self.handle_array_declaration(id, size as usize);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn analyze_statement(&mut self, stmt: &Statement) -> Result<(), ArrayError> {
        match stmt {
            Statement::Expression(expr) => self.analyze_expression(expr),
            _ => Ok(()),
        }
    }

    pub fn analyze(&mut self) -> Result<(), ArrayError> {
        for stmt in self.program {
            self.analyze_statement(stmt)?;
        }
        Ok(())
    }
}
