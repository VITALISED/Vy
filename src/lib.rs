pub mod api;
pub mod types;
pub mod object;
pub mod compiler;
pub mod lex;
pub mod opcodes;
pub mod state;
pub mod vm;

pub enum VyResult {
    VyOk = 0,
    VyError = -1
}

#[macro_export]
macro_rules! vy_succeeded {
    ( $( $res:expr ),*) => {
        res>=0
    };
}

#[macro_export]
macro_rules! vy_failed {
    ( $( $res:expr ),*) => {
        res<0
    };
}