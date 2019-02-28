// main.rs --- 
// 
// Filename: main.rs
// Author: Louise <ludwigette>
// Created: Wed Feb 27 21:16:05 2019 (+0100)
// Last-Updated: Thu Feb 28 00:59:44 2019 (+0100)
//           By: Louise <ludwigette>
//
#[macro_use]
extern crate pest_derive;
use pest::Parser;
use clap::{App, Arg};

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod parser;
use parser::{BasicParser, Rule};

mod env;

fn main() {
    let mut rl = Editor::<()>::new();

    let matches = App::new("Casio Basic interpreter")
        .version("0.1")
        .author("Louise Z.")
        .about("A Casio Basic Interpreter")
        .arg(Arg::with_name("source_file")
             .value_name("FILE")
             .help("A source file to process")
             .takes_value(true)
        )
        .get_matches();

    if let Some(filename) = matches.value_of("source_file") {
        // File parsing
        use std::fs::read_to_string;

        if let Ok(file_content) = read_to_string(filename) {
            let mut env = env::Environment::new();
            let parse = BasicParser::parse(Rule::program, &file_content);
            
            match parse {
                Ok(mut p) => {
                    let rule = p.next().unwrap();
                    
                    env.eval_program(rule);
                },
                Err(e) => println!("{}", e),
            }
        } else {
            eprintln!("Couldn't read file {}", filename);
        }
    } else {
        // REPL
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
                Err(ReadlineError::Interrupted) => {},
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
    }
}
