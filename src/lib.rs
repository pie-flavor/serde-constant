//! This crate provides a type that can represent a single-const serde value. For example, asserting that a particular bool is always true.
//! You can use this for disambiguation in `serde(untagged)` structures, or just for validation.
//!
//! # Examples
//!
//! ```rust
//! # use serde::Deserialize;
//! # use serde_json::json;
//! use serde_constant::ConstBool;
//! #[derive(Deserialize)]
//! struct Foo {
//!     bar: String,
//!     baz: ConstBool<true>,
//! }
//!
//! assert!(serde_json::from_value::<Foo>(json!({ "bar": "quux", "baz": true })).is_ok());
//! assert!(serde_json::from_value::<Foo>(json!({ "bar": "quux", "baz": false })).is_err());
//! ```
//!
//! ```
//! # use serde::Deserialize;
//! # use serde_json::{json, from_value};
//! # use std::error::Error;
//! use serde_constant::ConstI64;
//! // int tags? No problem!
//! #[derive(Deserialize)]
//! #[serde(untagged)]
//! enum Foo {
//!     Bar {
//!         tag: ConstI64<1>,
//!     },
//!     Baz {
//!         tag: ConstI64<2>,
//!         x: Option<String>,
//!     },
//! }
//! # fn main() -> Result<(), Box<dyn Error>> {
//! assert!(matches!(
//!     serde_json::from_value(json!({ "tag": 2, "x": null }))?,
//!     // would have been Bar if `tag` were just `i64`
//!     Foo::Baz { x: None, .. },
//! ));
//! # Ok(()) }
//! ```
#![no_std]
#![allow(clippy::unnecessary_cast)]
#![warn(missing_docs)]
use core::fmt;
use serde::{
    de::{self, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

/// A const `bool`.
///
/// Deserialization fails if the value is not `V`.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct ConstBool<const V: bool>;

impl<const V: bool> Serialize for ConstBool<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(V)
    }
}

impl<'de, const V: bool> Deserialize<'de> for ConstBool<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bool(ConstBoolVisitor::<V>)
    }
}

struct ConstBoolVisitor<const V: bool>;

impl<'de, const V: bool> Visitor<'de> for ConstBoolVisitor<V> {
    type Value = ConstBool<V>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{V}")
    }
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == V {
            Ok(ConstBool::<V>)
        } else {
            Err(E::invalid_value(Unexpected::Bool(v), &self))
        }
    }
}

macro_rules! declare_int {
    ($($type:ty => $struct:ident $visitor:ident $ser_func:ident $deser_func:ident),* $(,)?) => {
        $(
            #[doc = concat!("A const `", stringify!($type), "`.")]
            ///
            /// Deserialization fails if the value is not `V`.
            #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
            pub struct $struct<const V: $type>;

            impl<const V: $type> Serialize for $struct<V> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.$ser_func(V)
                }
            }

            impl<'de, const V: $type> Deserialize<'de> for $struct<V> {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    deserializer.$deser_func($visitor::<V>)
                }
            }

            struct $visitor<const V: $type>;

            impl<'de, const V: $type> Visitor<'de> for $visitor<V> {
                type Value = $struct<V>;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "{V}")
                }
                fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    if v == V as i128 {
                        Ok($struct::<V>)
                    } else {
                        Err(E::invalid_value(Unexpected::Signed(V as i64), &self))
                    }
                }
                fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(
                        i128::try_from(v)
                            .map_err(|_| E::invalid_value(Unexpected::Unsigned(v as u64), &Self))?,
                    )
                }
            }
        )*
    };
}

declare_int!(
    i8 => ConstI8 ConstI8Visitor serialize_i8 deserialize_i8,
    i16 => ConstI16 ConstI16Visitor serialize_i16 deserialize_i16,
    i32 => ConstI32 ConstI32Visitor serialize_i32 deserialize_i32,
    i64 => ConstI64 ConstI64Visitor serialize_i64 deserialize_i64,
    i128 => ConstI128 ConstI128Visitor serialize_i128 deserialize_i128,
);

macro_rules! declare_uint {
    ($($type:ty => $struct:ident $visitor:ident $ser_func:ident $deser_func:ident),* $(,)?) => {
        $(
            #[doc = concat!("A const `", stringify!($type), "`.")]
            ///
            ///  Deserialization fails if the value is not `V`.
            #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
            pub struct $struct<const V: $type>;

            impl<const V: $type> Serialize for $struct<V> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.$ser_func(V)
                }
            }

            impl<'de, const V: $type> Deserialize<'de> for $struct<V> {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    deserializer.$deser_func($visitor::<V>)
                }
            }

            struct $visitor<const V: $type>;

            impl<'de, const V: $type> Visitor<'de> for $visitor<V> {
                type Value = $struct<V>;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "{V}")
                }
                fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_i128(v as i128)
                }
                fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    if v < 0 {
                        Err(E::invalid_value(Unexpected::Signed(v as i64), &self))
                    } else {
                        self.visit_u128(v as u128)
                    }
                }
                fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_u128(v as u128)
                }
                fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_u128(v as u128)
                }
                fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_u128(v as u128)
                }
                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    self.visit_u128(v as u128)
                }
                fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    if v == V as u128 {
                        Ok($struct::<V>)
                    } else {
                        Err(E::invalid_value(Unexpected::Unsigned(v as u64), &self))
                    }
                }
            }
        )*
    };
}

declare_uint!(
    u8 => ConstU8 ConstU8Visitor serialize_u8 deserialize_u8,
    u16 => ConstU16 ConstU16Visitor serialize_u16 deserialize_u16,
    u32 => ConstU32 ConstU32Visitor serialize_u32 deserialize_u32,
    u64 => ConstU64 ConstU64Visitor serialize_u64 deserialize_u64,
    u128 => ConstU128 ConstU128Visitor serialize_u128 deserialize_u128,
);

/// A const `char`.
///
/// Deserialization fails if the value is not `V`.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct ConstChar<const V: char>;

impl<const V: char> Serialize for ConstChar<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_char(V)
    }
}

impl<'de, const V: char> Deserialize<'de> for ConstChar<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_char(ConstCharVisitor::<V>)
    }
}

struct ConstCharVisitor<const V: char>;

impl<'de, const V: char> Visitor<'de> for ConstCharVisitor<V> {
    type Value = ConstChar<V>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "'{V}'")
    }
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == V {
            Ok(ConstChar::<V>)
        } else {
            Err(E::invalid_value(Unexpected::Char(v), &self))
        }
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut chs = v.chars();
        let ch = chs.next().ok_or_else(|| E::invalid_length(0, &self))?;
        if chs.next().is_some() {
            Err(E::invalid_length(2, &self))
        } else {
            self.visit_char(ch)
        }
    }
}
