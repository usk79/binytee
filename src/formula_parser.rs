use crate::tree::{*};

#[derive(Debug, Clone, PartialEq)]
struct Loc(usize, usize);

#[derive(Debug, Clone, PartialEq)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Invalid_char(char),
    Invalid_FloatValue,
}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Float(f64),         // 数値はすべて浮動小数扱いする
    Variable(String),   // 変数
    Plus,               // '+'
    Minus,              // '-'
    Mul,                // '*'
    Div,                // '/'
    LParen,             // '('
    RParen,             // ')'
}

impl TokenKind {
    pub fn is_operator(target: &u8) -> bool {
        b"+-*/()".contains(target)
    }

    pub fn valid_char_for_variable(target: &u8) -> bool {
        match target {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => true,
            _ => false,
        }
    }

    pub fn is_whitespace(target: &u8) -> bool {
        match target {
            b' ' | b'\n' | b'\t' => true,
            _ => false,
        }
    }
}

type Token = Annot<TokenKind>;

// FormulaParserは、数式文字列から2分木を構築する + 計算を行う
pub struct FormulaParser { 
    tree: Option<Node<TokenKind>>,
}

impl FormulaParser {
    pub fn new() -> Self {
        Self {
            tree: None,
        }
    }

    pub fn parse(&mut self, formula: &str) -> Result<(), ParseError> {
        let tokens = Self::lexer(formula);

        println!("{:?}", tokens);

        Ok(())
    }

    fn lexer(formula: &str) -> Result<Vec<Token>, ParseError> {
        let input = formula.as_bytes();
        let mut tokens = Vec::new();
        let mut pos = 0;

        macro_rules! push_operator {
            ($operator:expr) => { {
                tokens.push(Token{value: $operator, loc: Loc(pos, pos + 1)});
                pos += 1;
            } }
        }

        macro_rules! push_value {
            ($fn_value:expr) => { {
                match $fn_value(input, &mut pos) {
                    Ok(token) => tokens.push(token),
                    Err(e) => return Err(e),
                }
            } }
        }

        while pos < input.len() {
            match input[pos] {
                b'+' => push_operator!(TokenKind::Plus),
                b'-' => push_operator!(TokenKind::Minus),
                b'*' => push_operator!(TokenKind::Mul),
                b'/' => push_operator!(TokenKind::Div),
                b'(' => push_operator!(TokenKind::LParen),
                b')' => push_operator!(TokenKind::RParen),
                b'0'..=b'9' => push_value!(Self::lex_number),
                b'a'..=b'z' | b'A'..=b'Z' => push_value!(Self::lex_variable),
                b' ' | b'\n' | b'\t' => { pos += 1 },
                _ => return Err(ParseError::Invalid_char(input[pos] as char))
            }
        }
        
        Ok(tokens)
    }

    fn lex_variable(input: &[u8], pos: &mut usize) -> Result<Token, ParseError> {
        use std::str::from_utf8;
        let start = *pos;
        let mut end = *pos + 1;

        while end < input.len() {

            if TokenKind::valid_char_for_variable(&input[end]) {
                end += 1;
            } else if TokenKind::is_operator(&input[end]) {
                break;
            } else if TokenKind::is_whitespace(&input[end]) {
                break;
            } else {
                return Err(ParseError::Invalid_char(input[end] as char))
            }
        }

        *pos = end;
        let variable_name = from_utf8(&input[start..end]).unwrap().to_string();
        Ok( Token{value: TokenKind::Variable(variable_name), loc: Loc(start, end)} )
    }

    fn lex_number(input: &[u8], pos: &mut usize) -> Result<Token, ParseError> {
        use std::str::from_utf8;
        let start = *pos;
        let mut end = *pos + 1;
        let mut decpoint = false;

        while end < input.len() && b"0123456789.".contains(&input[end]) {
            if input[end] == b'.' { // 2回目の小数点が現れたら
                if decpoint == true {
                    return Err(ParseError::Invalid_FloatValue)
                }
                decpoint = true;
            }
            end += 1;
        }
        
        let value = from_utf8(&input[start..end]).unwrap().parse().unwrap();   

        *pos = end;
        Ok( Token{value: TokenKind::Float(value), loc: Loc(start, end)} )
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexnumber() {
        let mut pos = 0;
        
        let token = FormulaParser::lex_number(&"3.1415926535".as_bytes(), &mut pos).unwrap();

        assert_eq!(token, Token{ value: TokenKind::Float(3.1415926535), loc: Loc(0, 12)});

        let mut pos = 0;
        
        let token = FormulaParser::lex_number(&"93.141xx".as_bytes(), &mut pos).unwrap();

        assert_eq!(token, Token{ value: TokenKind::Float(93.141), loc: Loc(0, 6)});
        
        let mut pos = 0;
        let token = FormulaParser::lex_number(&"3.3.2".as_bytes(), &mut pos);

        assert_eq!(token, Err(ParseError::Invalid_FloatValue));
    }

    #[test]
    fn test_lexvariable() {
        let mut pos = 0;
        
        let token = FormulaParser::lex_variable(&"apple".as_bytes(), &mut pos).unwrap();

        assert_eq!(token, Token{ value: TokenKind::Variable("apple".to_string()), loc: Loc(0, 5)});

        let mut pos = 0;
        
        let token = FormulaParser::lex_variable(&"apple2".as_bytes(), &mut pos).unwrap();

        assert_eq!(token, Token{ value: TokenKind::Variable("apple2".to_string()), loc: Loc(0, 6)});
        
        let mut pos = 0;
        let token = FormulaParser::lex_variable(&"aa#aa".as_bytes(), &mut pos);

        assert_eq!(token, Err(ParseError::Invalid_char('#')));
    }
}

