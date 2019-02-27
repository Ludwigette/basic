// parser.rs --- 
// 
// Filename: parser.rs
// Author: Louise <ludwigette>
// Created: Wed Feb 27 22:03:10 2019 (+0100)
// Last-Updated: Wed Feb 27 22:03:34 2019 (+0100)
//           By: Louise <ludwigette>
// 
use pest::Parser;

#[derive(Parser)]
#[grammar = "basic.pest"]
pub struct BasicParser;
