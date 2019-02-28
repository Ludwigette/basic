// parser.rs --- 
// 
// Filename: parser.rs
// Author: Louise <ludwigette>
// Created: Wed Feb 27 22:03:10 2019 (+0100)
// Last-Updated: Thu Feb 28 00:59:54 2019 (+0100)
//           By: Louise <ludwigette>
// 
#[derive(Parser)]
#[grammar = "basic.pest"]
pub struct BasicParser;
