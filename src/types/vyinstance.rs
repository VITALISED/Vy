use crate::object::VyObject;

use super::VyBoolean;

pub struct VyInstanceBaseObj {
    locked: VyBoolean,
    base: VyObject,
}

pub struct VyInstance;