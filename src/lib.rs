#![crate_name = "rustache"]

use std::collections::HashMap;
use std::fmt;
use std::io::File;
use template::Template;
use compiler::Compiler;
use parser::{Parser, Node};

pub use build::{HashBuilder, VecBuilder};

pub fn render<'a, W: Writer>(path: Path, data: &HashBuilder, writer: &mut W) {
    let file = Read::read_file(path);
    let compiler = Compiler::new(file.as_slice());
    let parser = Parser::new(&compiler.tokens);
    Template::render_data(writer, data, &parser);
}

pub fn render_text<'a, W: Writer>(input: &'a str, data: &HashBuilder, writer: &mut W) {
    let compiler = Compiler::new(input);
    let parser = Parser::new(&compiler.tokens);
    Template::render_data(writer, data, &parser);
}

pub struct Read;

impl Read {
    pub fn read_file(path: Path) -> String {
        // let path = Path::new(template_path);
        // Open the file path
        let mut file = match File::open(&path) {
            Err(why) => fail!("{}", why.desc),
            Ok(file) => file,
        };

        // Read the file contents into a heap allocated string
        let contents = match file.read_to_string() {
            Err(why) => fail!("{}", why.desc),
            Ok(text) => text,
        };

        contents
    }
}

/// Represents the possible types that passed in data may take on
pub enum Data<'a> {
    Strng(String),
    Bool(bool),
    Vector(Vec<Data<'a>>),
    Hash(HashMap<String, Data<'a>>)
}

impl<'a> PartialEq for Data<'a> {
    fn eq(&self, other: &Data<'a>) -> bool {
        match (self, other) {
            (&Strng(ref val0), &Strng(ref val1)) => val0 == val1,
            (&Bool(ref val0), &Bool(ref val1)) => val0 == val1,
            (&Vector(ref val0), &Vector(ref val1)) => val0 == val1,
            (&Hash(ref val0), &Hash(ref val1)) => val0 == val1,
            (_, _) => false
        }
    }
}

impl<'a> fmt::Show for Data<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Strng(ref val) => write!(f, "String({})", val),
            Bool(val)    => write!(f, "Boolean({})", val),
            Vector(ref val) => write!(f, "Vector({})", val),
            Hash(ref val)    => write!(f, "Hash({})", val) 
        }
    }
}



mod compiler;
mod parser;
mod build;
mod template;
