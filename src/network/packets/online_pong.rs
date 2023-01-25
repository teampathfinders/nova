use bytes::{BufMut, BytesMut};

use crate::error::VexResult;
use crate::network::traits::Encodable;

#[derive(Debug)]
pub struct OnlinePong {
    pub ping_time: i64,
    pub pong_time: i64,
}

impl OnlinePong {
    pub const ID: u8 = 0x03;
}

impl Encodable for OnlinePong {
    fn encode(&self) -> VexResult<BytesMut> {
        let mut buffer = BytesMut::with_capacity(1 + 8 + 8);

        buffer.put_u8(Self::ID);
        buffer.put_i64(self.ping_time);
        buffer.put_i64(self.pong_time);

        Ok(buffer)
    }
}