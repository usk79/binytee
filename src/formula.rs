
use std::collections::HashMap;
use crate::tree::{*};
use crate::varpool::{*};
use colored::*;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Loc(usize, usize);

#[derive(Debug, Clone, PartialEq)]
struct Annot<T> {
    value: T,
    loc: Loc,
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
    pub fn to_string(&self) -> String {
        match self {
            TokenKind::Plus        => "+".to_string(),
            TokenKind::Minus       => "-".to_string(),
            TokenKind::Mul         => "*".to_string(),
            TokenKind::Div         => "/".to_string(),
            TokenKind::Mod         => "%".to_string(),
            TokenKind::Equal       => "=".to_string(),
            TokenKind::LParen      => "(".to_string(),
            TokenKind::RParen      => ")".to_string(),
            TokenKind::Float(f)    => f.to_string(),
            TokenKind::Variable(v) => v.clone(),
        }
    }

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
    formula_str: String,
}

impl FormulaCalculator {
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        vars.insert("ans".to_string(), 0.0);

        FormulaCalculator {
            tree: None,
            formula_str: String::new(),
        }
    }

    pub fn set_formula(formula: &str) -> Result<Self, FormulaErr> {
        let mut f = Self::new();
        f.parse(formula)?;
        Ok(f)
    }

    pub fn calc(&mut self, vers: &VarPool) -> Result<VarData, FormulaErr> {
        match self.calc_root(vers) {
            Ok(f) => Ok(f),
            Err(mut e) => {
                e.formula = self.formula_str.clone();
                return Err(e);
            }
        }
    }

    fn calc_root(&mut self, vars: &VarPool) -> Result<VarData, FormulaErr> {

        if let Some(n) = &mut self.tree {
            match n.as_ref().value {
                TokenKind::Equal => { // rootが = の場合（代入）
                    Self::replace_variable(n.right_mut(), vars)?;

                    let ans = Self::calculate(n.right())?; // = の右側を計算
                    
                    match n.left() {
                        Some(left) => { 
                            let var = &left.as_ref().value;
                            
                            match var { // 左側に来るのは変数のみ
                                TokenKind::Variable(v) => {
                                                       
                                    return Ok( VarData(v.to_string(), ans) )
                                },
                                _ => return Err( FormulaErr::new(ErrType::InvalidFormula, "= is found, but left term is not variable", n.as_ref().loc ) ),
                            }

                        },
                        None => return Err( FormulaErr::new(ErrType::InvalidFormula, "= is found, but left term is None.", n.as_ref().loc) ),
                    }
                    
                },
                TokenKind::Float(_) | TokenKind::Variable(_) | TokenKind::Plus | TokenKind::Minus | TokenKind::Mul | TokenKind::Div => { // 代入式ではない場合は ans 変数に計算結果を入れる
                    Self::replace_variable(self.tree.as_mut(), vars)?;

                    let ans = Self::calculate(self.tree.as_ref())?;
                                        
                    return Ok( VarData("ans".to_string(), ans) )
                }
                _ => return Err( FormulaErr::new(ErrType::InvalidFormula, "invalid formula", n.as_ref().loc) ),
            }
        }

        return Err( FormulaErr::new(ErrType::NoTree, "can not calculate empty tree.", Loc(0, 0)) )
    }

    fn replace_variable(node: Option<&mut Node<Token>>, vars: &VarPool) -> Result<(), FormulaErr> {
        match node {
            Some(n) => {
                n.foreach_mut(&SearchOrder::PreOrder, &mut |elem: &mut Token| {
                    if let TokenKind::Variable(v) = &elem.value {                        
                        if let Some(f) = vars.get(v) {          // ここでは、定義されている変数を見つけた場合には値に置き換えている。　
                            elem.value = TokenKind::Float(f);  // Noneが来た場合は何もしない　→　calculate関数を実行したときにVariableを見つけたら未定義としてはじく
                        }
                    }
                });
            },
            None => return Err(FormulaErr::new(ErrType::EmptyFormula, "enmpty formula is found.", Loc(0, 0))),
        }
        Ok(())
    }

    fn calculate(node: Option<&Node<Token>>) -> Result<f64, FormulaErr> { // ツリーから計算を行う
        match node {
            Some(n) => {
                match n.as_ref().value {
                    TokenKind::Plus     => return Ok( Self::calculate(n.left())? + Self::calculate(n.right())? ),
                    TokenKind::Minus    => return Ok( Self::calculate(n.left())? - Self::calculate(n.right())? ),
                    TokenKind::Mul      => return Ok( Self::calculate(n.left())? * Self::calculate(n.right())? ),
                    TokenKind::Div      => {
                        let right = Self::calculate(n.right())?;
                        if right == 0.0 { return Err( FormulaErr::new(ErrType::ZeroDiv, "Divided by zero!", n.as_ref().loc ) ); }
                        return Ok( Self::calculate(n.left())? / right );
                    }, 
                    TokenKind::Mod      => {
                        let right = Self::calculate(n.right())?;
                        if right == 0.0 { return Err( FormulaErr::new(ErrType::ZeroDiv, "Divided by zero!", n.as_ref().loc ) ); }
                        return Ok( Self::calculate(n.left())? % right );
                    }
                    TokenKind::Float(f) => return Ok( f ),
                    TokenKind::Variable(_) => return Err( FormulaErr::new(ErrType::UndefinedVariable, "Undefined Variable is found.", n.as_ref().loc ) ),
                    _ => return Err( FormulaErr::new(ErrType::InvalidFormula, "Unexpected Error", Loc(0, 0) ) ),
                }
            }
            None => { return Err( FormulaErr::new(ErrType::IInvalidTree, "Invalid tree", Loc(0, 0) ) ); } // 演算子の子がNoneはありえない
        }
    }

    pub fn parse(&mut self, formula: &str) -> Result<(), FormulaErr> {
        self.formula_str = formula.to_string();

        match Self::lexer(formula) {
            Ok(tokens) => {
                match Self::parser(&tokens) {
                    Ok(tree) => self.tree = Some(tree),
                    Err(mut e) => { e.formula = self.formula_str.clone(); return Err(e); }
                }
            },
            Err(mut e) => { e.formula = self.formula_str.clone(); return Err(e); }
        }

        Ok(())
    }

    fn parser(tokens: &[Token]) -> Result<Node<Token>, FormulaErr> { // https://smdn.jp/programming/tips/polish/を参考に構文解析をする
        // Step. 0: カッコの数をチェック & カッコ外し
        let tokens = Self::check_brackets(tokens)?;

        // Step. 1: tokensの長さが1の時(木の末端)　or tokensの長さが2の時（単項演算子）
        if tokens.len() == 1 {
            if TokenKind::is_value(&tokens[0].value) {
                return Ok( Node::new(tokens[0].clone()) );
            } else {
                return Err( FormulaErr::new(ErrType::InvalidFormula, "a float value is expected, but other token is found.", tokens[0].loc) );
            }
        } else if tokens.len() == 2 {
            match tokens[0].value {
                TokenKind::Plus => {
                    match tokens[1].value {
                        TokenKind::Float(_) => return Ok( Node::new(tokens[1].clone()) ),
                        _ => return Err( FormulaErr::new(ErrType::InvalidFormula, "a float value is expected, but other token is found.", tokens[1].loc) ),
                    }
                },
                TokenKind::Minus => {
                    match tokens[1].value {
                        TokenKind::Float(f) => {
                            let mut n = Node::new(tokens[1].clone());
                            n.as_mut().value = TokenKind::Float(-f);
                            return Ok(n);
                        },
                        _ => return Err( FormulaErr::new(ErrType::InvalidFormula, "a float value is expected, but other token is found.", tokens[1].loc) ),
                    }
                },
                _ => return Err( FormulaErr::new(ErrType::InvalidFormula, "'+' or '-' is expected, but other token is found.", tokens[0].loc) ),
            }
        }
        
        // Step. 2: 式の中で最も右にありかつ優先度の低い演算子を抽出する。
        let mut priority = 0;
        let mut target_ope = 0; // 一番優先度の低い演算子の番号
        let mut ope_found = false; // 演算子があるかないか
        let mut braket_cnt = 0;
        let mut loc = Loc(0, 0);

        for (idx, token) in tokens.iter().enumerate() {   
            
            loc.1 = token.loc.1;
            match token.value {
                TokenKind::LParen => braket_cnt += 1,
                TokenKind::RParen => braket_cnt -= 1,
                _ => {
                    if idx == 0 { loc.0 = token.loc.0; continue; }     // 一番左に単項演算子がある場合を想定　-1 + 2　この-1はStep. 1の単項演算子処理部で処理される
                    
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
        if ope_found == false { return Err( FormulaErr::new(ErrType::InvalidFormula, "Required operator is not found.", loc ) ); }

        // Step. 2.1: target_opeの左隣も四則演算の場合
        if TokenKind::is_alsoperator(&tokens[target_ope - 1].value) {
            match tokens[target_ope].value {
                TokenKind::Plus | TokenKind::Minus => target_ope -= 1,
                _ => return Err( FormulaErr::new(ErrType::InvalidOperator, "Invalid Operator is found.", tokens[target_ope].loc ) ),
            }
        }

        // Step. 3: 一番優先度の低い演算子でノードを作成
        let mut node = Node::new(tokens[target_ope].clone());
        
        // Step. 4: 演算子を中心に左と右に分ける
        println!("{:?}", target_ope);
        node.add_node_left( Self::parser(&tokens[0..target_ope])? )?;
        node.add_node_right( Self::parser(&tokens[target_ope + 1..])? )?;
        
        Ok( node )
    }

    fn lexer(formula: &str) -> Result<Vec<Token>, FormulaErr> {
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
                tokens.push($fn_value(input, &mut pos)?);
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
                _ => return Err( FormulaErr::new(ErrType::InvalidChar(input[pos] as char), "Invalid char is found.", Loc(pos, pos + 1)) ),
            }
        }
        
        Ok(tokens)
    }

    fn lex_variable(input: &[u8], pos: &mut usize) -> Result<Token, FormulaErr> {
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
                return Err( FormulaErr::new(ErrType::InvalidChar(input[end] as char), "Invalid char is found.", Loc(start, end + 1)) );
            }
        }

        *pos = end;
        let variable_name = from_utf8(&input[start..end]).unwrap().to_string();
        Ok( Token{value: TokenKind::Variable(variable_name), loc: Loc(start, end)} )
    }

    fn lex_number(input: &[u8], pos: &mut usize) -> Result<Token, FormulaErr> {
        use std::str::from_utf8;
        let start = *pos;
        let mut end = *pos + 1;
        let mut decpoint = false;

        while end < input.len() && b"0123456789.".contains(&input[end]) {
            if input[end] == b'.' { // 2回目の小数点が現れたら
                if decpoint == true {
                    return Err( FormulaErr::new(ErrType::InvalidFloatValue, "Invalid float number is found.", Loc(start, end + 1)) );
                }
                decpoint = true;
            }
            end += 1;
        }
        
        let value = from_utf8(&input[start..end]).unwrap().parse().unwrap();   

        *pos = end;
        Ok( Token{value: TokenKind::Float(value), loc: Loc(start, end)} )
    }

    fn check_brackets(tokens: &[Token]) -> Result<&[Token], FormulaErr> {
        let mut checker = 0;

        println!("{:?}\n", tokens);

        // カッコの数が間違っていないかチェック
        for t in tokens.iter() {
            match t.value {
                TokenKind::LParen => checker += 1,
                TokenKind::RParen => checker -= 1,
                _ => (),
            }
        }
        
        if checker > 0 {
            for t in tokens.iter() {
                if let TokenKind::LParen = t.value {
                    return Err( FormulaErr::new(ErrType::InvalidBracket, "an extra ( is found.", t.loc) );
                }
            }
        } else if checker < 0 {
            for t in tokens.iter().rev() {
                if let TokenKind::RParen = t.value {
                    return Err( FormulaErr::new(ErrType::InvalidBracket, "an extra ) is found.", t.loc) );
                }
            }
        }

        // 最初が'('で最後が')'だったら一番外側のカッコを外す
        if tokens[0].value == TokenKind::LParen && tokens[tokens.len() - 1].value == TokenKind::RParen {
            Ok(&tokens[1..tokens.len() - 1])
        } else {
            Ok(tokens)
        }
    }
}

