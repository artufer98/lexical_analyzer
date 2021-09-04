#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate regex;
use std::env;
use regex::Regex;

mod data_file;


fn check_caracters(length: usize) {
	println!("Size: {}", length);
}


fn main() {
	let data_file = data_file::DataFile::new(env::args().collect());

	println!("{}, {}", data_file.content.len(), &data_file.content[472]);

	// let re_lcorchete = Regex::new(r"\[").unwrap();
	// let re_rcorchete = Regex::new(r"\]").unwrap();
	// let re_lllave = Regex::new(r"\{").unwrap();
	// let re_rllave = Regex::new(r"\}").unwrap();
	// let re_coma = Regex::new(r",").unwrap();
	// let re_dos_puntos = Regex::new(r":").unwrap();
	// let re_literal_num = Regex::new(r"\d").unwrap();

	// for caracter in data_file.content.split("") {
	// 	if re_lcorchete.is_match(caracter) {
	// 		println!("{} | L_CORCHETE", caracter);
	// 	}
	// 	else if re_rcorchete.is_match(caracter) {
	// 		println!("{} | R_CORCHETE", caracter);
	// 	}
	// 	else if re_lllave.is_match(caracter) {
	// 		println!("{} | L_LLAVE", caracter);
	// 	}
	// 	else if re_rllave.is_match(caracter) {
	// 		println!("{} | R_LLAVE", caracter);
	// 	}
	// 	else if re_coma.is_match(caracter) {
	// 		println!("{} | COMA", caracter);
	// 	}
	// 	else if re_dos_puntos.is_match(caracter) {
	// 		println!("{} | DOS_PUNTOS", caracter);
	// 	}
	// 	else if re_literal_num.is_match(caracter) {
	// 		println!("{} | NUMBER", caracter);
	// 	}
	// }
}
