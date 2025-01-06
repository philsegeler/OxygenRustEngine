use logos::Logos;
use std::convert::Infallible;
use std::ffi::CString;
use std::fmt;

// generic Token trait
pub trait TokenTrait : Clone {
  fn get_content(&self) -> Option<TokenContent>;
}


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

#[derive(Logos, Debug, PartialEq, Clone, Default)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a>{

    #[regex(r#"[_a-zA-Z\u0080-\uFFFF][_a-zA-Z0-9\u0080-\uFFFF]*"#, |lex| lex.slice())]
    IdentifierDef(&'a str),

    #[regex("-?[0-9]+", callback = |lex| lex.slice().parse())]
    IntegerDef(i64),

    #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    FloatDef(f64),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| {let temp = lex.slice();
                                                                    if temp.len() > 2{
                                                                     &lex.slice()[1..temp.len()-1]
                                                                    }
                                                                    else{
                                                                     lex.slice()
                                                                    }})]
    StringDef(&'a str),

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
    Int(i64),
    Float(f64),
    SomeStr(CString),
}

impl fmt::Display for TokenContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl<'a> TokenTrait for Token<'a> {
  fn get_content(&self) -> Option<TokenContent>{
    match self {
      Token::IdentifierDef(s) => Some(TokenContent::SomeStr(CString::new(*s).ok()?)),
      Token::StringDef(s) => Some(TokenContent::SomeStr(CString::new(*s).ok()?)),
      Token::IntegerDef(s) => Some(TokenContent::Int(*s)),
      Token::FloatDef(s) => Some(TokenContent::Float(*s)),
      _ => None
    }
  }
}

impl TokenContent {
  pub fn get_str(&self) -> Option<&str>{
    match self {
      TokenContent::SomeStr(s) => Some(s.to_str().unwrap()),
      _ => None
    }
  }
  pub fn get_int(&self) -> Option<i64>{
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
}
