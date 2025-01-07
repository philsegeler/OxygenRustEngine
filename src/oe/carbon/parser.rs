use std::collections::HashMap;
use super::lexer::*;
use logos::{Logos};
use std::ffi::CString;


use compact_str::CompactString;
type ParserMap<T> = HashMap<CompactString, T>;

#[derive(Default, Debug)]
pub struct Element {
    pub elements : ParserMap<Vec<Element>>,

    pub attributes : ParserMap<TokenContent>,
    pub assignments : ParserMap<TokenContent>,
}

impl Element {
    pub fn print_oneself(self) -> String{
        let mut output = String::new();
        output = output + " Element ";
        for (key, val) in self.attributes{
            output = output + &key.to_string() + " = " + &val.to_string() + " ";
        }
        output = output + ">\n";

        for (key, val) in self.assignments{
            output = output + &key.to_string() + " = " + &val.to_string() + "\n";
        }
        /*for (key, val) in self.list_assignments{
            let mut lista = String::new();
            for subval in val {
                lista = lista + &subval.to_string() + ",";
            }
            output = output + key.to_str().unwrap() + " = {" + &lista + "}\n";
        }*/
        for (key, elems) in self.elements{
            for elem in elems{
                output = output + "<" + &key.to_string() + " " + &elem.print_oneself();
            }
        }
        output = output + "</Element>\n";
        output
    }
}


pub struct Parser<T, A> {
    token_it_ : A,
    cur_token_ : T,
    cur_line_ : usize,
}

