//! AMQP 0-9-1 types definition and implementation.
//!
//! See [RabbitMQ's Definition](https://github.com/rabbitmq/rabbitmq-codegen/blob/main/amqp-rabbitmq-0.9.1.json).
//!
//! See [RabbitMQ errata](https://www.rabbitmq.com/amqp-0-9-1-errata.html)
use std::{
    collections::HashMap,
    fmt::{self, Debug},
    num::TryFromIntError,
};

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// DO NOT USE. No primitive rust type to represent single bit.
///
/// Bits are packed in octect according to AMQP 0-9-1 protocol.
pub type Bit = u8;

pub type Octect = u8;
pub type Boolean = bool; // 0 = FALSE, else TRUE
pub type ShortShortUint = u8;
pub type ShortShortInt = i8;
pub type ShortUint = u16;
pub type ShortInt = i16;
pub type LongUint = u32;
pub type LongInt = i32;
pub type LongLongUint = u64;
pub type LongLongInt = i64;
pub type TimeStamp = u64;
pub type Float = f32;
pub type Double = f64;

/////////////////////////////////////////////////////////////////////////////
/// AMQP short string type.
///
/// User should not directly create it, but use conversion method to create
/// from `String` or `&str`.
///
/// # Usage
///
/// ```
/// # use amqp_serde::types::ShortStr;
/// // create a ShortStr from &str
/// let s: ShortStr = "hello".try_into().unwrap();
///
/// // create a ShortStr from String
/// let s: ShortStr = String::from("hello").try_into().unwrap();
///
/// ```
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct ShortStr(u8, String);

impl fmt::Display for ShortStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
impl Default for ShortStr {
    fn default() -> Self {
        Self(0, "".to_string())
    }
}
impl From<ShortStr> for String {
    fn from(s: ShortStr) -> Self {
        s.1
    }
}
impl AsRef<String> for ShortStr {
    fn as_ref(&self) -> &String {
        &self.1
    }
}

impl TryFrom<String> for ShortStr {
    type Error = TryFromIntError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let len = u8::try_from(s.len())?;
        Ok(Self(len, s))
    }
}
impl TryFrom<&str> for ShortStr {
    type Error = TryFromIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.to_string().try_into()
    }
}

/////////////////////////////////////////////////////////////////////////////
/// AMQP long string type.
///
/// User should not directly create it, but use conversion method to create
/// from `String` or `&str`.
///
/// # Usage
///
/// ```
/// # use amqp_serde::types::LongStr;
/// // create a LongStr from `&str`
/// let s: LongStr = "hello".try_into().unwrap();
/// // create a LongStr from `String`
/// let s: LongStr = String::from("hello").try_into().unwrap();
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LongStr(u32, String);

impl fmt::Display for LongStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
impl Default for LongStr {
    fn default() -> Self {
        Self(0, "".to_string())
    }
}

impl AsRef<String> for LongStr {
    fn as_ref(&self) -> &String {
        &self.1
    }
}
impl TryFrom<String> for LongStr {
    type Error = TryFromIntError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let len = u32::try_from(s.len())?;
        Ok(Self(len, s))
    }
}
impl TryFrom<&str> for LongStr {
    type Error = TryFromIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.to_string().try_into()
    }
}
impl From<LongStr> for String {
    fn from(s: LongStr) -> Self {
        s.1
    }
}

/////////////////////////////////////////////////////////////////////////////
/// AMQP decimal type.
///
/// decimal-value = "scale long-int".
///
/// RabbitMQ treat the decimal value as signed integer.
/// See notes "Decimals encoding" in [amqp-0-9-1-errata](https://www.rabbitmq.com/amqp-0-9-1-errata.html).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DecimalValue(Octect, LongInt);

impl DecimalValue {
    pub fn new(scale: Octect, value: LongInt) -> Self {
        Self(scale, value)
    }
}

impl fmt::Display for DecimalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Decimal({}, {})", self.0, self.1)
    }
}

/////////////////////////////////////////////////////////////////////////////
/// AMQP byte array type.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ByteArray(LongUint, Vec<u8>);
impl TryFrom<Vec<u8>> for ByteArray {
    type Error = TryFromIntError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let len = LongUint::try_from(bytes.len())?;
        Ok(Self(len, bytes))
    }
}
impl From<ByteArray> for Vec<u8> {
    fn from(arr: ByteArray) -> Self {
        arr.1
    }
}
impl fmt::Display for ByteArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.1)
    }
}
/////////////////////////////////////////////////////////////////////////////

/// AMQP field array type.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct FieldArray(LongUint, Vec<FieldValue>); // RabbitMQ use LongUint as length value

impl FieldArray {
    pub fn new() -> Self {
        Self(0, Vec::with_capacity(0))
    }
}

impl TryFrom<Vec<FieldValue>> for FieldArray {
    type Error = TryFromIntError;

    fn try_from(values: Vec<FieldValue>) -> Result<Self, Self::Error> {
        let len = LongUint::try_from(values.len())?;
        Ok(Self(len, values))
    }
}
impl From<FieldArray> for Vec<FieldValue> {
    fn from(arr: FieldArray) -> Self {
        arr.1
    }
}

