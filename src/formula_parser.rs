use std::collections::HashMap;
use crate::tree::{*};

#[derive(Debug, Clone, PartialEq)]
struct Loc(usize, usize);

#[derive(Debug, Clone, PartialEq)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CalcError {
    InvalidChar(char),
    InvalidFloatValue,
    InvalidOperator,
    InvalidFormula,
    NodeError(NodeError),
    IInvalidTree,
    ZeroDiv,
    InvalidBracket,
    InvalidEqualOperator,
    UnexpectVariable,
    EmptyFormula,
    UndefinedVariable,
    NoTree,
}

impl From<NodeError> for CalcError {
    fn from(e: NodeError) -> Self {
        CalcError::NodeError(e)
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
    Mod,                // '%'
    Equal,              // '=' 
    LParen,             // '('
    RParen,             // ')'
}

impl TokenKind {
    pub fn is_operator_char(target: &u8) -> bool { // 演算子なら優先度を返す 0が一番優先度高い
        b"+-*/%()".contains(target)
    }

    pub fn is_value(token: &TokenKind) -> bool {
        match token {
            TokenKind::Float(_) | TokenKind::Variable(_) => true,
            _ => false,
        }
    }

    pub fn is_unaryoperator(token: &TokenKind) -> bool {
        match token {
            TokenKind::Plus | TokenKind::Minus => true,
            _ => false,
        }
    }

    pub fn is_alsoperator(token: &TokenKind) -> bool {
        match token {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Mul | TokenKind::Div => true,
            _ => false,
        }
    }

