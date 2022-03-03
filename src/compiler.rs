use crate::{
    vm::VyVM,
    opcodes::VyToken,
    lex::VyLexer
};

pub struct VyCompiler<'a> {
    token: VyToken,
    lex: &'a VyLexer<'a>,
}

impl VyCompiler<'_> {
    fn compile(vm: &VyVM, sourcefile: String, reader: &fn()) {

    }
}