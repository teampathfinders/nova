use crate::{RefTag, Value, TAG_BYTE, TAG_END};
use bytes::{BufMut, BytesMut};

pub fn serialize_be(name: &str, value: &Value, stream: &mut BytesMut) {
    Value::serialize_tag_be(stream, (name, value))
}

impl RefTag<'_> {
    /// Writes the NBT structure into the provided stream (big endian).
    pub fn serialize_be(&self, stream: &mut BytesMut) {
        Value::serialize_tag_be(stream, (self.name, self.value))
    }
}

impl Value {
    fn serialize_tag_be(stream: &mut BytesMut, tag: (&str, &Self)) {
        stream.put_u8(tag.1.as_numeric_id());
        if matches!(tag.1, Self::End) {
            return;
        }

        Self::serialize_name_be(stream, tag.0);
        Self::serialize_value_be(stream, tag.1);
    }

    fn serialize_name_be(stream: &mut BytesMut, string: &str) {
        stream.put_u16(string.len() as u16);
        stream.put(string.as_bytes());
    }

    fn serialize_value_be(stream: &mut BytesMut, value: &Self) {
        match value {
            Self::End => (),
            Self::Byte(v) => stream.put_i8(*v),
            Self::Short(v) => stream.put_i16(*v),
            Self::Int(v) => stream.put_i32(*v),
            Self::Long(v) => stream.put_i64(*v),
            Self::Float(v) => stream.put_f32(*v),
            Self::Double(v) => stream.put_f64(*v),
            Self::String(v) => Self::serialize_name_be(stream, v),
            Self::List(v) => {
                stream.put_u8(
                    v.get(0).map(|t| t.as_numeric_id()).unwrap_or(TAG_BYTE),
                );
                for t in v {
                    Self::serialize_value_be(stream, t);
                }
            }
            Self::Compound(v) => {
                for t in v.iter() {
                    Self::serialize_tag_be(stream, (t.0, t.1)); // Tuple is like this to force &String to convert to &str.
                }
                stream.put_u8(TAG_END);
            }
            Self::ByteArray(v) => {
                stream.put_i32(v.len() as i32);
                for t in v {
                    stream.put_i8(*t);
                }
            }
            Self::IntArray(v) => {
                stream.put_i32(v.len() as i32);
                for t in v {
                    stream.put_i32(*t);
                }
            }
            Self::LongArray(v) => {
                stream.put_i32(v.len() as i32);
                for t in v {
                    stream.put_i64(*t);
                }
            }
        }
    }
}
