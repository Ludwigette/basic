// env.rs --- 
// 
// Filename: env.rs
// Author: Louise <ludwigette>
// Created: Wed Feb 27 22:43:37 2019 (+0100)
// Last-Updated: Sat Mar  2 15:07:18 2019 (+0100)
//           By: Louise <ludwigette>
//
use std::io::Write;
use std::rc::Rc;
use std::cell::Cell;
use pest::Parser;
use pest::iterators::Pair;
use crate::parser::{BasicParser, Rule};
use crate::error::BasicError;

fn get_string_from_lit(lit: Pair<Rule>) -> &str {
    let s = lit.as_str();
    s.get(1..s.len() - 1).unwrap()
}

#[derive(Default, Debug)]
pub struct Environment {
    variables: [Rc<Cell<f64>>; 26],
}

impl Environment {
    pub fn new() -> Environment {
        Default::default()
    }

    pub fn get_var(&self, v: &str) -> Rc<Cell<f64>> {
        let u = (v.chars().next().unwrap() as usize) - b'A' as usize;

        self.variables[u].clone()
    }

    pub fn eval_factor(&self, expr: Pair<Rule>) -> Result<f64, BasicError> {
        let factor = expr.into_inner().next().unwrap();
        
        match factor.as_rule() {
            Rule::number => factor.as_str().parse::<f64>().map_err(|_e| BasicError::SyntaxError),
            Rule::variable => Ok(self.get_var(factor.as_str()).get()),
            Rule::expression => self.eval_expr(factor),
            Rule::question => {
                if let Some(prompt) = factor.into_inner().next() {
                    print!("{}? ", get_string_from_lit(prompt));
                } else {
                    print!("? ");
                }
                let _ = std::io::stdout().flush();
                let stdin = std::io::stdin();
                let mut s = String::new();

                stdin.read_line(&mut s).map_err(|e| BasicError::IOError(e))?;
                
                if let Ok(mut parse) = BasicParser::parse(Rule::expression, &s) {
                    self.eval_expr(parse.next().unwrap())
                } else {
                    Err(BasicError::SyntaxError)
                }
            }
            _ => unreachable!(),
        }
    }
    
    pub fn eval_term(&self, expr: Pair<Rule>) -> Result<f64, BasicError> {
        let mut iter = expr.into_inner();
        let mut value = self.eval_factor(iter.next().unwrap())?;

        while let Some(op) = iter.next() {
            let new = self.eval_factor(iter.next().unwrap())?;
            
            match op.as_str() {
                "×" => value *= new,
                "÷" => value /= new,
                _ => unreachable!(),
            }
        }

        Ok(value)
    }
    
    pub fn eval_arith_expr(&self, expr: Pair<Rule>) -> Result<f64, BasicError> {
        let mut iter = expr.into_inner();
        let mut value = self.eval_term(iter.next().unwrap())?;

        while let Some(op) = iter.next() {
            let new = self.eval_term(iter.next().unwrap())?;
            
            match op.as_str() {
                "+" => value += new,
                "-" => value -= new,
                _ => unreachable!(),
            }
        }

        Ok(value)
    }
    
    pub fn eval_expr(&self, expr: Pair<Rule>) -> Result<f64, BasicError> {
        let mut iter = expr.into_inner();
        let mut value = self.eval_arith_expr(iter.next().unwrap())?;

        while let Some(op) = iter.next() {
            let new = self.eval_arith_expr(iter.next().unwrap())?;
            
            match op.as_str() {
                "=" => value = if value == new { 1.0 } else { 0.0 },
                "≠" => value = if value != new { 1.0 } else { 0.0 },
                "<" => value = if value <  new { 1.0 } else { 0.0 },
                ">" => value = if value >  new { 1.0 } else { 0.0 },
                "≤" => value = if value <= new { 1.0 } else { 0.0 },
                "≥" => value = if value >= new { 1.0 } else { 0.0 },
                _ => unreachable!(),
            }
        }

        Ok(value)
    }
    
    pub fn eval_stmt(&mut self, stmt: Pair<Rule>) -> Result<(), BasicError> {
        match stmt.as_rule() {
            Rule::assign_stmt => {
                let mut iter = stmt.into_inner();
                let expr_pair = iter.next().unwrap();
                let var_s = iter.next().unwrap();
                
                let expr_val = self.eval_expr(expr_pair)?;
                let var = self.get_var(var_s.as_str());

                var.set(expr_val);

                Ok(())
            },
            Rule::conditional_stmt => {
                let mut iter = stmt.into_inner();
                let condition = self.eval_expr(iter.next().unwrap())?;
                let cond_stmt = iter.next().unwrap().into_inner().next().unwrap();

                if condition != 0.0 {
                    self.eval_stmt(cond_stmt)
                } else {
                    Ok(())
                }
            },
            Rule::conditional_block => {
                let mut iter = stmt.into_inner();
                let condition = self.eval_expr(iter.next().unwrap())?;
                let cond_stmts = iter.next().unwrap();
                let else_stmts_op = iter.next();
                
                if condition != 0.0 {
                    for cond_stmt in cond_stmts.into_inner() {
                        self.eval_stmt(cond_stmt.into_inner().next().unwrap())?;
                    }
                } else if let Some(else_stmts) = else_stmts_op {
                    for else_stmt in else_stmts.into_inner() {
                        self.eval_stmt(else_stmt.into_inner().next().unwrap())?;
                    }
                }

                Ok(())
            },
            Rule::while_block => {
                let mut iter = stmt.into_inner();
                let condition = iter.next().unwrap();
                let loop_stmts = iter.next().unwrap();
                
                while self.eval_expr(condition.clone())? != 0.0 {
                    for loop_stmt in loop_stmts.clone().into_inner() {
                        self.eval_stmt(loop_stmt.into_inner().next().unwrap())?;
                    }
                }

                Ok(())
            },
            Rule::display_stmt => {
                let mut iter = stmt.into_inner();
                let thingy = iter.next().unwrap();

                match thingy.as_rule() {
                    Rule::expression => {
                        println!("{}", self.eval_expr(thingy)?);

                        Ok(())
                    },
                    Rule::string_literal => {
                        println!("{}", get_string_from_lit(thingy));

                        Ok(())
                    },
                    _ => {
                        eprintln!("unknown thing to display: {:?}", thingy);
                        Err(BasicError::ParsingError)
                    }
                }
            }
            _ => {
                eprintln!("unknown stmt: {:?}", stmt);
                Err(BasicError::ParsingError)
            }
        }
    }

    pub fn eval_stmts(&mut self, stmts: Pair<Rule>) {
        for pair in stmts.into_inner() {
            match pair.as_rule() {
                Rule::stmt => {
                    let stmt = pair.into_inner().next().unwrap();
                    
                    if let Err(e) = self.eval_stmt(stmt) {
                        eprintln!("{:?}", e);
                    }
                },
                Rule::EOI => (),
                _ => (),
            }
        }
    }
    
    pub fn eval_program(&mut self, input: Pair<Rule>) {
        let stmts = input.into_inner().next().unwrap();
        self.eval_stmts(stmts);
    }
}
