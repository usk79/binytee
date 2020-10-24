use crate::tree::{*};

#[derive(Debug, Clone, PartialEq)]
struct Loc(usize, usize);

#[derive(Debug, Clone, PartialEq)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

#[derive(Debug)]
pub enum ParseError {
    Invalid_char(char),
    Invalid_FloatValue,
    Invalid_Operator,
    Invalid_Formula,
    NodeError(NodeError),
}

impl From<NodeError> for ParseError {
    fn from(e: NodeError) -> Self {
        ParseError::NodeError(e)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Float(f64),         // 数値はすべて浮動小数扱いする
    Variable(String),   // 変数
    Plus,               // '+'
    Minus,              // '-'
    Mul,                // '*'
    Div,                // '/'
    Equal,              // '=' 
    LParen,             // '('
    RParen,             // ')'
}

impl TokenKind {
    pub fn is_operator(target: &u8) -> bool { // 演算子なら優先度を返す 0が一番優先度高い
        b"+-*/()".contains(target)
    }

    pub fn is_value(token: &TokenKind) -> bool {
        match token {
            TokenKind::Float(_) | TokenKind::Variable(_) => true,
            _ => false,
        }
    }

    pub fn ope_priority(operator: &TokenKind) -> Option<u8> {
        match operator {
            TokenKind::Mul | TokenKind::Div => Some(0),
            TokenKind::Plus | TokenKind::Minus => Some(1),
            TokenKind::Equal => Some(2),
            _ => None,
        }
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

// FormulaCalculatorは、数式文字列から2分木を構築する + 計算を行う
pub struct FormulaCalculator { 
    tree: Option<Node<Token>>,
}

impl FormulaCalculator {
    pub fn new() -> Self {
        FormulaCalculator {
            tree: None,
        }
    }

    pub fn parse(&mut self, formula: &str) -> Result<(), ParseError> {
        let tokens = Self::lexer(formula);
        
        match tokens {
            Ok(t) => {
                let res = Self::parser(&t);
                match res {
                    Ok(n) => self.tree = Some(n),
                    Err(e) => return Err(e),
                }
                
            },
            Err(e) => return Err(e),
        }
        
        println!("{:?}", self.tree);
        Ok(())
    }

    fn parser(tokens: &[Token]) -> Result< Node<Token>, ParseError> { // https://smdn.jp/programming/tips/polish/を参考に構文解析をする
        let mut priority = 0;
        let mut target_ope = 0; // 一番優先度の低い演算子の番号
        let mut ope_found = false; // 演算子があるかないか

        // Step. 0: tokensの長さが1の時(木の末端)
        if tokens.len() == 1 {
            if TokenKind::is_value(&tokens[0].value) {
                return Ok( Node::new(tokens[0].clone()) );
            } else {
                return Err( ParseError::Invalid_Formula );
            }
        }
        
        // Step. 1: 式の中で最も右にありかつ優先度の低い演算子を抽出する。
        for (idx, token) in tokens.iter().enumerate() {           
            if let Some(p) = TokenKind::ope_priority(&token.value) {
                if p >= priority {
                    priority = p;
                    target_ope = idx;
                }
                ope_found = true;
            }                     
        }
        if ope_found == false { return Err( ParseError::Invalid_Formula ); }

        // Step. 2: 一番優先度の低い演算子でノードを作成
        let mut node = Node::new(tokens[target_ope].clone());
        
        // Step. 3: 演算子を中心に左と右に分ける
        node.add_node_left( Self::parser(&tokens[0..target_ope])? )?;   // このunwrapはダメ　add_node_leftのエラーがstringなのが間違い
        node.add_node_right( Self::parser(&tokens[target_ope + 1..])? )?;
        
        Ok( node )
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
                b'=' => push_operator!(TokenKind::Equal),
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
        
        let token = FormulaCalculator::lex_number(&"3.1415926535".as_bytes(), &mut pos).unwrap();

        assert_eq!(token, Token{ value: TokenKind::Float(3.1415926535), loc: Loc(0, 12)});

        let mut pos = 0;
        
        let token = FormulaCalculator::lex_number(&"93.141xx".as_bytes(), &mut pos).unwrap();

        assert_eq!(token, Token{ value: TokenKind::Float(93.141), loc: Loc(0, 6)});
        
        let mut pos = 0;
        let token = FormulaCalculator::lex_number(&"3.3.2".as_bytes(), &mut pos);

        assert_eq!(token, Err(ParseError::Invalid_FloatValue));
    }

    #[test]
    fn test_lexvariable() {
        let mut pos = 0;
        
        let token = FormulaCalculator::lex_variable(&"apple".as_bytes(), &mut pos).unwrap();

        assert_eq!(token, Token{ value: TokenKind::Variable("apple".to_string()), loc: Loc(0, 5)});

        let mut pos = 0;
        
        let token = FormulaCalculator::lex_variable(&"apple2".as_bytes(), &mut pos).unwrap();

        assert_eq!(token, Token{ value: TokenKind::Variable("apple2".to_string()), loc: Loc(0, 6)});
        
        let mut pos = 0;
        let token = FormulaCalculator::lex_variable(&"aa#aa".as_bytes(), &mut pos);

        assert_eq!(token, Err(ParseError::Invalid_char('#')));
    }
}

