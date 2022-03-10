use crate::{
    vm::VyVM,
    opcodes::{VyToken, VyCompilerFlag},
    lex::VyLexer, types::VyInteger, object::VyObject
};

pub struct VyCompiler<'a> {
    token: VyToken,
    lexer: &'a VyLexer<'a>,
    sourcefile: &'a str,
    line: VyInteger,
    flags: VyCompilerFlag,
    stack: Vec<VyObject>
}

// macro_rules! lex {
//     ($self: ident) => {
//         $self.lexer.lex(); $self.token = $self.lexer.token;
//     };
// }

impl VyCompiler<'_> {
    fn compile(&self, vm: &VyVM, sourcefile: String, reader: &fn()) {
        while self.token != VyToken::TkEOF {
            self.statement();
        }
    }

    fn error(&self, msg: String, found: Option<String>) {
        let file_and_line = format!("{}:{}", self.sourcefile, self.line);

        // clean this up later im lazy
        if Some(found) == None {
            println!("{} Compiler Error: \"{}\"", file_and_line, msg);
        } else {
            println!("{} Compiler Error: \"{}\", found \"{}\"", file_and_line, msg, found.unwrap());
        }
    }

    fn statements(&self) {
        // figure out how compilers work
        while !self.lexer.is_eof() {
            self.statement();
        }
    }

    fn statement(&self) {
        match self.token {
            VyToken::TkIdentifier => {
                todo!();
            }
            _ => {
                //lex!(self);
            } 
        }
    }

    fn variable_statement(&self) {
        todo!();
    }

    fn function_statement(&self) {
        todo!();
    }

    fn expression(&self) {
        todo!();
    }
}