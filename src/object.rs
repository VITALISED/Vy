pub type VyByte = u8;
pub type VyInteger = i64;
pub type VyFloat = f32;
pub type VyDouble = f64;
pub type VyBoolean = bool;
//pub type VyString();
pub type VyChar = char;
pub type VyRange = (i64, i64);

pub type VyVoid;

pub struct VyClosure;
pub struct VyArray;
pub struct VyTable;
pub struct VyClass;
pub struct VyInstance;
pub struct VyEnum;

pub enum VyObjectTypeTag {
    OTByte,
    OTInteger,
    OTFloat,
    OTDouble,
    OTBoolean,
    //OTString,
    OTChar,
    OTRange,
    OTVoid,
    OTClosure,
    OTArray,
    OTTable,
    OTClass,
    OTInstance,
    OTEnum,
}

pub struct VyObject<T, Z: VyObjectTypeTag> {
    val: T,
    ty: Z,
}