impl fmt::Display for FieldArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        let len = self.1.len();
        for v in self.1.iter().take(len - 1) {
            write!(f, "{}, ", v)?;
        }
        if let Some(v) = self.1.last() {
            write!(f, "{} ", v)?;
        }

        write!(f, "]")?;
        Ok(())
    }
}
//////////////////////////////////////////////////////////////////////////////
/// AMQP field value type.
///
/// User is recommended to use conversion method to create FieldValue from rust's type.
///
/// See [RabbitMQ errata](https://www.rabbitmq.com/amqp-0-9-1-errata.html#section_3).
///
/// # Usage
///
/// ```
/// # use amqp_serde::types::FieldValue;
/// // convert from `bool`
/// let x: FieldValue = true.into();
///
/// // convert into `bool`
/// let y: bool = x.try_into().unwrap();
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum FieldValue {
    t(Boolean),
    b(ShortShortInt),
    B(ShortShortUint),
    // U(ShortInt),     // not exist in RabbitMQ
    s(ShortInt), // used in RabbitMQ equivalent to 'U' in 0-9-1 spec
    u(ShortUint),
    I(LongInt),
    i(LongUint),
    // L(LongLongInt),  // not exist in RabbitMQ
    l(LongLongInt), // RabbitMQ is signed, 0-9-1 spec is unsigned
    f(Float),
    d(Double),
    D(DecimalValue),
    // s(ShortStr),     // not exist in RabbitMQ
    S(LongStr),
    A(FieldArray),
    T(TimeStamp),
    F(FieldTable),
    V,
    x(ByteArray), // RabbitMQ only
}

impl From<bool> for FieldValue {
    fn from(v: bool) -> Self {
        FieldValue::t(v)
    }
}
impl TryInto<bool> for FieldValue {
    type Error = crate::Error;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            FieldValue::t(v) => Ok(v),
            _ => Err(crate::Error::Message("not a bool".to_string())),
        }
    }
}
impl From<FieldTable> for FieldValue {
    fn from(v: FieldTable) -> Self {
        FieldValue::F(v)
    }
}
impl TryInto<FieldTable> for FieldValue {
    type Error = crate::Error;

    fn try_into(self) -> Result<FieldTable, Self::Error> {
        match self {
            FieldValue::F(v) => Ok(v),
            _ => Err(crate::Error::Message("not a FieldTable".to_string())),
        }
    }
}

impl From<LongStr> for FieldValue {
    fn from(v: LongStr) -> Self {
        FieldValue::S(v)
    }
}

impl TryInto<LongStr> for FieldValue {
    type Error = crate::Error;

    fn try_into(self) -> Result<LongStr, Self::Error> {
        match self {
            FieldValue::S(v) => Ok(v),
            _ => Err(crate::Error::Message("not a LongStr".to_string())),
        }
    }
}

/// RabbitMQ's field value support only long string variant, so rust string type
/// always converted to long string variant.
impl From<String> for FieldValue {
    fn from(v: String) -> Self {
        FieldValue::S(v.try_into().unwrap())
    }
}

impl TryInto<String> for FieldValue {
    type Error = crate::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            FieldValue::S(v) => Ok(v.into()),
            _ => Err(crate::Error::Message("not a LongStr".to_string())),
        }
    }
}

impl From<&str> for FieldValue {
    fn from(v: &str) -> Self {
        FieldValue::S(v.try_into().unwrap())
    }
}

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldValue::t(v) => write!(f, "{}", v),
            FieldValue::b(v) => write!(f, "{}", v),
            FieldValue::B(v) => write!(f, "{}", v),
            FieldValue::s(v) => write!(f, "{}", v),
            FieldValue::u(v) => write!(f, "{}", v),
            FieldValue::I(v) => write!(f, "{}", v),
            FieldValue::i(v) => write!(f, "{}", v),
            FieldValue::l(v) => write!(f, "{}", v),
            FieldValue::f(v) => write!(f, "{}", v),
            FieldValue::d(v) => write!(f, "{}", v),
            FieldValue::D(v) => write!(f, "{}", v),
            FieldValue::S(v) => write!(f, "{}", v),
            FieldValue::A(v) => write!(f, "{}", v),
            FieldValue::T(v) => write!(f, "{}", v),
            FieldValue::F(v) => write!(f, "{}", v),
            FieldValue::V => write!(f, "()"),
            FieldValue::x(v) => write!(f, "{}", v),
        }
    }
}
//////////////////////////////////////////////////////////////////////////////

pub type FieldName = ShortStr;
/// AMQP field table type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct FieldTable(HashMap<FieldName, FieldValue>);

impl FieldTable {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn insert(&mut self, k: FieldName, v: FieldValue) -> Option<FieldValue> {
        self.0.insert(k, v)
    }

    pub fn remove(&mut self, k: &FieldName) -> Option<FieldValue> {
        self.0.remove(k)
    }

