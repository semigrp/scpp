use std::{collections::HashMap, rc::Rc, cell::RefCell};
use crate::parser::cpp_parser::{Declaration, Expression, Statement};

pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
}

pub struct Param {
    pub is_pointer: bool,
}

pub enum PointerErrorKind {
    IncorrectNumberOfArguments,
    NonPointerArgumentForPointerParameter,
    NullDereference,
}

pub struct PointerError {
    pub kind: PointerErrorKind,
    pub details: String,
}

impl std::fmt::Display for PointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::fmt::Debug for PointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.details)
    }
}

impl std::error::Error for PointerError {}

impl PointerError {
    pub fn new(kind: PointerErrorKind, details: String) -> Self {
        PointerError { kind, details }
    }
}

pub struct PointerAnalyzer {
    declarations: Vec<Declaration>,
    pointer_states: Rc<RefCell<HashMap<String, PointerState>>>,
    functions: HashMap<String, Function>,
}

#[derive(Copy, Clone, PartialEq)]
enum PointerState {
    Allocated,
    Deallocated,
}

impl PointerAnalyzer {
    pub fn new(declarations: Vec<Declaration>) -> Self {
        PointerAnalyzer {
            declarations,
            pointer_states: Rc::new(RefCell::new(HashMap::new())),
            functions: HashMap::new(),
        }
    }

    fn analyze_variable_declaration(&mut self, decl: &Declaration) -> Result<(), PointerError> {
        if let Declaration::Variable(name, expr) = decl {
            if self.is_pointer_expression(expr) {
                self.pointer_states.borrow_mut().insert(name.clone(), PointerState::Allocated);
            }
        }
        Ok(())
    }

    fn analyze_function_call(&mut self, expr: &Expression) -> Result<(), PointerError> {
        if let Expression::FunctionCall(name, args) = expr {
            self.check_function_call_arguments(name, args)?;
            for arg in args {
                self.analyze_expression(arg)?;
            }
        }
        Ok(())
    }

    fn analyze_function_call_args(&mut self, args: &Vec<Expression>) -> Result<(), PointerError> {
        for arg in args {
            self.analyze_expression(arg)?;
        }
        Ok(())
    }

    fn is_pointer_expression(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Variable(ref id) => self.pointer_states.borrow().contains_key(id),
            Expression::Dereference(_) => true,
            _ => false,
        }
    }

    fn check_function_call_arguments(
        &self,
        name: &str,
        args: &Vec<Expression>,
    ) -> Result<(), PointerError> {
        if let Some(func) = self.functions.get(name) {
            if func.params.len() != args.len() {
                return Err(PointerError::new(
                    PointerErrorKind::IncorrectNumberOfArguments,
                    format!(
                        "Function '{}' called with incorrect number of arguments",
                        name
                    ),
                ));
            }
            for (arg, param) in args.iter().zip(func.params.iter()) {
                if param.is_pointer && !self.is_pointer_expression(arg) {
                    return Err(PointerError::new(
                        PointerErrorKind::NonPointerArgumentForPointerParameter,
                        format!(
                            "Function '{}' called with non-pointer argument for a pointer parameter",
                            name
                        ),
                    ));
                }
            }
        }
        Ok(())
    }


    fn analyze_expression(&mut self, expr: &Expression) -> Result<(), PointerError> {
        match expr {
            Expression::FunctionCall(name, args) => {
                self.check_function_call_arguments(name, args)?;
                for arg in args {
                    self.analyze_expression(arg)?;
                }
                Ok(())
            }
            Expression::Dereference(expr) => {
                let is_pointer = self.is_pointer_expression(&**expr);
                if let Expression::Variable(id) = &**expr {
                    let pointer_states = self.pointer_states.borrow();
                    if let Some(state) = pointer_states.get(id) {
                        if *state == PointerState::Deallocated && is_pointer {
                            return Err(PointerError::new(
                                PointerErrorKind::NullDereference,
                                format!("Null dereference of variable '{}'", id),
                            ));
                        }
                    }
                }
                self.analyze_expression(&**expr)
            }
            Expression::BinaryOperation(_, left, right) => {
                self.analyze_expression(&**left)?;
                self.analyze_expression(&**right)
            }
            Expression::Assignment(left, right) => {
                let left_id = if let Expression::Variable(id) = &**left {
                    Some(id.clone())
                } else {
                    None
                };
                self.analyze_expression(&**left)?;
                self.analyze_expression(&**right)?;

                if let Some(id) = left_id {
                    let mut pointer_states = self.pointer_states.borrow_mut();
                    if let Some(state) = pointer_states.get_mut(&id) {
                        if self.is_pointer_expression(&**right) {
                            *state = PointerState::Allocated;
                        } else {
                            *state = PointerState::Deallocated;
                        }
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn analyze_statement(&mut self, stmt: &Statement) -> Result<(), PointerError> {
        match stmt {
            Statement::Return(expr) => self.analyze_expression(expr),
            _ => Ok(()),
        }
    }

    fn analyze_declaration(&mut self, declaration: &Declaration) -> Result<(), PointerError> {
        match declaration {
            Declaration::Function(_, _, stmt) => self.analyze_statement(stmt),
            Declaration::Variable(_, expr) => self.analyze_expression(expr),
        }
    }

    pub fn analyze(&mut self) -> Result<(), PointerError> {
        for decl in &self.declarations {
            if let Declaration::Function(name, params, _) = decl {
                self.functions.insert(
                    name.clone(),
                    Function {
                        //params is bool
                        name: name.clone(),
                        params: params.clone(),
                    },
                );
            }
        }

        for decl in &self.declarations {
            self.analyze_variable_declaration(decl)?;
        }

        for decl in &self.declarations {
            self.analyze_declaration(decl)?;
        }

        Ok(())
    }

pub fn analyze_pointer_usage(declarations: Vec<Declaration>) -> Result<(), PointerError> {
    let mut analyzer = PointerAnalyzer::new(declarations);
    analyzer.analyze()
}
}
