// env.rs --- 
// 
// Filename: env.rs
// Author: Louise <ludwigette>
// Created: Wed Feb 27 22:43:37 2019 (+0100)
// Last-Updated: Thu Feb 28 00:19:26 2019 (+0100)
//           By: Louise <ludwigette>
//
use std::rc::Rc;
use std::cell::Cell;
use pest::iterators::Pair;
use crate::parser::Rule;

#[derive(Default, Debug)]
pub struct Environment {
    variables: [Rc<Cell<f64>>; 26],
}

impl Environment {
    pub fn new() -> Environment {
        Default::default()
    }

    pub fn get_var(&mut self, v: &str) -> Rc<Cell<f64>> {
        let u = (v.chars().next().unwrap() as usize) - b'A' as usize;

        self.variables[u].clone()
    }

    pub fn eval_factor(&mut self, expr: Pair<Rule>) -> f64 {
        let factor = expr.into_inner().next().unwrap();
        
        match factor.as_rule() {
            Rule::number => factor.as_str().parse::<f64>().unwrap(),
            Rule::variable => self.get_var(factor.as_str()).get(),
            Rule::expression => self.eval_expr(factor),
            _ => 0.0
        }
    }
    
    pub fn eval_term(&mut self, expr: Pair<Rule>) -> f64 {
        let mut iter = expr.into_inner();
        let mut value = self.eval_factor(iter.next().unwrap());

        while let Some(op) = iter.next() {
            let new = self.eval_factor(iter.next().unwrap());
            
            match op.as_str() {
                "×" => value *= new,
                "÷" => value /= new,
                _ => unreachable!(),
            }
        }

        value
    }
    
    pub fn eval_arith_expr(&mut self, expr: Pair<Rule>) -> f64 {
        let mut iter = expr.into_inner();
        let mut value = self.eval_term(iter.next().unwrap());

        while let Some(op) = iter.next() {
            let new = self.eval_term(iter.next().unwrap());
            
            match op.as_str() {
                "+" => value += new,
                "-" => value -= new,
                _ => unreachable!(),
            }
        }

        value
    }
    
    pub fn eval_expr(&mut self, expr: Pair<Rule>) -> f64 {
        let mut iter = expr.into_inner();
        let mut value = self.eval_arith_expr(iter.next().unwrap());

        while let Some(op) = iter.next() {
            let new = self.eval_arith_expr(iter.next().unwrap());
            
            match op.as_str() {
                "=" => value = if value == new { 1.0 } else { 0.0 },
                "≠" => value = if value != new { 1.0 } else { 0.0 },
                "<" => value = if value <  new { 1.0 } else { 0.0 },
                ">" => value = if value >  new { 1.0 } else { 0.0 },
                _ => unreachable!(),
            }
        }

        value
    }
    
    pub fn eval_stmt(&mut self, stmt: Pair<Rule>) {
        match stmt.as_rule() {
            Rule::assign_stmt => {
                let mut iter = stmt.into_inner();
                let expr_pair = iter.next().unwrap();
                let var_s = iter.next().unwrap();
                
                let expr_val = self.eval_expr(expr_pair);
                let var = self.get_var(var_s.as_str());

                var.set(expr_val);
                
                println!("{:?}", expr_val);
                println!("{:?}", var);
            },
            Rule::conditional_stmt => {
                let mut iter = stmt.into_inner();
                let condition = self.eval_expr(iter.next().unwrap());
                let cond_stmt = iter.next().unwrap().into_inner().next().unwrap();

                if condition != 0.0 {
                    self.eval_stmt(cond_stmt);
                }
            },
            _ => {
                eprintln!("unknown stmt: {:?}", stmt);
            }
        }
    }
    
    pub fn eval(&mut self, input: Pair<Rule>) {
        for pair in input.into_inner() {
            match pair.as_rule() {
                Rule::stmt => {
                    let stmt = pair.into_inner().next().unwrap();
                    self.eval_stmt(stmt);
                },
                Rule::EOI => (),
                _ => (),
            }
        }
    }
}
