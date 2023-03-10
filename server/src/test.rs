use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr};

use base64::Engine;
use bytes::{Buf, BufMut, BytesMut};
use flate2::read::DeflateDecoder;
use tokio::net::windows::named_pipe::PipeMode::Byte;

use crate::instance_manager::{IPV4_LOCAL_ADDR, IPV6_LOCAL_ADDR};
use crate::network::raknet::{Frame, OrderChannel};
use crate::network::Header;
use common::{ReadExtensions, WriteExtensions};
use common::{Serialize, VResult};

#[test]
fn read_write_header() {
    let header = Header {
        id: 129,
        sender_subclient: 3,
        target_subclient: 2,
    };

    let mut buffer = BytesMut::new();
    header.serialize(&mut buffer);

    assert_eq!(Header::deserialize(&mut buffer.freeze()).unwrap(), header);
}

#[test]
fn order_channel() {
    let mut test_frame = Frame::default();
    let mut channel = OrderChannel::new();

    test_frame.order_index = 0;
    assert!(channel.insert(test_frame.clone()).is_some());

    test_frame.order_index = 2;
    assert!(channel.insert(test_frame.clone()).is_none());

    test_frame.order_index = 1;
    let output = channel.insert(test_frame).unwrap();

    assert_eq!(output.len(), 2);
    assert_eq!(output[0].order_index, 1);
    assert_eq!(output[1].order_index, 2);
}
