use crate::{RefTag, Value, TAG_BYTE, TAG_END};
use bytes::{BufMut, BytesMut};

pub fn serialize_le(name: &str, value: &Value, stream: &mut BytesMut) {
    Value::serialize_tag_le(stream, (name, value))
}

impl RefTag<'_> {
    /// Writes the NBT data into the given stream (little endian).
    pub fn serialize_le(&self, stream: &mut BytesMut) {
        Value::serialize_tag_le(stream, (self.name, self.value))
    }
}

impl Value {
    /// Encodes a name-value combo (little endian).
    fn serialize_tag_le(stream: &mut BytesMut, tag: (&str, &Self)) {
        stream.put_u8(tag.1.as_numeric_id());
        if matches!(tag.1, Self::End) {
            return;
        }

        Self::serialize_name_le(stream, tag.0);
        Self::serialize_value_le(stream, tag.1);
    }

    /// Encodes an NBT tag name (little endian).
    fn serialize_name_le(stream: &mut BytesMut, string: &str) {
        stream.put_u16_le(string.len() as u16);
        stream.put(string.as_bytes());
    }

    /// Encodes an NBT tag value (little endian).
    fn serialize_value_le(stream: &mut BytesMut, value: &Self) {
        match value {
            Self::End => (),
            Self::Byte(v) => stream.put_i8(*v),
            Self::Short(v) => stream.put_i16_le(*v),
            Self::Int(v) => stream.put_i32_le(*v),
            Self::Long(v) => stream.put_i64_le(*v),
            Self::Float(v) => stream.put_f32_le(*v),
            Self::Double(v) => stream.put_f64_le(*v),
            Self::String(v) => Self::serialize_name_le(stream, v),
            Self::List(v) => {
                stream.put_u8(
                    v.get(0).map(|t| t.as_numeric_id()).unwrap_or(TAG_BYTE),
                );
                for t in v {
                    Self::serialize_value_le(stream, t);
                }
            }
            Self::Compound(v) => {
                for t in v.iter() {
                    Self::serialize_tag_le(stream, (t.0, t.1)); // Tuple is like this to force &String to convert to &str.
                }
                stream.put_u8(TAG_END);
            }
            Self::ByteArray(v) => {
                stream.put_i32_le(v.len() as i32);
                for t in v {
                    stream.put_i8(*t);
                }
            }
            Self::IntArray(v) => {
                stream.put_i32_le(v.len() as i32);
                for t in v {
                    stream.put_i32_le(*t);
                }
            }
            Self::LongArray(v) => {
                stream.put_i32_le(v.len() as i32);
                for t in v {
                    stream.put_i64_le(*t);
                }
            }
        }
    }
}
