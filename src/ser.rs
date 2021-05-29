use serde::{ser, Serialize};

use crate::{Error, RESPType, Result};
use serde::ser::SerializeSeq;
use std::io::Write;
use std::result;

/// Serialize given value to string
pub fn to_string(value: RESPType) -> Result<String> {
    let mut buf: Vec<u8> = Vec::new();
    let mut serializer = Serializer { writer: &mut buf };
    value.serialize(&mut serializer)?;
    Ok(String::from_utf8(buf)?)
}

pub struct Serializer<W: Write> {
    writer: W,
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        unimplemented!()
    }

    /// RESPType::Integer
    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.writer.write_all(b":")?;
        self.writer.write_all(format!("{}", v).as_bytes())?;
        self.writer.write_all(b"\r\n")?;
        Ok(())
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.serialize_str(&v.to_string())
    }

    /// RESPType::SimpleString and RESPType::Error
    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.writer.write_all(v.as_bytes())?;
        self.writer.write_all(b"\r\n")?;
        Ok(())
    }

    /// RESPType::BulkString
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.writer.write_all(b"$")?;
        self.writer
            .write_all(format!("{}\r\n", v.len() as u64).as_bytes())?;
        self.writer.write_all(v)?;
        self.writer.write_all(b"\r\n")?;
        Ok(())
    }

    /// RESPType::Array(None)
    fn serialize_none(self) -> Result<Self::Ok> {
        self.writer.write_all(b"*-1\r\n")?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    /// RESPType::BulkString(None)
    fn serialize_unit(self) -> Result<Self::Ok> {
        self.writer.write_all(b"$-1\r\n")?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    /// RESPType::Array
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(l) => self.writer.write_all(format!("*{}\r\n", l).as_bytes())?,
            None => unimplemented!(),
        };
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeSeq for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeTupleVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeMap for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeTuple for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl serde::Serialize for RESPType {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RESPType::SimpleString(string) => serializer.serialize_str(&("+".to_owned() + string)),
            RESPType::Error(string) => serializer.serialize_str(&("-".to_owned() + string)),
            RESPType::Integer(int) => serializer.serialize_i64(*int),
            RESPType::BulkString(value) => match value {
                Some(seq) => serializer.serialize_bytes(seq),
                None => serializer.serialize_unit(),
            },
            RESPType::Array(value) => match value {
                Some(array) => {
                    let mut es = serializer.serialize_seq(Some(array.len()))?;
                    for v in array {
                        es.serialize_element(v)?;
                    }
                    es.end()
                }
                None => serializer.serialize_none(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser_simple_string() {
        let simple_str = RESPType::SimpleString("Hello".to_owned());
        assert_eq!("+Hello\r\n", to_string(simple_str).unwrap());
    }

    #[test]
    fn ser_error() {
        let error = RESPType::Error("SERIALIZATION_ERROR".to_owned());
        assert_eq!("-SERIALIZATION_ERROR\r\n", to_string(error).unwrap());
    }

    #[test]
    fn ser_integer() {
        let integer = RESPType::Integer(25);
        assert_eq!(":25\r\n", to_string(integer).unwrap());
    }

    #[test]
    fn ser_empty_bulk_string() {
        let empty_string = RESPType::BulkString(None);
        assert_eq!("$-1\r\n", to_string(empty_string).unwrap());
    }

    #[test]
    fn ser_bulk_string() {
        let bulk_string = RESPType::BulkString(Some("TEST".to_owned().into_bytes()));
        assert_eq!("$4\r\nTEST\r\n", to_string(bulk_string).unwrap());
    }

    #[test]
    fn ser_null_array() {
        let null_array = RESPType::Array(None);
        assert_eq!("*-1\r\n", to_string(null_array).unwrap());
    }

    #[test]
    fn ser_empty_array() {
        let empty_array = RESPType::Array(Some(vec![]));
        assert_eq!("*0\r\n", to_string(empty_array).unwrap());
    }

    #[test]
    fn ser_array() {
        let array = RESPType::Array(Some(vec![
            RESPType::BulkString(Some("Hello".to_owned().into())),
            RESPType::Array(Some(vec![RESPType::Integer(2)])),
            RESPType::SimpleString("Hello".to_owned()),
        ]));
        assert_eq!(
            "*3\r\n$5\r\nHello\r\n*1\r\n:2\r\n+Hello\r\n",
            to_string(array).unwrap()
        );
    }
}
