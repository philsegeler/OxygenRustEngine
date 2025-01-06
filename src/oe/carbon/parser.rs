use std::collections::HashMap;
use super::lexer::*;
use logos::{Logos, SpannedIter};
use std::mem::discriminant;
use std::ffi::CString;
use std::ops::Range;

type ParserMap<T> = HashMap<CString, T>;

type SingleAssignment = TokenContent;
type ListAssignment = Vec<TokenContent>;

enum GenericAssignment{
    SingleA(SingleAssignment),
    ListA(ListAssignment)
}


#[derive(Default, Debug)]
pub struct Element {
    pub elements : ParserMap<Vec<Element>>,

    pub attributes : ParserMap<SingleAssignment>,
    pub single_assignments : ParserMap<SingleAssignment>,
    pub list_assignments : ParserMap<ListAssignment>,
}

impl Element {
    pub fn print_oneself(self) -> String{
        let mut output = String::new();
        output = output + " Element ";
        for (key, val) in self.attributes{
            output = output + key.to_str().unwrap() + " = " + &val.to_string() + " ";
        }
        output = output + ">\n";

        for (key, val) in self.single_assignments{
            output = output + key.to_str().unwrap() + " = " + &val.to_string() + "\n";
        }
        for (key, val) in self.list_assignments{
            let mut lista = String::new();
            for subval in val {
                lista = lista + &subval.to_string() + ",";
            }
            output = output + key.to_str().unwrap() + " = {" + &lista + "}\n";
        }
        for (key, elems) in self.elements{
            for elem in elems{
                output = output + "<" + key.to_str().unwrap() + " " + &elem.print_oneself();
            }
        }
        output = output + "</Element>\n";
        output
    }
}

#[derive(Default, Debug)]
pub struct Parser<T> {
    token_it_ : Vec<T>,
    cur_index_ : usize,
}

impl<'a> Parser<(Result<Token<'a>, LexingError>, Range<usize>)> {

    fn new(tokens : Vec<(Result<Token, LexingError>, Range<usize>)>) -> Parser<(Result<Token, LexingError>, Range<usize>)>{
        Parser{
            token_it_ : tokens,
            cur_index_ : 0,
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
        self.expect_token(&Token::IdentifierDef("")).unwrap();

        let el_name = self.get_token_content();
        self.pass_token();

        while self.has_type(&Token::IdentifierDef("")) {
            let att_name = CString::new(self.get_token_content().get_str().unwrap()).unwrap();

            self.pass_token();
            self.expect_token(&Token::EqualDef).unwrap();
            self.pass_token();
            result.attributes.insert(att_name, self.parse_single_assignment());
        }

        self.expect_token(&Token::GTDef).unwrap();
        self.pass_token();

        // parse element content
        while self.cur_index_ != self.token_it_.len() {
            //println!("blabl {:?}", self.get_token());
            if self.has_type(&Token::IdentifierDef("")) {

                let as_name = CString::new(self.get_token_content().get_str().unwrap()).unwrap();
                self.pass_token();

                self.expect_token(&Token::EqualDef).unwrap();
                self.pass_token();

                match self.parse_assigment(){
                    GenericAssignment::SingleA(s) => {
                        result.single_assignments.insert(as_name, s);
                    }
                    GenericAssignment::ListA(s) => {
                        result.list_assignments.insert(as_name, s);
                    }
                }

            }
            else if self.has_type(&Token::LTDef) {
                self.pass_token();
                let sub_el_name = CString::new(self.get_token_content().get_str().unwrap()).unwrap();
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

        self.expect_token(&Token::IdentifierDef("")).unwrap();
        if el_name != self.get_token_content(){
            panic!("Closing tag identifier does not match opening tag identifier.");
        }

        self.pass_token();
        self.expect_token(&Token::GTDef).unwrap();
        self.pass_token();


        result
    }
    fn get_token(&self) -> Token {
        self.token_it_[self.cur_index_].0.as_ref().unwrap().clone()
    }
    fn get_token_content(&self) -> TokenContent{
        let content = self.token_it_[self.cur_index_].0.as_ref().unwrap();
        //println!("{:?}", content);
        content.get_content().unwrap()
    }

    fn parse_assigment(&mut self) -> GenericAssignment{
        self.expect_tokens(&[Token::IntegerDef(0), Token::FloatDef(0.0), Token::IdentifierDef(""), Token::OpenBraceDef, Token::StringDef("")]).unwrap();

        if self.has_type(&Token::OpenBraceDef){
            self.pass_token();
            GenericAssignment::ListA(self.parse_list_assignment())
        }
        else {
            GenericAssignment::SingleA(self.parse_single_assignment())
        }
    }

    fn parse_single_assignment(&mut self) -> TokenContent {
        let output = self.get_token_content();
        self.pass_token();
        output
    }
    fn parse_list_assignment(&mut self) -> Vec<TokenContent> {
        let mut result : Vec<TokenContent> = Default::default();
        if self.is_value(){
            result.push(self.get_token_content());

            self.pass_token();
            while self.has_type(&Token::SemiColonDef){
                self.pass_token();
                self.expect_value().unwrap();

                result.push(self.get_token_content());
                self.pass_token();
            }
        }
        self.expect_token(&Token::CloseBraceDef).unwrap();
        self.pass_token();
        result
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
        if std::mem::discriminant(self.token_it_[self.cur_index_].0.as_ref().unwrap()) == std::mem::discriminant(comparison) {
            Some(true)
        }
        else{
            None
        }
    }

    fn expect_value(&self) -> Option<bool>{
        self.expect_tokens(&[Token::IntegerDef(4), Token::FloatDef(0.0), Token::IdentifierDef(""), Token::StringDef("")])
    }

    fn pass_token(&mut self){
        self.cur_index_ += 1;
    }
    fn is_value(&self) -> bool {
        self.expect_tokens(&[Token::IntegerDef(4), Token::FloatDef(0.0), Token::IdentifierDef(""), Token::StringDef("")]).unwrap_or(false)
    }
    fn has_type(&self, comparison : &Token) -> bool{
        self.expect_token(comparison).unwrap_or(false)
    }
}

pub fn parse_string(input_str : &String) -> Element {
    let tokens: Vec<(Result<Token, LexingError>, Range<usize>)> = Token::lexer(input_str.as_str()).spanned().collect();
    let mut parser = Parser::new(tokens);

    parser.parse()
}