    pub fn ope_priority(operator: &TokenKind) -> Option<u8> {
        match operator {
            TokenKind::Mul | TokenKind::Div => Some(0),
            TokenKind::Plus | TokenKind::Minus | TokenKind::Mod => Some(1),
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
#[derive(Debug)]
pub struct FormulaCalculator { 
    tree: Option<Node<Token>>,
    vars: HashMap<String, f64>,
}

impl FormulaCalculator {
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        vars.insert("ans".to_string(), 0.0);

        FormulaCalculator {
            tree: None,
            vars: vars,
        }
    }

    pub fn set_formula(formula: &str) -> Result<Self, CalcError> {
        let mut f = Self::new();
        match f.parse(formula) {
            Ok(_) => Ok(f),
            Err(e) => Err(e),
        }
    }

    pub fn calc(&mut self) -> Result<f64, CalcError> {

        if let Some(n) = &mut self.tree {
            match n.as_ref().value {
                TokenKind::Equal => { // rootが = の場合（代入）
                    Self::replace_variable(n.right_mut(), &self.vars)?;

                    let ans = Self::calculate(n.right())?; // = の右側を計算
                    
                    match n.left() {
                        Some(left) => { 
                            let var = &left.as_ref().value;
                            
                            match var { // 左側に来るのは変数のみ
                                TokenKind::Variable(v) => {
                                    self.vars.insert(v.to_string(), ans);
                                    
                                    return Ok( ans )
                                },
                                _ => return Err( CalcError::InvalidFormula),
                            }

                        },
                        None => return Err( CalcError::InvalidFormula),
                    }
                    
                },
                TokenKind::Float(_) | TokenKind::Variable(_) | TokenKind::Plus | TokenKind::Minus => { // 代入式ではない場合は ans 変数に計算結果を入れる
                    Self::replace_variable(self.tree.as_mut(), &self.vars)?;

                    let ans = Self::calculate(self.tree.as_ref())?;
                    self.vars.insert("ans".to_string(), ans);
                    
                    return Ok( ans )
                }
                _ => return Err( CalcError::InvalidFormula),
            }
        }

        return Err( CalcError::NoTree )
    }

    fn replace_variable(node: Option<&mut Node<Token>>, vars: &HashMap<String, f64>) -> Result<(), CalcError> {
        match node {
            Some(n) => {
                n.foreach_mut(&SearchOrder::PreOrder, &mut |elem: &mut Token| {
                    if let TokenKind::Variable(v) = &elem.value {                        
                        if let Some(f) = vars.get(v) {          // ここでは、定義されている変数を見つけた場合には値に置き換えている。　
                            elem.value = TokenKind::Float(*f);  // Noneが来た場合は何もしない　→　calculate関数を実行したときにVariableを見つけたら未定義としてはじく
                        }
                    }
                });
            },
            None => return Err(CalcError::EmptyFormula)
        }
        Ok(())
    }

    fn calculate(node: Option<&Node<Token>>) -> Result<f64, CalcError> { // ツリーから計算を行う
        match node {
            Some(n) => {
                match n.as_ref().value {
                    TokenKind::Plus     => return Ok( Self::calculate(n.left())? + Self::calculate(n.right())? ),
                    TokenKind::Minus    => return Ok( Self::calculate(n.left())? - Self::calculate(n.right())? ),
                    TokenKind::Mul      => return Ok( Self::calculate(n.left())? * Self::calculate(n.right())? ),
                    TokenKind::Div      => {
                        let right = Self::calculate(n.right())?;
                        if right == 0.0 { return Err( CalcError::ZeroDiv ); }
                        return Ok( Self::calculate(n.left())? / right );
                    }, 
                    TokenKind::Mod      => {
                        let right = Self::calculate(n.right())?;
                        if right == 0.0 { return Err( CalcError::ZeroDiv ); }
                        return Ok( Self::calculate(n.left())? % right );
                    }
                    TokenKind::Float(f) => return Ok( f ),
                    TokenKind::Variable(_) => return Err( CalcError::UndefinedVariable ),
                    _ => return Err( CalcError::InvalidFormula )
                }
            }
            None => { return Err( CalcError::IInvalidTree ); } // 演算子の子がNoneはありえない
        }
    }

    pub fn parse(&mut self, formula: &str) -> Result<(), CalcError> {
        let tokens = Self::lexer(formula)?;
 
        let res = Self::parser(&tokens);
        match res {
            Ok(n) => self.tree = Some(n),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn parser(tokens: &[Token]) -> Result<Node<Token>, CalcError> { // https://smdn.jp/programming/tips/polish/を参考に構文解析をする
        // Step. 0: カッコの数をチェック & カッコ外し
        let tokens = Self::check_brackets(tokens)?;

        // Step. 1: tokensの長さが1の時(木の末端)　or tokensの長さが2の時（単項演算子）
        if tokens.len() == 1 {
            if TokenKind::is_value(&tokens[0].value) {
                return Ok( Node::new(tokens[0].clone()) );
            } else {
                return Err( CalcError::InvalidFormula );
            }
        } else if tokens.len() == 2 {
            match tokens[0].value {
                TokenKind::Plus => {
                    match tokens[1].value {
                        TokenKind::Float(_) => return Ok( Node::new(tokens[1].clone()) ),
                        _ => return Err( CalcError::InvalidFormula),
                    }
                },
                TokenKind::Minus => {
                    match tokens[1].value {
                        TokenKind::Float(f) => {
                            let mut n = Node::new(tokens[1].clone());
                            n.as_mut().value = TokenKind::Float(-f);
                            return Ok(n);
                        },
                        _ => return Err( CalcError::InvalidFormula),
                    }
                },
                _ => return Err( CalcError::InvalidFormula),
            }
        }
        
        // Step. 2: 式の中で最も右にありかつ優先度の低い演算子を抽出する。
        let mut priority = 0;
        let mut target_ope = 0; // 一番優先度の低い演算子の番号
        let mut ope_found = false; // 演算子があるかないか
        let mut braket_cnt = 0;

        for (idx, token) in tokens.iter().enumerate() {   
            if idx == 0 { continue; }     // 一番左に単項演算子がある場合を想定　-1 + 2　この-1はStep. 1の単項演算子処理部で処理される
            match token.value {
                TokenKind::LParen => braket_cnt += 1,
                TokenKind::RParen => braket_cnt -= 1,
                _ => {
                    if let Some(p) = TokenKind::ope_priority(&token.value) {
                        if p >= priority && braket_cnt == 0 { // カッコの中にいるとき(bracket_cnt > 0)は無視する
                            priority = p;
                            target_ope = idx;
                        }
                        ope_found = true;
                    }
                }
            }
        }
        if ope_found == false { return Err( CalcError::InvalidFormula ); }

        // Step. 2.1: target_opeの左隣も四則演算の場合
        if TokenKind::is_alsoperator(&tokens[target_ope - 1].value) {
            match tokens[target_ope].value {
                TokenKind::Plus | TokenKind::Minus => target_ope -= 1,
                _ => return Err( CalcError::InvalidOperator ),
            }
        }

        // Step. 3: 一番優先度の低い演算子でノードを作成
        let mut node = Node::new(tokens[target_ope].clone());
        
        // Step. 4: 演算子を中心に左と右に分ける
        node.add_node_left( Self::parser(&tokens[0..target_ope])? )?;
        node.add_node_right( Self::parser(&tokens[target_ope + 1..])? )?;
        
        Ok( node )
    }

    fn lexer(formula: &str) -> Result<Vec<Token>, CalcError> {
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
                b'%' => push_operator!(TokenKind::Mod),
                b'=' => push_operator!(TokenKind::Equal),
                b'(' => push_operator!(TokenKind::LParen),
                b')' => push_operator!(TokenKind::RParen),
                b'0'..=b'9' => push_value!(Self::lex_number),
                b'a'..=b'z' | b'A'..=b'Z' => push_value!(Self::lex_variable),
                b' ' | b'\n' | b'\t' => { pos += 1 },
                _ => return Err(CalcError::InvalidChar(input[pos] as char))
            }
        }
        
        Ok(tokens)
    }

    fn lex_variable(input: &[u8], pos: &mut usize) -> Result<Token, CalcError> {
        use std::str::from_utf8;
        let start = *pos;
        let mut end = *pos + 1;

        while end < input.len() {

            if TokenKind::valid_char_for_variable(&input[end]) {
                end += 1;
            } else if TokenKind::is_operator_char(&input[end]) {
                break;
            } else if TokenKind::is_whitespace(&input[end]) {
                break;
            } else {
                return Err(CalcError::InvalidChar(input[end] as char))
            }
        }

        *pos = end;
        let variable_name = from_utf8(&input[start..end]).unwrap().to_string();
        Ok( Token{value: TokenKind::Variable(variable_name), loc: Loc(start, end)} )
    }

    fn lex_number(input: &[u8], pos: &mut usize) -> Result<Token, CalcError> {
        use std::str::from_utf8;
        let start = *pos;
        let mut end = *pos + 1;
        let mut decpoint = false;

        while end < input.len() && b"0123456789.".contains(&input[end]) {
            if input[end] == b'.' { // 2回目の小数点が現れたら
                if decpoint == true {
                    return Err(CalcError::InvalidFloatValue)
                }
                decpoint = true;
            }
            end += 1;
        }
        
        let value = from_utf8(&input[start..end]).unwrap().parse().unwrap();   

        *pos = end;
        Ok( Token{value: TokenKind::Float(value), loc: Loc(start, end)} )
    }

    fn check_brackets(tokens: &[Token]) -> Result<&[Token], CalcError> {
        let mut checker = 0;

        // カッコの数が間違っていないかチェック
        for t in tokens.iter() {
            match t.value {
                TokenKind::LParen => checker += 1,
                TokenKind::RParen => checker -= 1,
                _ => (),
            }
        }

        if checker != 0 {
            return Err(CalcError::InvalidBracket);
        }

        // 最初が'('で最後が')'だったら一番外側のカッコを外す
        if tokens[0].value == TokenKind::LParen && tokens[tokens.len() - 1].value == TokenKind::RParen {
            Ok(&tokens[1..tokens.len() - 1])
        } else {
            Ok(tokens)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_test() {
        let mut fc = FormulaCalculator::set_formula("1 + 2 * 3 * 8.5").unwrap();
        assert_eq!(fc.calc().unwrap(), 52.0);

        let mut fc = FormulaCalculator::set_formula("-1 * 2").unwrap();
        assert_eq!(fc.calc().unwrap(), -2.0);

        let mut fc = FormulaCalculator::set_formula("-1 * -2").unwrap();
        assert_eq!(fc.calc().unwrap(), 2.0);

        let mut fc = FormulaCalculator::set_formula("1 + -2 * (3 + 2)").unwrap();
        assert_eq!(fc.calc().unwrap(), -9.0);

        let mut fc = FormulaCalculator::set_formula("1  2 * 3 * 8.5");
        assert_eq!(fc.unwrap_err(), CalcError::InvalidFormula);

        let mut fc = FormulaCalculator::set_formula("1 + 2 / 0").unwrap();
        assert_eq!(fc.calc(), Err(CalcError::ZeroDiv));

        let mut fc = FormulaCalculator::set_formula("1 + 2 * (3 + 15 / (1 + 3)) + 1 / 2").unwrap();
        assert_eq!(fc.calc().unwrap(), 15.0);

        let mut fc = FormulaCalculator::set_formula("1 + 2 * (3 + 15 / (1 + 3)) + 1 / 2)");
        assert_eq!(fc.unwrap_err(), CalcError::InvalidBracket);

        let mut fc = FormulaCalculator::set_formula("1 + 2 * * 3 * 8.5");
        assert_eq!(fc.unwrap_err(), CalcError::InvalidOperator);
    }

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

        assert_eq!(token, Err(CalcError::InvalidFloatValue));
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

        assert_eq!(token, Err(CalcError::InvalidChar('#')));
    }
}

