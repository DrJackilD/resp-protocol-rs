use serde::de::{self, DeserializeOwned, DeserializeSeed, SeqAccess, Visitor};

use crate::error::{Error, Result};
use crate::RESPType;
use std::fmt;
use std::io::{BufRead, BufReader, Cursor};
use std::option::Option::None;

pub fn from_string<T>(s: String) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut reader = BufReader::new(Cursor::new(s));
    from_buf_reader(&mut reader)
}

pub fn from_buf_reader<R, T>(reader: &mut R) -> Result<T>
where
    R: BufRead,
    T: DeserializeOwned,
{
    let mut deserializer = Deserializer::from_buf_reader(reader);
    let result = T::deserialize(&mut deserializer)?;
    Ok(result)
}

pub struct Deserializer<'de, R: BufRead> {
    reader: &'de mut R,
}

impl<'de, R: BufRead> Deserializer<'de, R> {
    pub fn from_buf_reader(reader: &'de mut R) -> Self {
        Deserializer { reader }
    }
}

impl<'de, R: BufRead> Deserializer<'de, R> {
    fn parse_isize(&mut self) -> Result<isize> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        let buf = buf.trim_end();
        match buf.parse::<isize>() {
            Ok(size) => Ok(size),
            Err(_) => Err(Error::Syntax),
        }
    }
}

impl<'de, 'a, R: BufRead> de::Deserializer<'de> for &'a mut Deserializer<'de, R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        match buf[0] {
            b'+' => self.deserialize_str(visitor),
            b'-' => self.deserialize_string(visitor),
            b':' => self.deserialize_i64(visitor),
            b'$' => self.deserialize_byte_buf(visitor),
            b'*' => self.deserialize_seq(visitor),
            _ => Err(Error::Syntax),
        }
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        match buf.trim_end().parse::<i64>() {
            Ok(i) => visitor.visit_i64(i),
            Err(_) => Err(Error::Syntax),
        }
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // The `Serializer` implementation on the previous page serialized chars as
    // single-character strings so handle that representation here.
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse a string, check that it is one character, call `visit_char`.
        unimplemented!()
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        visitor.visit_str(buf.trim_end())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        visitor.visit_string(buf.trim_end().to_string())
    }

    // The `Serializer` implementation on the previous page serialized byte
    // arrays as JSON arrays of bytes. Handle that representation here.
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let size = self.parse_isize()?;
        if size < 0 {
            return visitor.visit_none();
        }
        let mut buf = vec![0u8; (size + 2) as usize];
        self.reader.read_exact(&mut buf)?;
        if buf.split_off(size as usize) != b"\r\n" {
            return Err(Error::Syntax);
        }
        visitor.visit_byte_buf(buf)
    }

    // An absent optional is represented as the JSON `null` and a present
    // optional is represented as just the contained value.
    //
    // As commented in `Serializer` implementation, this is a lossy
    // representation. For example the values `Some(())` and `None` both
    // serialize as just `null`. Unfortunately this is typically what people
    // expect when working with JSON. Other formats are encouraged to behave
    // more intelligently if possible.
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let size = self.parse_isize()?;
        if size < 0 {
            return visitor.visit_unit();
        }
        visitor.visit_seq(RESPArray {
            de: &mut self,
            remaining: size as usize,
        })
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently.
    //
    // As indicated by the length parameter, the `Deserialize` implementation
    // for a tuple in the Serde data model is required to know the length of the
    // tuple before even looking at the input data.
    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Structs look just like maps in JSON.
    //
    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct RESPArray<'a, 'de, R: BufRead> {
    de: &'a mut Deserializer<'de, R>,
    remaining: usize,
}

impl<'a, 'de, R: BufRead> de::SeqAccess<'de> for RESPArray<'a, 'de, R> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::result::Result<Option<<T as DeserializeSeed<'de>>::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1; // remove read element from the remaining count
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct RESPTypeVisitor;

impl<'de> de::Visitor<'de> for RESPTypeVisitor {
    type Value = RESPType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A RESP value")
    }

    fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::Integer(v))
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::SimpleString(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::Error(v))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::BulkString(Some(v)))
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::BulkString(None))
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::Array(None))
    }

    fn visit_seq<A>(
        self,
        mut seq: A,
    ) -> std::result::Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut array = Vec::with_capacity(seq.size_hint().unwrap_or_default());
        while let Some(element) = seq.next_element()? {
            array.push(element)
        }
        Ok(RESPType::Array(Some(array)))
    }
}

impl<'de> de::Deserialize<'de> for RESPType {
    fn deserialize<D>(
        deserializer: D,
    ) -> std::result::Result<Self, <D as de::Deserializer<'de>>::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(RESPTypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_simple_string() {
        let s = String::from("+Hello\r\n");
        let result: RESPType = from_string(s).unwrap();
        assert_eq!(result, RESPType::SimpleString("Hello".to_string()));
    }

    #[test]
    fn de_error() {
        let s = String::from("-UNEXPECTED\r\n");
        let result: RESPType = from_string(s).unwrap();
        assert_eq!(result, RESPType::Error("UNEXPECTED".to_string()));
    }

    #[test]
    fn de_integer() {
        let s = String::from(":23\r\n");
        let result: RESPType = from_string(s).unwrap();
        assert_eq!(result, RESPType::Integer(23));
    }

    #[test]
    fn de_null_bulk_string() {
        let s = String::from("$-1\r\n");
        let result: RESPType = from_string(s).unwrap();
        assert_eq!(result, RESPType::BulkString(None));
    }

    #[test]
    fn de_bulk_string() {
        let s = String::from("$5\r\nHello\r\n");
        let result: RESPType = from_string(s).unwrap();
        assert_eq!(
            result,
            RESPType::BulkString(Some(String::from("Hello").into_bytes()))
        );
    }

    #[test]
    fn de_null_array() {
        let s = String::from("*-1\r\n");
        let result: RESPType = from_string(s).unwrap();
        assert_eq!(result, RESPType::Array(None));
    }

    #[test]
    fn de_empty_array() {
        let s = String::from("*0\r\n");
        let result: RESPType = from_string(s).unwrap();
        assert_eq!(result, RESPType::Array(Some(Vec::new())));
    }

    #[test]
    fn de_array() {
        let s = String::from("*3\r\n:23\r\n+Hello\r\n$5\r\nthere\r\n");
        let result: RESPType = from_string(s).unwrap();
        assert_eq!(
            result,
            RESPType::Array(Some(vec![
                RESPType::Integer(23),
                RESPType::SimpleString("Hello".to_owned()),
                RESPType::BulkString(Some(String::from("there").into_bytes()))
            ]))
        );
    }
}
