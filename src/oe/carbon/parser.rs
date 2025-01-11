use std::collections::HashMap;
use super::lexer::*;
use logos::Logos;
//use std::ffi::CString;


use compact_str::CompactString;
pub type ParserMap<T> = HashMap<CompactString, T>;

#[derive(Default, Debug, PartialEq)]
pub struct Element {
    elements_ : Option<ParserMap<Vec<ElementEnum>>>,

    attributes_ : Option<ParserMap<TokenContent>>,
    assignments_ : ParserMap<TokenContent>,
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct TriangleElement{
    pub v1 : [i32; 8],
    pub v2 : [i32; 8],
    pub v3 : [i32; 8],
    pub num_of_uvs : u8
}

impl TriangleElement{
    pub fn print_oneself(&self) -> String{
        let mut output = String::new();
        output = output + " Triangle ";
        output = output + ">\n";

        output = output + "v1" + " = " + &format!("{:?}", &self.v1[0..self.num_of_uvs as usize]) + "\n";
        output = output + "v2" + " = " + &format!("{:?}", &self.v2[0..self.num_of_uvs as usize]) + "\n";
        output = output + "v3" + " = " + &format!("{:?}", &self.v3[0..self.num_of_uvs as usize]) + "\n";

        output = output + "</Triangle>\n";
        output
    }
}

#[derive(Debug, PartialEq)]
pub enum ElementEnum{
    NormalElement(Box<Element>),
    TriangleElement(TriangleElement),
}

impl ElementEnum{
    pub fn print_oneself(&self) -> String{
        match self {
            ElementEnum::NormalElement(s) => s.print_oneself(),
            ElementEnum::TriangleElement(s) => s.print_oneself(),
        }
    }

    pub fn get(&self) -> Option<&Box<Element>>{
        match self {
            ElementEnum::NormalElement(s) => Some(s),
            ElementEnum::TriangleElement(_) => None,
        }
    }
    pub fn get_triangle(&self) -> Option<&TriangleElement>{
        match self {
            ElementEnum::TriangleElement(s) => Some(s),
            _ => None,
        }
    }
}

impl Element {

    pub fn elements(&mut self) -> &mut ParserMap<Vec<ElementEnum>>{
        if self.elements_ == None {self.elements_ = Some(Default::default());}
        return self.elements_.as_mut().unwrap();
    }
    pub fn elements_ref(&self) -> &ParserMap<Vec<ElementEnum>>{
        return self.elements_.as_ref().unwrap();
    }
    pub fn attributes(&mut self) -> &mut ParserMap<TokenContent>{
        if self.attributes_ == None {self.attributes_ = Some(Default::default());}
        return self.attributes_.as_mut().unwrap();
    }
    pub fn assignments(&mut self) -> &mut ParserMap<TokenContent>{
        return &mut self.assignments_;
    }
    pub fn attributes_ref(&self) -> &ParserMap<TokenContent>{
        return self.attributes_.as_ref().unwrap();
    }
    pub fn assignments_ref(&self) -> &ParserMap<TokenContent>{
        return &self.assignments_;
    }

