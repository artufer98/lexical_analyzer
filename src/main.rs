#![allow(unused_comparisons)]


mod lexical_analyzer;

use std::env;
use crate::lexical_analyzer::analyzer;


fn main() {
    println!("{}", analyzer(env::args().collect()));
}
