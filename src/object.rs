use crate::types::VyObjectType;

struct _VyObject {
    val: VyObjectType,
}

// remember to actually figure out what memory model to do this in since lifetimes everywhere is just not gonna work
pub type VyObject = Box<_VyObject>;