#[derive(Debug)]
pub struct FormulaErr {
    formula: String,
    err_type: ErrType,
    err_msg: String,
    loc: Loc,
}

impl FormulaErr {
    fn new(etype: ErrType, msg: &str, loc: Loc) -> Self {
        Self {
            formula: String::new(),
            err_type: etype,
            err_msg: msg.to_string(),
            loc: loc,
        }
    }

    pub fn print(&self) {
        let token_len = self.loc.1 - self.loc.0;

        println!("{}", self.formula);
        if token_len > 0 {
            println!("{}{}", " ".repeat(self.loc.0), "^".repeat(token_len).yellow() );
        }
        println!("{}", self.err_msg.blue());
    }
}

impl From<NodeError> for FormulaErr {
    fn from(e: NodeError) -> Self {
        FormulaErr::new(ErrType::NodeError(e), "bintree error is occuerd", Loc(0, 0))
    }
}

pub trait Formula {
    fn to_formula(&self) -> Result<FormulaCalculator, FormulaErr>;
}

impl<'a> Formula for &'a str {
    fn to_formula(&self) -> Result<FormulaCalculator, FormulaErr> {
        FormulaCalculator::set_formula(self)
    }
}

impl Formula for String {
    fn to_formula(&self) -> Result<FormulaCalculator, FormulaErr> {
        FormulaCalculator::set_formula(self)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ErrType {
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

impl From<NodeError> for ErrType {
    fn from(e: NodeError) -> Self {
        ErrType::NodeError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_test() {
        let mut pool = VarPool::new();

        let mut fc = FormulaCalculator::set_formula("x = (1 + 2) * 3").unwrap();
        assert_eq!(fc.calc(&pool).unwrap().1, 9.0);

        let mut fc = FormulaCalculator::set_formula("-1 * 2").unwrap();
        assert_eq!(fc.calc(&pool).unwrap().1, -2.0);

        let mut fc = FormulaCalculator::set_formula("-1 * -2").unwrap();
        assert_eq!(fc.calc(&pool).unwrap().1, 2.0);

        let mut fc = FormulaCalculator::set_formula("1 + -2 * (3 + 2)").unwrap();
        assert_eq!(fc.calc(&pool).unwrap().1, -9.0);

        let mut fc = FormulaCalculator::set_formula("1  2 * 3 * 8.5");
        assert_eq!(fc.unwrap_err().err_type, ErrType::InvalidFormula);

        let mut fc = FormulaCalculator::set_formula("1 + 2 / 0").unwrap();
        assert_eq!(fc.calc(&pool).unwrap_err().err_type, ErrType::ZeroDiv);

        let mut fc = FormulaCalculator::set_formula("1 + 2 * (3 + 15 / (1 + 3)) + 1 / 2").unwrap();
        assert_eq!(fc.calc(&pool).unwrap().1, 15.0);

        let mut fc = FormulaCalculator::set_formula("1 + 2 * (3 + 15 / (1 + 3)) + 1 / 2)");
        assert_eq!(fc.unwrap_err().err_type, ErrType::InvalidBracket);

        let mut fc = FormulaCalculator::set_formula("1 + 2 * * 3 * 8.5");
        assert_eq!(fc.unwrap_err().err_type, ErrType::InvalidOperator);
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

        assert_eq!(token.unwrap_err().err_type, ErrType::InvalidFloatValue);
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

        assert_eq!(token.unwrap_err().err_type, ErrType::InvalidChar('#'));
    }
}

