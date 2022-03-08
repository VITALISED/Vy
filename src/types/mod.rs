pub mod vyarray;
pub mod vyclass;
pub mod vyclosure;
pub mod vyenum;
pub mod vyinstance;
pub mod vystring;
pub mod vytable;

use crate::{VyResult};
use self::{
    vyarray::VyArray,
    vyclass::VyClass,
    vyclosure::VyClosure,
    vyenum::VyEnum,
    vyinstance::VyInstance,
    vystring::VyString,
    vytable::VyTable,
};

pub type VyByte = u8;
pub type VyInteger = i64;
pub type VyFloat = f32;
pub type VyDouble = f64;
pub type VyBoolean = bool;

pub enum VyNumeric {
    Byte(VyByte),
    Integer(VyInteger),
    Float(VyFloat),
    Double(VyDouble),
}

pub type VyChar = char;
pub type VyRange<T> = (T, T);

pub type VyVoid = ();

pub type VyUserFunction<T> = fn(T) -> VyResult;

pub enum VyObjectType<'a> {
    OTByte(VyByte),
    OTInteger(VyInteger),
    OTFloat(VyFloat),
    OTDouble(VyDouble),
    OTBoolean(VyBoolean),
    OTString(VyString),
    OTChar(VyChar),
    OTRange(VyRange<VyNumeric>),
    OTVoid(VyVoid),
    OTClosure(VyClosure),
    OTArray(VyArray<'a>),
    OTTable(VyTable),
    OTClass(VyClass),
    OTInstance(VyInstance),
    OTEnum(VyEnum),
}