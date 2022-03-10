use crate::types::VyChar;

pub struct VyString {
    next: *mut VyString,
    val: VyChar
}
