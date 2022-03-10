use crate::{types::{
    VyInteger,
    VyChar,
    VyBoolean,
    vystring::VyString, vytable::VyTable
}};

// const VY_EOB: i32 = -1;

// macro_rules! is_eob {
//     ( $(),*) => {
//         token != VY_EOB 
//     };
// }

macro_rules! add_keyword {
    ( $($lexer:ident, $key:ident, $id:expr ),*) => {
        
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
        let ret = VyLexer {
            keywords: VyTable::new(),
            compiler_error: error_func,
            column: -1,
            token: '\n',
            prev_token: '\n',
            last_line: -1,
            current_line: 0,
            eof: false
        };

        add_keyword!(ret, bool, TkBoolSig);
        add_keyword!(ret, byte, TkByteSig);
        add_keyword!(ret, int, TkIntSig);
        add_keyword!(ret, float, TkFloatSig);
        add_keyword!(ret, double, TkDoubleSig);
        add_keyword!(ret, char, TkCharSig);
        add_keyword!(ret, void, TkVoidSig);
        add_keyword!(ret, string, TkStringSig);
        
        add_keyword!(ret, fn, TkClosureSig);
        add_keyword!(ret, class, TkClassSig);
        add_keyword!(ret, array, TkArraySig);
        add_keyword!(ret, table, TkTableSig);
        add_keyword!(ret, enum, TkEnumSig);

        add_keyword!(ret, if, TkIf);
        add_keyword!(ret, else, TkElse);
        add_keyword!(ret, while, TkWhile);
        add_keyword!(ret, for, TkFor);
        add_keyword!(ret, in, TkIn);
        add_keyword!(ret, notin, TkNotIn);
        add_keyword!(ret, foreach, TkForEach);
        add_keyword!(ret, return, TkReturn);
        add_keyword!(ret, break, TkBreak);
        add_keyword!(ret, continue, TkContinue);
        add_keyword!(ret, import, TkImport);
        add_keyword!(ret, typeof, TkTypeof);
        add_keyword!(ret, new, TkNew);
        add_keyword!(ret, delete, TkDelete);
        add_keyword!(ret, null, TkNull);
        //add_keyword!(ret, yield, TkYield);
        add_keyword!(ret, this, TkThis);
        add_keyword!(ret, super, TkSuper);
        //add_keyword!(ret, resume, TkResume);
        add_keyword!(ret, throw, TkThrow);
        add_keyword!(ret, try, TkTry);
        add_keyword!(ret, catch, TkCatch);
        add_keyword!(ret, switch, TkSwitch);
        add_keyword!(ret, case, TkCase);
        add_keyword!(ret, default, TkDefault);
        add_keyword!(ret, instanceof, TkInstanceof);
        add_keyword!(ret, constructor, TkConstructor);
        add_keyword!(ret, static, TkStatic);
        add_keyword!(ret, public, TkPublic);
        add_keyword!(ret, private, TkPrivate);
        add_keyword!(ret, extends, TkExtends);
        add_keyword!(ret, const, TkConst);
        add_keyword!(ret, JSON, TkJSON);
        add_keyword!(ret, async, TkAsync);
        add_keyword!(ret, await, TkAwait);
        add_keyword!(ret, thread, TkThread);
        //add_keyword!(ret, generator, TkGenerator);
        add_keyword!(ret, true, TkTrue);
        add_keyword!(ret, false, TkFalse);

        return ret
    }

    // pub fn lex() {
    //     todo!();
    // }

    //todo: this
    // fn read_tokens(&self) {
    //     while(self.token != VY_EOB) {
    //         match self.token {
    //             '\t' | '\r' | ' ' => {
    //                 self.next();
    //             }
    //             '\n' => {
    //                 self.current_line += 1;
    //             }
    //             '/' => {
    //                 self.next();
    //                 match self.token {
    //                     '/' => {
    //                         // do line comment
    //                     }
    //                     '*' => {
    //                         // do block comment
    //                     }
    //                     '=' => {
    //                         self.next();
    //                         return VyToken::TkDivideEq;
    //                     }
    //                 }
    //             }
    //             '#' => {
    //                 // compiler flags
    //             }
    //             '=' => {

    //             }
    //             _ => {
    //                 // gonna have to do a bunch of stuff for hex and all that jazz.
    //                 // if self.token.is_digit {
    //                 //     return VyToken::TkInteger 
    //                 // }
    //                 // else if self.token.is_alpha {
    //                 //     return VyToken::TkIdentifier
    //                 // }
    //             }
    //         }
    //     }
    // }

    // fn next() {
    //     todo!()
    // }

    pub fn is_eof(&self) -> bool {
        return self.eof;
    }
}