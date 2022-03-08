use crate::types::VyObjectType;

// remember to actually figure out what memory model to do this in since lifetimes everywhere is just not gonna work
pub struct VyObject<'a> {
    val: VyObjectType<'a>,
}