    pub fn print_oneself(&self) -> String{
        let mut output = String::new();
        output = output + " Element ";
        for (key, val) in self.attributes_ref().iter(){
            output = output + &key.to_string() + " = " + &val.to_string() + " ";
        }
        output = output + ">\n";

        for (key, val) in self.assignments_ref().iter(){
            output = output + &key.to_string() + " = " + &val.to_string() + "\n";
        }
        for (key, elems) in self.elements_ref().iter(){
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

    pub fn parse(&mut self) -> Box<Element> {
        self.expect_token(&Token::LTDef);
        self.pass_token();

        match self.parse_element(){
            ElementEnum::NormalElement(s) => s,
            _ => panic!("wtf")
        }
    }

    fn parse_element(&mut self) -> ElementEnum{
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
            result.attributes().insert(att_name, self.parse_single_assignment());
        }

        self.expect_token(&Token::GTDef).unwrap();
        self.pass_token();

        if el_name.get_str().unwrap() == "Triangle" && result.attributes().len() == 0{
            return ElementEnum::TriangleElement(self.parse_triangle_element());
        }

        // parse element content
        loop {
            //println!("blabl {:?}", self.get_token());
            if self.has_type(&Token::IdentifierDef(None)) {

                let as_name = CompactString::new(self.get_token_content().get_str().unwrap());
                self.pass_token();

                self.expect_token(&Token::EqualDef).unwrap();
                self.pass_token();

                result.assignments().insert(as_name, self.parse_assigment());
            }
            else if self.has_type(&Token::LTDef) {
                self.pass_token();
                let sub_el_name = CompactString::new(self.get_token_content().get_str().unwrap());
                if ! result.elements().contains_key(&sub_el_name){
                    result.elements().insert(sub_el_name.clone(), Default::default());
                }
                result.elements().get_mut(&sub_el_name).unwrap().push(self.parse_element())

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

        result.elements().shrink_to_fit();
        result.assignments().shrink_to_fit();
        result.attributes().shrink_to_fit();
        ElementEnum::NormalElement(Box::new(result))
    }

    fn parse_triangle_element(&mut self) -> TriangleElement{
        let mut output : TriangleElement = Default::default();
        // parse element content
        loop {
            //println!("blabl {:?}", self.get_token());
            if self.has_type(&Token::IdentifierDef(None)) {

                let as_name = CompactString::new(self.get_token_content().get_str().unwrap());
                self.pass_token();

                self.expect_token(&Token::EqualDef).unwrap();
                self.pass_token();


                let assignment = self.parse_assigment();
                if as_name == "v1" {
                    let triangle_indices = assignment.get_int_list().unwrap();
                    output.num_of_uvs = triangle_indices.len() as u8;
                    for id in 0..triangle_indices.len(){
                        output.v1[id] = triangle_indices[id];
                    }
                }
                else if as_name == "v2" {
                    let triangle_indices = assignment.get_int_list().unwrap();
                    for id in 0..triangle_indices.len(){
                        output.v2[id] = triangle_indices[id];
                    }
                }
                else if as_name == "v3" {
                    let triangle_indices = assignment.get_int_list().unwrap();
                    for id in 0..triangle_indices.len(){
                        output.v3[id] = triangle_indices[id];
                    }
                }
                else {
                }
            }
            else {
                break;
            }
        }

        // parse closing tag
        self.expect_tokens(&[Token::LtSlashDef, Token::LTDef]).unwrap();
        self.pass_token();

        self.expect_token(&Token::IdentifierDef(None)).unwrap();
        if "Triangle" != self.get_token_content().get_str().unwrap(){
            panic!("Closing tag identifier does not match opening tag identifier.");
        }

        self.pass_token();
        self.expect_token(&Token::GTDef).unwrap();
        self.pass_token();

        output
    }

    fn get_token(&self) -> Token {
        self.cur_token_.clone()
    }
    fn get_token_content(&self) -> TokenContent{
        let content = self.cur_token_.get_content().unwrap();
        //println!("{:?}", content);
        content
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
            let mut result : Vec<i32> = Vec::with_capacity(4);
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
            let mut result : Vec<CompactString> = Vec::with_capacity(4);
                result.push(CompactString::new(self.get_token_content().get_str().unwrap()));

                self.pass_token();
                while self.has_type(&Token::SemiColonDef){
                    self.pass_token();
                    self.expect_value().unwrap();

                    result.push(CompactString::new(self.get_token_content().get_str().unwrap()));
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

pub fn parse_string(input_str : &str) -> Box<Element> {
    let tokens: _ = Token::lexer(input_str).spanned().map(|x| x.0).into_iter();
    let mut parser = Parser::new(tokens);

    parser.parse()
}


