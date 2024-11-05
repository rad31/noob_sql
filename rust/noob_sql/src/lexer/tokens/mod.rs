pub mod char_token;
pub mod identifier_token;
pub mod integer_token;
pub mod keyword_token;
pub mod operator_token;
pub mod punctuator_token;
pub mod string_token;

#[derive(Debug)]
pub enum TokenVariant<'a> {
    Char(char_token::CharToken<'a>),
    Integer(integer_token::IntegerToken<'a>),
    Identifier(identifier_token::IdentifierToken<'a>),
    Keyword(keyword_token::KeywordToken<'a>),
    Operator(operator_token::OperatorToken<'a>),
    Punctuator(punctuator_token::PunctuatorToken<'a>),
    String(string_token::StringToken<'a>),
}

impl TokenVariant<'_> {
    fn to_string(&self) -> &str {
        match *self {
            TokenVariant::Char(_) => CHAR,
            TokenVariant::Integer(_) => INTEGER,
            TokenVariant::Identifier(_) => IDENTIFIER,
            TokenVariant::Keyword(_) => KEYWORD,
            TokenVariant::Operator(_) => OPERATOR,
            TokenVariant::Punctuator(_) => PUNCTUATOR,
            TokenVariant::String(_) => STRING,
        }
    }
}

impl std::fmt::Display for TokenVariant<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub trait Token {
    fn get_lexeme(&self) -> &str;
    fn is_start(c: char) -> bool;
    fn is_end(curr: char, prev: Option<char>) -> bool;
}

pub const CHAR: &str = "Char";
pub const INTEGER: &str = "Integer";
pub const IDENTIFIER: &str = "Identifier";
pub const KEYWORD: &str = "Keyword";
pub const OPERATOR: &str = "Operator";
pub const PUNCTUATOR: &str = "Punctuator";
pub const STRING: &str = "String";
pub const INVALID: &str = "Invalid";
