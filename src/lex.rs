const VY_EOB: i32 = 0;

macro_rules! is_eob {
    ( $(),*) => {
        token != VY_EOB 
    };
}

macro_rules! add_keyword {
    ( $( $key:expr, $id:expr ),*) => {
        //todo
    };
}

struct VyLexer
{
    keywords: VyTable,
    column: VyInteger,
    token: VyChar,
    prev_token: VyChar,
    compiler_error: &fn(String, Optional<String>),
    last_line: VyInteger,
    current_line: VyInteger,
    eof: VyFalse
}

impl VyLexer
{
    fn new(error_func: &fn(String, Optional<String>)) -> VyLexer
    {
        let ret = VyLexer
        {
            keywords: VyTable::new(),
            compiler_error: error_func
        };

        add_keyword!("bool", TkBoolSig);
        add_keyword!("byte", TkByteSig);
        add_keyword!("int", TkIntSig);
        add_keyword!("float", TkFloatSig);
        add_keyword!("double", TkDoubleSig);
        add_keyword!("char", TkCharSig);
        add_keyword!("void", TkVoidSig);
        add_keyword!("string", TkStringSig);
        
        add_keyword!("fn", TkClosureSig);
        add_keyword!("class", TkClassSig);
        add_keyword!("array", TkArraySig);
        add_keyword!("table", TkTableSig);
        add_keyword!("enum", TkEnumSig);

        add_keyword!("if", TkIf);
        add_keyword!("else", TkElse);
        add_keyword!("while", TkWhile);
        add_keyword!("for", TkFor);
        add_keyword!("in", TkIn);
        add_keyword!("notin", TkNotIn);
        add_keyword!("foreach", TkForEach);
        add_keyword!("return", TkReturn);
        add_keyword!("break", TkBreak);
        add_keyword!("continue", TkContinue);
        add_keyword!("import", TkImport);
        add_keyword!("typeof", TkTypeof);
        add_keyword!("new", TkNew);
        add_keyword!("delete", TkDelete);
        add_keyword!("null", TkNull);
        add_keyword!("yield", TkYield);
        add_keyword!("this", TkThis);
        add_keyword!("super", TkSuper);
        add_keyword!("resume", TkResume);
        add_keyword!("throw", TkThrow);
        add_keyword!("try", TkTry);
        add_keyword!("catch", TkCatch);
        add_keyword!("switch", TkSwitch);
        add_keyword!("case", TkCase);
        add_keyword!("default", TkDefault);
        add_keyword!("instanceof", TkInstanceof);
        add_keyword!("constructor", TkConstructor);
        add_keyword!("static", TkStatic);
        add_keyword!("public", TkPublic);
        add_keyword!("private", TkPrivate);
        add_keyword!("extends", TkExtends);
        add_keyword!("const", TkConst);
        add_keyword!("JSON", TkJSON);
        add_keyword!("async", TkAsync);
        add_keyword!("await", TkAwait);
        add_keyword!("thread", TkThread);
        //add_keyword!("generator", TkGenerator);
        add_keyword!("true", TkTrue);
        add_keyword!("false", TkFalse);

        return ret
    }

    fn lex(&self) -> VyToken {
        while(self.token != VY_EOB) {
            match self.token {
                '\t' | '\r' | ' ' => {
                    self.next();
                }
                '\n' => {
                    self.current_line += 1;
                }
                '/' => {
                    self.next();
                    match self.token {
                        '/' => {
                            // do line comment
                        }
                        '*' => {
                            // do block comment
                        }
                        '=' => {
                            self.next();
                            return TkDivideEq;
                        }
                    }
                }
            }
        }
    }

    fn next() {
        //todo
    }
}