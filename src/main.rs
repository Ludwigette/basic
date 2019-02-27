// main.rs --- 
// 
// Filename: main.rs
// Author: Louise <ludwigette>
// Created: Wed Feb 27 21:16:05 2019 (+0100)
// Last-Updated: Thu Feb 28 00:31:30 2019 (+0100)
//           By: Louise <ludwigette>
//
#[macro_use]
extern crate pest_derive;
use pest::Parser;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod parser;
use parser::{BasicParser, Rule};

mod env;

fn main() {
    let mut rl = Editor::<()>::new();
    let mut env = env::Environment::new();
    
    loop {
        let s = rl.readline(">> ");
        match s {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());

                let parse = BasicParser::parse(Rule::program, &line);

                match parse {
                    Ok(mut p) => {
                        let rule = p.next().unwrap();
                        
                        env.eval_program(rule);
                    },
                    Err(e) => println!("{}", e),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
