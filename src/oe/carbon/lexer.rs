use logos::{Logos, Lexer};
use std::convert::Infallible;
//use std::ffi::CString;
use compact_str::CompactString;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other
}

impl From<std::num::ParseIntError> for LexingError {
   fn from(_: std::num::ParseIntError) -> Self {
      LexingError::NumberParseError
  }
}

impl From<std::num::ParseFloatError> for LexingError {
  fn from(_: std::num::ParseFloatError) -> Self {
     LexingError::NumberParseError
  }
}

impl From<Infallible> for LexingError {
  fn from(_: Infallible) -> Self {
     LexingError::Other
  }
}

fn newline_callback<'a>(lex: &mut Lexer<'a, Token<'a>>) -> usize{
    lex.extras += 1;
    lex.extras
}

#[derive(Logos, Debug, PartialEq, Clone, Default)]
#[logos(error = LexingError)]
#[logos(extras = usize)]
#[logos(skip r"[ \t\f]+")]
pub enum Token<'a>{

    #[regex(r#"[_a-zA-Z\u0080-\uFFFF][_a-zA-Z0-9\u0080-\uFFFF]*"#, |lex| Some(lex.slice()))]
    IdentifierDef(Option<&'a str>),

    #[regex("-?[0-9]+", callback = |lex| lex.slice().parse())]
    IntegerDef(i32),

    #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    FloatDef(f64),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| {let temp = lex.slice();
                                                                    if temp.len() > 2{
                                                                     Some(&lex.slice()[1..temp.len()-1])
                                                                    }
                                                                    else{
                                                                     Some(lex.slice())
                                                                    }})]
    StringDef(Option<&'a str>),

    #[token("\n", newline_callback)]
    NewLineDef(usize),

    #[token("<")]
    LTDef,

    #[token(">")]
    GTDef,

    #[token("/")]
    SlashDef,

    #[token("</")]
    LtSlashDef,

    #[token(";")]
    SemiColonDef,

    #[token(",")]
    CommaDef,

    #[default]
    #[token("=")]
    EqualDef,

    #[regex("\\{")]
    OpenBraceDef,

    #[regex("\\}")]
    CloseBraceDef,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenContent{
    Int(i32),
    Float(f64),
    SomeStr(Box<CompactString>),
    IntList(Box<Vec<i32>>),
    FloatList(Box<Vec<f64>>),
    StringList(Box<Vec<CompactString>>),
}

impl fmt::Display for TokenContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl<'a> Token<'a> {
  //NOTE: leaks the content
  pub fn get_content(&self) -> Option<TokenContent>{
    match self {
      //Token::IdentifierDef(s) => Some(TokenContent::SomeStr(Box::new(CString::new(*(s.as_ref().unwrap())).ok()?))),//Some(TokenContent::SomeStr(Box::new(CompactString::new(Box::<&str>::leak(s.clone()?))))),
      //Token::StringDef(s) =>Some(TokenContent::SomeStr(Box::new(CString::new(*(s.as_ref().unwrap())).ok()?))),//Some(TokenContent::SomeStr(Box::new(CompactString::new(Box::<&str>::leak(s.clone()?))))),
      Token::IdentifierDef(s) => Some(TokenContent::SomeStr(Box::new(CompactString::new(s.unwrap())))),
      Token::StringDef(s) =>Some(TokenContent::SomeStr(Box::new(CompactString::new(s.unwrap())))),//
      Token::IntegerDef(s) => Some(TokenContent::Int(*s)),
      Token::FloatDef(s) => Some(TokenContent::Float(*s)),
      _ => None
    }
  }
}

impl TokenContent {
  pub fn get_str(&self) -> Option<&str>{
    match self {
      TokenContent::SomeStr(s) => Some(&s),//Some(s.to_str().unwrap()),
      //TokenContent::SomeStr(s) => Some(s.to_str().unwrap()),
      _ => None
    }
  }
  pub fn get_int(&self) -> Option<i32>{
    match self {
      TokenContent::Int(s) => Some(*s),
      _ => None
    }
  }
  pub fn get_float(&self) -> Option<f64>{
    match self {
      TokenContent::Float(s) => Some(*s),
      _ => None
    }
  }
  pub fn get_str_list(&self) -> Option<&Vec<CompactString>>{
    match self {
      TokenContent::StringList(s) => Some(&*s),
      _ => None
    }
  }
  pub fn get_int_list(&self) -> Option<&Vec<i32>>{
    match self {
      TokenContent::IntList(s) => Some(&*s),
      _ => None
    }
  }
  pub fn get_float_list(&self) -> Option<&Vec<f64>>{
    match self {
      TokenContent::FloatList(s) => Some(&*s),
      _ => None
    }
  }
}