    pub fn get(&self, k: &FieldName) -> Option<&FieldValue> {
        self.0.get(k)
    }
}
impl fmt::Display for FieldTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for (k, v) in self.0.iter().take(self.0.len() - 1) {
            write!(f, "{}: {}, ", k, v)?;
        }
        if let Some((k, v)) = self.0.iter().last() {
            write!(f, "{}: {} ", k, v)?;
        }

        write!(f, "}}")?;
        Ok(())
    }
}

/////////////////////////////////////////////////////////////////////////////
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub struct FieldTable(HashMap<FieldName, FieldValue>);
// impl Default for FieldTable {
//     fn default() -> Self {
//         Self(HashMap::new())
//     }
// }

// impl Deref for FieldTable {
//     type Target = HashMap<FieldName, FieldValue>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl FieldTable {
//     pub fn new() -> Self {
//         Self(HashMap::new())
//     }

//     pub fn insert<V>(&mut self, k: String, v: V)
//     where
//         V: Any,
//     {
//         let k: FieldName = k.try_into().unwrap();
//         let v = &v as &dyn Any;

//         if v.is::<bool>() {
//             let v = v.downcast_ref::<bool>().unwrap();
//             let old = self.0.insert(k, FieldValue::t(v.clone()));
//         } else if v.is::<i8>() {
//             let v = v.downcast_ref::<i8>().unwrap();
//             let old = self.0.insert(k, FieldValue::b(v.clone()));
//         } else if v.is::<u8>() {
//             let v = v.downcast_ref::<u8>().unwrap();
//             let old = self.0.insert(k, FieldValue::B(v.clone()));
//         } else if v.is::<i16>() {
//             let v = v.downcast_ref::<bool>().unwrap();
//             let old = self.0.insert(k, FieldValue::t(v.clone()));
//         } else if v.is::<u16>() {
//         } else if v.is::<i32>() {
//         } else if v.is::<u32>() {
//         } else if v.is::<i64>() {
//         } else if v.is::<f32>() {
//         } else if v.is::<f64>() {
//         } else if v.is::<DecimalValue>() {
//         } else if v.is::<String>() {
//             // RabbitMQ does not have "short string" type in field value,
//             let v = v.downcast_ref::<String>().unwrap();
//             let old = self
//                 .0
//                 .insert(k, FieldValue::S(v.clone().try_into().unwrap()));
//         } else if v.is::<FieldArray>() {
//         } else if v.is::<u64>() { // RabbitMQ do not have "Unsigned 64-bit" field value, so `u64` can be uniquely mapped to TimeStamp
//         } else if v.is::<Self>() {
//         } else if v.is::<()>() {
//         } else if v.is::<ByteArray>() {
//         } else {
//             panic!("unsupported value type {:?} ", v);
//         }

//     }
// }

/////////////////////////////////////////////////////////////////////////////
// AMQP domains
/// Note: it is different from definition in [`RabbitMQ Definition`].
///
/// In [`RabbitMQ Definition`], it is defined as `longstr`, and only used in `open-ok` frame.
///
/// Here, it is defined as [`ShortUint`], which is the type of channel id field in AMQP frame.
/// It is intended to be used in place where readability can be improved.
///
/// [`RabbitMQ Definition`]: https://github.com/rabbitmq/rabbitmq-codegen/blob/main/amqp-rabbitmq-0.9.1.json
pub type AmqpChannelId = ShortUint;

pub type AmqpClassId = ShortUint;
pub type AmqpMethodId = ShortUint;
pub type AmqpConsumerTag = ShortStr;
pub type AmqpDeliveryTag = LongLongUint;
pub type AmqpDestination = ShortStr;
pub type AmqpDuration = LongLongUint;
pub type AmqpExchangeName = ShortStr;
pub type AmqpMessageCount = LongUint;
pub type AmqpOffset = LongUint;
pub type AmqpPath = ShortStr;
pub type AmqpPeerProperties = FieldTable;
pub type AmqpQueueName = ShortStr;
pub type AmqpReference = LongStr;
pub type AmqpRejectCode = ShortUint;
pub type AmqpRejectText = ShortStr;
pub type AmqpReplyCode = ShortUint;
pub type AmqpReplyText = ShortStr;
pub type AmqpSecurityToken = LongStr;
pub type AmqpTable = FieldTable;
pub type AmqpTimeStamp = TimeStamp;

/////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::types::{DecimalValue, FieldArray, FieldValue};

    use super::FieldTable;
    #[test]
    fn test_field_table_display() {
        let mut table = FieldTable::new();
        table.insert(
            "Cash".try_into().unwrap(),
            FieldValue::D(DecimalValue(3, 123456)),
        );

        assert_eq!("{ Cash: Decimal(3, 123456) }", format!("{}", table));
    }
    #[test]
    fn test_field_array_display() {
        let field_arr = FieldArray(
            2,
            vec![FieldValue::t(true), FieldValue::D(DecimalValue(3, 123456))],
        );
        assert_eq!("[ true, Decimal(3, 123456) ]", format!("{}", field_arr));
    }
}
