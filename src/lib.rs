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