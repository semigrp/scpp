use crate::parser::cpp_parser::{Declaration, Expression, Statement};
use std::{collections::HashMap, fmt::Display};

enum PointerOperation {
    Allocation(Expression),
    Deallocation(Expression),
    Dereference(Expression),
}

#[derive(Debug, PartialEq)]
pub enum PointerError {
    DoubleFree(String),
    InvalidFree(String),
    NullDereference(String),
}

impl Display for PointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PointerError::DoubleFree(id) => write!(f, "Double free of pointer '{}'", id),
            PointerError::InvalidFree(id) => write!(f, "Invalid free of pointer '{}'", id),
            PointerError::NullDereference(id) => write!(f, "Null dereference of pointer '{}'", id),
        }
    }
}

impl PointerError {
    fn new_double_free(id: &str) -> Self {
        PointerError::DoubleFree(id.to_string())
    }

    fn new_invalid_free(id: &str) -> Self {
        PointerError::InvalidFree(id.to_string())
    }

    fn new_null_dereference(id: &str) -> Self {
        PointerError::NullDereference(id.to_string())
    }
}

pub struct PointerAnalyzer<'a> {
    declarations: &'a Vec<Declaration>,
    pointer_states: HashMap<String, PointerState>,
}

#[derive(Copy, Clone, PartialEq)]
enum PointerState {
    Allocated,
    Deallocated,
}

impl<'a> PointerAnalyzer<'a> {
    pub fn new(declarations: &'a Vec<Declaration>) -> Self {
        PointerAnalyzer {
            declarations,
            pointer_states: HashMap::new(),
        }
    }

    fn analyze_variable_declaration(&mut self, stmt: &Statement) -> Result<(), PointerError> {
        match stmt {
            Statement::Return(expr) => self.analyze_expression(expr),
        }
    }

    fn analyze_function_call(&mut self, expr: &Expression) -> Result<(), PointerError> {
        if let Expression::FunctionCall(name, args) = expr {
            for arg in args {
                self.analyze_expression(arg)?;
            }
            if let Some(func) = self.functions.get(name) {
                // 関数の引数と呼び出し側の引数を比較し、ポインタに関するエラーがあるか確認
                if func.params.len() != args.len() {
                    return Err(PointerError {
                        details: format!(
                            "Function '{}' called with incorrect number of arguments",
                            name
                        ),
                    });
                }
                for (arg, param) in args.iter().zip(func.params.iter()) {
                    if param.is_pointer && !self.is_pointer_expression(arg) {
                        return Err(PointerError {
                            details: format!(
                                "Function '{}' called with non-pointer argument for a pointer parameter",
                                name
                            ),
                        });
                    }
                }
            }
        }
        Ok(())
    }

        fn analyze_operator_overload(&mut self, expr: &Expression) -> Result<(), PointerError> {
        if let Expression::BinaryOperation(lhs, op, rhs) = expr {
            self.analyze_expression(lhs)?;
            self.analyze_expression(rhs)?;

            if op.requires_pointer() {
                if !self.is_pointer_expression(lhs) || !self.is_pointer_expression(rhs) {
                    return Err(PointerError {
                        details: format!("Operator '{}' requires pointer operands", op),
                    });
                }
            }
        }
        Ok(())
    }


    fn analyze_pointer_operation(&mut self, operation: &PointerOperation) -> Result<(), PointerError> {
        match operation {
            PointerOperation::Allocation(expr) => {
                if let Expression::Identifier(ref id) = expr {
                    self.pointer_states.insert(id.clone(), PointerState::Allocated);
                }
                Ok(())
            }
            PointerOperation::Deallocation(expr) => {
                if let Expression::Identifier(ref id) = expr {
                    match self.pointer_states.get(id) {
                        Some(PointerState::Allocated) => {
                            self.pointer_states.insert(id.clone(), PointerState::Deallocated);
                            Ok(())
                        }
                        Some(PointerState::Deallocated) => Err(PointerError::DoubleFree(id.clone())),
                        None => Err(PointerError::InvalidFree(id.clone())),
                    }
                } else {
                    Ok(())
                }
            }
            PointerOperation::Dereference(expr) => {
                if let Expression::Identifier(ref id) = expr {
                    match self.pointer_states.get(id) {
                        Some(PointerState::Allocated) => Ok(()),
                        Some(PointerState::Deallocated) | None => {
                            Err(PointerError::NullDereference(id.clone()))
                        }
                    }
                } else {
                    Ok(())
                }
            }
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) -> Result<(), PointerError> {
        match expr {
            Expression::Identifier(_) => self.analyze_operator_overload(expr),
            Expression::Integer(_) => Ok(()),
        }
    }

    fn analyze_statement(&mut self, stmt: &Statement) -> Result<(), PointerError> {
        match stmt {
            Statement::Return(expr) => self.analyze_expression(expr),
        }
    }

    fn analyze_declaration(&mut self, declaration: &Declaration) -> Result<(), PointerError> {
        match declaration {
            Declaration::Function(_, _, stmt) => self.analyze_statement(stmt),
            Declaration::Variable(_, expr) => self.analyze_expression(expr),
        }
    }

    pub fn analyze(&mut self) -> Result<(), PointerError> {
        for declaration in self.declarations {
            self.analyze_declaration(declaration)?;
        }
        Ok(())
    }
}

pub fn analyze_pointer_usage(declarations: &Vec<Declaration>) -> Result<(), PointerError> {
    let mut analyzer = PointerAnalyzer::new(declarations);
    analyzer.analyze()
}

