pub mod file;
pub mod component;

use regex::{Captures};
use crate::lexical_analyzer::file::File;
use crate::lexical_analyzer::component::token::Token;


fn get_file(args: Vec<String>) -> File {
    File::new(args)
}


fn convert(words: &str, components: &[Token; 10],
        current_position: usize) -> String {
    if current_position >= components.len()-1 {
        return component::Component::literal_cadena(&words);
    }

    let current_component = &components[current_position];
    let words_replaced = current_component.pattern.replace_all(
        &words, |_caps: &Captures| {
            current_component.lexical_component.to_owned()
        });

    convert(&words_replaced, components, current_position+1)
}


pub fn analyzer(args: Vec<String>) -> String{
    convert(&get_file(args).content, &component::Component::get(), 0)
}
