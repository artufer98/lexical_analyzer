#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_comparisons)]


mod lexical_analyzer;

use std::env;
use crate::lexical_analyzer::analyzer;


fn main() {
    println!("{}", analyzer(env::args().collect()));

    // let test = &"HOLA\"MUN\"DO".find('"');
    //
    // let mut position = match test {
    //     None => -1 as isize,
    //     Some(value) => *value as isize,
    // };
    //
    // println!("{}", position);
    //
    // let test2 = &"HOLA\"MUN\"DO"[4+1..].find('"');
    // position = match test2 {
    //     None => -1 as isize,
    //     Some(value) => *value as isize,
    // };
    //
    // println!("{}", position+4+1);
    //
    // println!("{:?}", "HOLA\"MUN\"DO".as_bytes()[8] as char);

    // println!("{}", &"HOLA\"MUN\"DO"[0..4]);
}
