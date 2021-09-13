use regex::Regex;


pub struct Token {
    pub pattern: Regex,
    pub lexical_component: String
}


impl Token {
    pub fn new(pattern: Regex, lexical_component: &str) -> Token {
        Token {pattern, lexical_component: lexical_component.to_owned()}
    }
}
