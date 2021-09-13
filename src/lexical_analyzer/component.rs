use std::fs;
use std::process;
use regex::{Regex, Captures};

pub mod token;

use crate::lexical_analyzer::Token;

pub struct Component {
    pub l_llave: token::Token,
    pub r_llave: token::Token,
    pub l_corchete: token::Token,
    pub r_corchete: token::Token,
    pub coma: token::Token,
    pub dos_puntos: token::Token,
    pub literal_num: token::Token,
    pub pr_true: token::Token,
    pub pr_false: token::Token,
    pub pr_null: token::Token,
}


impl Component {
    pub fn get() -> [Token; 10] {
        let component = Component {
            l_llave: token::Token::new(Regex::new(r"\{").unwrap(), "L_LLAVE"),
            r_llave: token::Token::new(Regex::new(r"\}").unwrap(), "R_LLAVE"),
            l_corchete: token::Token::new(
                Regex::new(r"\[").unwrap(), "L_CORCHETE"),
            r_corchete: token::Token::new(
                Regex::new(r"\]").unwrap(), "R_CORCHETE"),
            coma: token::Token::new(Regex::new(r",").unwrap(), "COMA"),
            dos_puntos: token::Token::new(
                Regex::new(r":").unwrap(), "DOS_PUNTOS"),
            literal_num: token::Token::new(
                Regex::new(
                    r"\d+(\.\d+)?((e|E)(\+|-)?\d+)?").unwrap(), "LITERAL_NUM"),
            pr_true: token::Token::new(
                Regex::new(r"true|TRUE").unwrap(), "PR_TRUE"),
            pr_false: token::Token::new(
                Regex::new(r"false|FALSE").unwrap(), "PR_FALSE"),
            pr_null: token::Token::new(
                Regex::new(r"null|NULL").unwrap(), "PR_NULL"),
        };

        [component.l_llave, component.r_llave, component.l_corchete,
        component.r_corchete, component.coma, component.dos_puntos,
        component.literal_num, component.pr_true, component.pr_false,
        component.pr_null]
    }

    pub fn literal_cadena(words: &str) -> String {
        fn get_quote_position(words: &str) -> isize {
            let result = &words.find('"');

            match result {
                None => -1 as isize,
                Some(value) => *value as isize,
            }
        }


        fn is_not_second_quote_escaped(
                word: &str,
                current_position: usize,
                count: usize
        ) -> bool {
            fn is_even(num: usize) -> bool {
                num % 2 == 0
            }

            if current_position < 0 {
                return is_even(count);
            }

            let character = word
                .to_string().as_bytes()[current_position] as char;

            if character != '\\' {
                return is_even(count);
            }

            is_not_second_quote_escaped(&word, current_position-1, count+1)
        }


        fn test_second_quote (
                words: String,
                sub_words: String,
                first_quote_position: usize,
                last_quote_position: usize
        ) -> String {
            fn get_word(words: String, f_pos: usize, l_pos: usize) -> String {
                words.as_str()[f_pos..l_pos].to_string()
            }


            fn replace_words(words: String, word: String) -> String  {
                let r_literal_cadena = token::Token::new(
                    Regex::new(&word).unwrap(), "LITERAL_CADENA");

                let words_replaced = r_literal_cadena.pattern.replace(
                    &words, |caps: &Captures| {
                        r_literal_cadena.lexical_component.to_owned()
                    });

                words_replaced.to_owned().to_string()
            }

            let second_quote_position = get_quote_position(&sub_words);

            if second_quote_position != -1 {
                let word = get_word(
                    words.to_owned(),
                    first_quote_position as usize,
                    ((second_quote_position+1) as usize)+last_quote_position
                );

                if is_not_second_quote_escaped(&word, word.len()-1, 0) {
                    let words_replaced = replace_words(words, word.to_string());

                    return words_replaced;
                }
                else {
                    let sub_words = words.as_str()[
                        (second_quote_position as usize)+1..].to_string();

                    return test_second_quote(
                        words, sub_words, first_quote_position,
                        (second_quote_position as usize)+1);
                }
            }

            words.to_owned()
        }


        fn replace_all_literal_cadena(words: &str) -> String {
            let first_quote_position = get_quote_position(&words);

            if first_quote_position != -1 {
                if ((first_quote_position+1) as usize) < words.len() {
                    let sub_words = &words[(first_quote_position+1) as usize..];

                    let replace_words = test_second_quote(
                        words.to_string(), sub_words.to_string(),
                        first_quote_position as usize,
                        (first_quote_position as usize)+1);

                        return replace_all_literal_cadena(&replace_words);
                    }
                }

            words.to_owned()
        }

        replace_all_literal_cadena(&words)
    }
}
