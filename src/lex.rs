use crate::{types::{
    VyInteger,
    VyChar,
    VyBoolean,
    vytable::VyTable
}, opcodes::VyToken};

const VY_EOB: i32 = -1;

macro_rules! is_eob {
    ( $(),*) => {
        token != VY_EOB 
    };
}

macro_rules! add_keyword {
    ( $($self:ident, $key:ident, $id:expr ),*) => {
        todo!();
    };
}

pub struct VyLexer<'a> {
    keywords: VyTable,
    pub column: VyInteger,
    pub token: VyChar,
    pub prev_token: VyChar,
    compiler_error: &'a fn(String, Option<String>),
    pub last_line: VyInteger,
    pub current_line: VyInteger,
    eof: VyBoolean
}

impl VyLexer<'_> {
    fn new(error_func: &fn(String, Option<String>)) -> VyLexer
    {
        let ret = VyLexer
        {
            keywords: VyTable::new(),
            compiler_error: error_func,
            column: -1,
            token: '\n',
            prev_token: '\n',
            last_line: -1,
            current_line: 0,
            eof: false
        };

        add_keyword!(self, bool, TkBoolSig);
        add_keyword!(self, byte, TkByteSig);
        add_keyword!(self, int, TkIntSig);
        add_keyword!(self, float, TkFloatSig);
        add_keyword!(self, double, TkDoubleSig);
        add_keyword!(self, char, TkCharSig);
        add_keyword!(self, void, TkVoidSig);
        add_keyword!(self, string, TkStringSig);
        
        add_keyword!(self, fn, TkClosureSig);
        add_keyword!(self, class, TkClassSig);
        add_keyword!(self, array, TkArraySig);
        add_keyword!(self, table, TkTableSig);
        add_keyword!(self, enum, TkEnumSig);

        add_keyword!(self, if, TkIf);
        add_keyword!(self, else, TkElse);
        add_keyword!(self, while, TkWhile);
        add_keyword!(self, for, TkFor);
        add_keyword!(self, in, TkIn);
        add_keyword!(self, notin, TkNotIn);
        add_keyword!(self, foreach, TkForEach);
        add_keyword!(self, return, TkReturn);
        add_keyword!(self, break, TkBreak);
        add_keyword!(self, continue, TkContinue);
        add_keyword!(self, import, TkImport);
        add_keyword!(self, typeof, TkTypeof);
        add_keyword!(self, new, TkNew);
        add_keyword!(self, delete, TkDelete);
        add_keyword!(self, null, TkNull);
        //add_keyword!(self, yield, TkYield);
        add_keyword!(self, this, TkThis);
        add_keyword!(self, super, TkSuper);
        //add_keyword!(self, resume, TkResume);
        add_keyword!(self, throw, TkThrow);
        add_keyword!(self, try, TkTry);
        add_keyword!(self, catch, TkCatch);
        add_keyword!(self, switch, TkSwitch);
        add_keyword!(self, case, TkCase);
        add_keyword!(self, default, TkDefault);
        add_keyword!(self, instanceof, TkInstanceof);
        add_keyword!(self, constructor, TkConstructor);
        add_keyword!(self, static, TkStatic);
        add_keyword!(self, public, TkPublic);
        add_keyword!(self, private, TkPrivate);
        add_keyword!(self, extends, TkExtends);
        add_keyword!(self, const, TkConst);
        add_keyword!(self, JSON, TkJSON);
        add_keyword!(self, async, TkAsync);
        add_keyword!(self, await, TkAwait);
        add_keyword!(self, thread, TkThread);
        //add_keyword!(self,generator, TkGenerator);
        add_keyword!(self, true, TkTrue);
        add_keyword!(self, false, TkFalse);

        return ret
    }

    pub fn lex() {
        todo!();
    }

    fn read_tokens(&self) {
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
                            return VyToken::TkDivideEq;
                        }
                    }
                }
                '#' => {
                    // compiler flags
                }
                '=' => {

                }
                _ => {
                    // gonna have to do a bunch of stuff for hex and all that jazz.
                    if(self.token.is_digit) {
                        return VyToken::TkInteger 
                    }
                    else if(self.token.is_alpha) {
                        return VyToken::TkIdentifier
                    }
                }
            }
        }
    }

    fn next() {
        todo!()
    }

    pub fn is_eof(&self) -> bool {
        return self.eof;
    }
}