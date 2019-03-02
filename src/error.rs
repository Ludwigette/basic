// error.rs --- 
// 
// Filename: error.rs
// Author: Louise <ludwigette>
// Created: Thu Feb 28 10:31:42 2019 (+0100)
// Last-Updated: Sat Mar  2 14:39:37 2019 (+0100)
//           By: Louise <ludwigette>
//
use std::io::Error;

#[derive(Debug)]
pub enum BasicError {
    IOError(Error),
    ParsingError,
    SyntaxError,
}