impl<'a, A> Parser<Token<'a>, A> where A : Iterator<Item=Result<Token<'a>, LexingError>> {

    fn new(mut tokens : A) -> Parser<Token<'a>, A>{
        Parser{
            cur_token_ : tokens.next().unwrap().unwrap(),
            token_it_ : tokens,
            cur_line_ : 0,
        }
    }

    pub fn parse(&mut self) -> Element {
        self.expect_token(&Token::LTDef);
        self.pass_token();

        self.parse_element()
    }

    fn parse_element(&mut self) -> Element{
        let mut result : Element = Default::default();

        // parse opening tag
        self.expect_token(&Token::IdentifierDef(None)).unwrap();

        let el_name = self.get_token_content();
        self.pass_token();

        while self.has_type(&Token::IdentifierDef(None)) {
            let att_name = CompactString::new(self.get_token_content().get_str().unwrap());

            self.pass_token();
            self.expect_token(&Token::EqualDef).unwrap();
            self.pass_token();
            result.attributes.insert(att_name, self.parse_single_assignment());
        }

        self.expect_token(&Token::GTDef).unwrap();
        self.pass_token();

        // parse element content
        loop {
            //println!("blabl {:?}", self.get_token());
            if self.has_type(&Token::IdentifierDef(None)) {

                let as_name = CompactString::new(self.get_token_content().get_str().unwrap());
                self.pass_token();

                self.expect_token(&Token::EqualDef).unwrap();
                self.pass_token();

                result.assignments.insert(as_name, self.parse_assigment());
            }
            else if self.has_type(&Token::LTDef) {
                self.pass_token();
                let sub_el_name = CompactString::new(self.get_token_content().get_str().unwrap());
                if ! result.elements.contains_key(&sub_el_name){
                    result.elements.insert(sub_el_name.clone(), Default::default());
                }
                result.elements.get_mut(&sub_el_name).unwrap().push(self.parse_element())

            }
            else {
                break;
            }
        }

        // parse closing tag
        self.expect_tokens(&[Token::LtSlashDef, Token::LTDef]).unwrap();
        self.pass_token();

        self.expect_token(&Token::IdentifierDef(None)).unwrap();
        if el_name != self.get_token_content(){
            panic!("Closing tag identifier does not match opening tag identifier.");
        }

        self.pass_token();
        self.expect_token(&Token::GTDef).unwrap();
        self.pass_token();

        result.elements.shrink_to_fit();
        //result.list_assignments.shrink_to_fit();
        result.assignments.shrink_to_fit();
        result
    }
    fn get_token(&self) -> Token {
        //self.token_it_[self.cur_index_].as_ref().unwrap().clone()
        self.cur_token_.clone()
    }
    fn get_token_content(&self) -> TokenContent{
        //let content = self.token_it_[self.cur_index_].as_ref().unwrap();
        let content = &self.cur_token_;
        //println!("{:?}", content);
        content.get_content().unwrap()
    }

    fn parse_assigment(&mut self) -> TokenContent{
        self.expect_tokens(&[Token::IntegerDef(0), Token::FloatDef(0.0), Token::IdentifierDef(None), Token::OpenBraceDef, Token::StringDef(None)]).unwrap();

        if self.has_type(&Token::OpenBraceDef){
            self.pass_token();
            self.parse_list_assignment().unwrap()
        }
        else {
            self.parse_single_assignment()
        }
    }

    fn parse_single_assignment(&mut self) -> TokenContent {
        let output = self.get_token_content();
        self.pass_token();
        output
    }
    fn parse_list_assignment(&mut self) -> Option<TokenContent> {
        //let mut result : Vec<TokenContent> = Default::default();

        //let mut result : TokenContent;
        let is_int = self.is_int();
        let is_float = self.is_float();
        let is_str = self.is_str();


        if is_float{
            let mut result : Vec<f64> = Vec::with_capacity(4);
                result.push(self.get_token_content().get_float().unwrap());

                self.pass_token();
                while self.has_type(&Token::SemiColonDef){
                    self.pass_token();
                    self.expect_value().unwrap();

                    result.push(self.get_token_content().get_float().unwrap());
                    self.pass_token();
                }
            self.expect_token(&Token::CloseBraceDef).unwrap();
            self.pass_token();
            Some(TokenContent::FloatList(Box::new(result)))
        }
        else if is_int{
            let mut result : Vec<i64> = Vec::with_capacity(4);
                result.push(self.get_token_content().get_int().unwrap());

                self.pass_token();
                while self.has_type(&Token::SemiColonDef){
                    self.pass_token();
                    self.expect_value().unwrap();

                    result.push(self.get_token_content().get_int().unwrap());
                    self.pass_token();
                }
            self.expect_token(&Token::CloseBraceDef).unwrap();
            self.pass_token();
            Some(TokenContent::IntList(Box::new(result)))
        }
        else if is_str{
            let mut result : Vec<CString> = Vec::with_capacity(4);
                result.push(CString::new(self.get_token_content().get_str().unwrap()).unwrap());

                self.pass_token();
                while self.has_type(&Token::SemiColonDef){
                    self.pass_token();
                    self.expect_value().unwrap();

                    result.push(CString::new(self.get_token_content().get_str().unwrap()).unwrap());
                    self.pass_token();
                }
            self.expect_token(&Token::CloseBraceDef).unwrap();
            self.pass_token();
            Some(TokenContent::StringList(Box::new(result)))
        }
        else {
            None
        }
    }

    fn expect_tokens(&self, comparison : &[Token]) -> Option<bool>{
        for token in comparison{
            match self.expect_token(token){
                Some(_) => {return Some(true);}
                None => {continue;}
            };
        }
        None
    }

    fn expect_token(&self, comparison : &Token) -> Option<bool>{
        //println!("{:?} {:?}", self.token_it_[self.cur_index_].0.as_ref().unwrap(), comparison);
        //if std::mem::discriminant(self.token_it_[self.cur_index_].as_ref().unwrap()) == std::mem::discriminant(comparison) {
        if std::mem::discriminant(&self.cur_token_) == std::mem::discriminant(comparison) {
            Some(true)
        }
        else{
            None
        }
    }

    fn expect_value(&self) -> Option<bool>{
        self.expect_tokens(&[Token::IntegerDef(4), Token::FloatDef(0.0), Token::IdentifierDef(None), Token::StringDef(None)])
    }

    fn pass_token(&mut self){
        match self.token_it_.next(){
            Some(s) => {self.cur_token_ = s.unwrap();}
            None => {return;}
        }
        while self.has_type(&Token::NewLineDef(0)){
            match self.token_it_.next(){
                Some(s) => {self.cur_token_ = s.unwrap();}
                None => {return;}
            }
            self.cur_line_ = match self.cur_token_ {
                Token::NewLineDef(s) => s,
                _ => self.cur_line_,
            }
        }
    }
    fn is_int(&self) -> bool {
        self.expect_token(&Token::IntegerDef(4)).unwrap_or(false)
    }
    fn is_str(&self) -> bool {
        self.expect_token(&Token::StringDef(None)).unwrap_or(false)
    }
    fn is_float(&self) -> bool {
        self.expect_token(&Token::FloatDef(0.0)).unwrap_or(false)
    }
    fn has_type(&self, comparison : &Token) -> bool{
        self.expect_token(comparison).unwrap_or(false)
    }
}

pub fn parse_string(input_str : &String) -> Element {
    let tokens: _ = Token::lexer(input_str.as_str()).spanned().map(|x| x.0).into_iter();
    //tokens = 5;
    let mut parser = Parser::new(tokens);

    parser.parse()
}


