use crate::parser::cpp_parser::Declaration;
use std::collections::{HashMap, HashSet};

pub enum MemoryErrorType {
    MemoryLeak,
    DoubleFree,
    UninitializedMemoryAccess,
    NullPointerDereference,
}

pub struct MemoryError {
    error_type: MemoryErrorType,
    details: String,
}

pub struct MemoryAnalyzer<'a> {
    declarations: &'a [Declaration],
    allocated_memory: HashMap<String, Expression>,
    freed_memory: HashSet<String>,
    uninitialized_memory: HashSet<String>,
    null_pointer_dereference: HashSet<String>,
}

impl<'a> MemoryAnalyzer<'a> {
    pub fn new(declarations: &'a [Declaration]) -> Self {
        MemoryAnalyzer {
            declarations,
            allocated_memory: HashMap::new(),
            freed_memory: HashSet::new(),
            uninitialized_memory: HashSet::new(),
            null_pointer_dereference: HashSet::new(),
        }
    }

    fn is_memory_allocated(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Identifier(id) => self.allocated_memory.contains_key(id),
            _ => false,
        }
    }

    fn is_memory_freed(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Identifier(id) => self.freed_memory.contains(id),
            _ => false,
        }
    }

    fn is_memory_uninitialized(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Identifier(id) => self.uninitialized_memory.contains(id),
            _ => false,
        }
    }

    fn is_null_pointer_dereference(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Identifier(id) => self.null_pointer_dereference.contains(id),
            _ => false,
        }
    }

    fn handle_memory_allocation(&mut self, id: &str, expr: &Expression) {
        if let Expression::FunctionCall(ref func_name, ref args) = expr {
            if func_name == "malloc" || func_name == "calloc" || func_name == "realloc" {
                self.allocated_memory.insert(id.to_string(), expr.clone());
                self.uninitialized_memory.insert(id.to_string());
            }
        }
    }

    fn handle_memory_free(&mut self, id: &str) {
        if self.allocated_memory.contains_key(id) && !self.freed_memory.contains(id) {
            self.freed_memory.insert(id.to_string());
        } else if self.freed_memory.contains(id) {
            let error = MemoryError {
                error_type: MemoryErrorType::DoubleFree,
                details: format!("Double free attempt on variable: {}", id),
            };
            self.report_error(error);
        }
    }

    fn handle_memory_assignment(&mut self, id: &str, expr: &Expression) {
        if self.is_memory_allocated(expr) {
            self.allocated_memory.insert(id.to_string(), expr.clone());
        } else {
            self.allocated_memory.remove(id);
        }

        if self.is_memory_freed(expr) {
            self.freed_memory.insert(id.to_string());
        } else {
            self.freed_memory.remove(id);
        }

        if self.is_memory_uninitialized(expr) {
            self.uninitialized_memory.insert(id.to_string());
        } else {
            self.uninitialized_memory.remove(id);
        }
    }
    
    fn analyze_expression(&mut self, expr: &Expression) -> Result<(), MemoryError> {
        match expr {
            Expression::Identifier(id) => {
                if self.is_null_pointer_dereference(expr) {
                    let error = MemoryError {
                        error_type: MemoryErrorType::NullPointerDereference,
                        details: format!("Null pointer dereference detected for variable: {}", id),
                    };
                    self.report_error(error);
                }
            }
            Expression::Assignment(id, assign_expr) => {
                self.handle_memory_assignment(id, assign_expr);
                self.analyze_expression(assign_expr)?;
            }
            Expression::FunctionCall(func_name, args) => {
                if func_name == "free" {
                    if let Some(arg) = args.first() {
                        if let Expression::Identifier(id) = arg {
                            self.handle_memory_free(id);
                        }
                    }
                } else {
                    for arg in args {
                        self.analyze_expression(arg)?;
                    }
                }
            }
            Expression::BinaryOperation(_, left, right) => {
                self.analyze_expression(left)?;
                self.analyze_expression(right)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn analyze_statement(&mut self, stmt: &Statement) -> Result<(), MemoryError> {
        match stmt {
            Statement::Declaration(id, expr) => {
                self.handle_memory_allocation(id, expr);
                self.analyze_expression(expr)?;
            }
            Statement::Expression(expr) => {
                self.analyze_expression(expr)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn analyze(&mut self, program: &'a [Statement]) -> Result<(), MemoryError> {
        for stmt in program {
            self.analyze_statement(stmt)?;
        }

        Ok(())
    }
}