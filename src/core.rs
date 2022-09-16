use std::io;

use crate::ast::def::*;

#[derive(Debug, PartialEq)]
struct AWKFields {
    fields: Vec<String>,
}

impl AWKFields {
    fn nf(&self) -> usize {
        self.fields.len()
    }
    fn get_field(&self, n: usize) -> Result<String, ()> {
        if n == 0 {
            Ok(self.fields.join(" "))
        } else if (1 <= n) && (n <= self.nf()) {
            Ok(self.fields[n - 1].clone())
        } else {
            Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AWKCore {
    program: AWKProgram,
    // environment
    nr: i64, // number of records
    nf: i64, // number of fields
}

impl AWKCore {
    pub fn new_core(program: AWKProgram) -> AWKCore {
        return AWKCore {
            program,
            nr: 0,
            nf: 0,
        };
    }

    pub fn exec_program(&mut self) {
        self.exec_all_begin_pattern();
        self.read_line_and_exec_program();
        self.exec_all_end_pattern();
    }

    fn read_line_and_exec_program(&mut self) {
        // TODO: IF AWKProgram has BEGIN or END pattern only, Skip main loop
        loop {
            self.nr += 1;
            // Read one line from stdin
            let mut line = String::new();
            if io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line.")
                != 0
            {
                let fields = AWKFields {
                    fields: line
                        .trim()
                        .split_whitespace()
                        .map(|f| f.to_string())
                        .collect(),
                };
                self.nf = fields.nf() as i64;

                for i in &self.program.item_list {
                    match i {
                        AWKItem::AWKPatternAction(pattern_action) => {
                            match pattern_action.pattern {
                                AWKPattern::Always => self.exec_awkaction(&pattern_action.action),
                                _ => (),
                            };
                        }
                    };
                }
            } else {
                break;
            }
        }
    }

    fn exec_all_begin_pattern(&self) {
        // find BEGIN pattern
        for i in &self.program.item_list {
            match i {
                AWKItem::AWKPatternAction(pattern_action) => {
                    match pattern_action.pattern {
                        AWKPattern::Begin => {
                            self.exec_awkaction(&pattern_action.action);
                        }
                        _ => (),
                    };
                }
            };
        }
    }

    fn exec_all_end_pattern(&self) {
        // find BEGIN pattern
        for i in &self.program.item_list {
            match i {
                AWKItem::AWKPatternAction(pattern_action) => {
                    match pattern_action.pattern {
                        AWKPattern::End => {
                            self.exec_awkaction(&pattern_action.action);
                        }
                        _ => (),
                    };
                }
            };
        }
    }
}

// AWKPatternAction
impl AWKCore {
    fn exec_awkaction(&self, actions: &Vec<AWKStatement>) {
        for statement in actions {
            match statement {
                AWKStatement::AWKPrint(awkprint) => self.exec_awkprint(awkprint),
            };
        }
    }

    // print statement
    fn exec_awkprint(&self, awkprint: &AWKPrint) {
        let mut s = false;
        for expr in &awkprint.exprlist {
            print!(
                "{}{}",
                if s { " " } else { "" },
                self.fmt_awkvalue(self.eval_awkexpr(expr))
            );
            s = true;
        }
        println!();
    }
}

// AWKExpr
impl AWKCore {
    fn eval_awkexpr(&self, expr: &AWKExpr) -> AWKValue {
        match expr {
            AWKExpr::AWKValue(value) => value.clone(),
        }
    }
}

// AWKValue
impl AWKCore {
    fn fmt_awkvalue(&self, value: AWKValue) -> String {
        match value {
            AWKValue::AWKNumber(n) => match n {
                AWKNumber::Int(i) => i.to_string(),
                AWKNumber::Float(f) => f.to_string(),
            },
            AWKValue::AWKString(s) => s.value.clone(),
        }
    }
}
