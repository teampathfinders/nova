use std::net::{IpAddr, SocketAddr};

use bytes::{Buf, BufMut, BytesMut};

use crate::error::VexResult;
use crate::instance::{IPV4_LOCAL_ADDR, IPV6_LOCAL_ADDR};
use crate::raknet::{Frame, FrameBatch, Header, OrderChannel, Reliability};
use crate::raknet::packets::{Decodable, NewIncomingConnection, OnlinePing};
use crate::util::{ReadExtensions, WriteExtensions};

#[test]
fn header_encoding_decoding() {
    // let mut encoded = BytesMut::new();
    // Header {
    //     id: 0xc1,
    //     target_subclient: 205,
    //     sender_subclient: 45
    // }.encode(&mut encoded);

    // let mut encoded = encoded.clone();
    let mut encoded = BytesMut::from([0xc1, 0xe9, 0x33].as_ref());
    let decoded = Header::decode(&mut encoded).unwrap();

    // assert_eq!(decoded, Header {
    //     id: 0xc1,
    //     target_subclient: 205,
    //     sender_subclient: 45
    // });

    let mut encoded = BytesMut::new();
    decoded.encode(&mut encoded);

    assert_eq!(encoded.as_ref(), &[0xc1, 0xe9, 0x33]);
}

#[test]
fn read_write_var_u32() {
    let mut buffer = BytesMut::new();
    buffer.put_var_u32(45);
    buffer.put_var_u32(2769);
    buffer.put_var_u32(105356);
    buffer.put_var_u32(359745976);

    let mut buffer = buffer.freeze();
    assert_eq!(buffer.get_var_u32().unwrap(), 45);
    assert_eq!(buffer.get_var_u32().unwrap(), 2769);
    assert_eq!(buffer.get_var_u32().unwrap(), 105356);
    assert_eq!(buffer.get_var_u32().unwrap(), 359745976);

    let mut buffer = BytesMut::from([0xc1, 0xe9, 0x33].as_ref());
    let a = buffer.get_var_u32().unwrap();

    let mut buffer2 = BytesMut::new();
    buffer2.put_var_u32(a);

    assert_eq!(&[0xc1, 0xe9, 0x33], buffer2.as_ref());
}

#[test]
fn read_write_u24_le() {
    let mut buffer = BytesMut::new();
    buffer.put_u24_le(125); // Test first byte only
    buffer.put_u24_le(50250); // Test first two bytes
    buffer.put_u24_le(1097359); // Test all bytes

    let mut buffer = buffer.freeze();
    assert_eq!(buffer.get_u24_le(), 125);
    assert_eq!(buffer.get_u24_le(), 50250);
    assert_eq!(buffer.get_u24_le(), 1097359);
}

#[test]
fn read_write_addr() -> VexResult<()> {
    let ipv4_test = SocketAddr::new(IpAddr::V4(IPV4_LOCAL_ADDR), 19132);
    let ipv6_test = SocketAddr::new(IpAddr::V6(IPV6_LOCAL_ADDR), 19133);

    let mut buffer = BytesMut::new();
    buffer.put_addr(ipv4_test); // Test IPv4
    buffer.put_addr(ipv6_test); // Test IPv6

    let mut buffer = buffer.freeze();
    assert_eq!(buffer.get_addr()?, ipv4_test);
    assert_eq!(buffer.get_addr()?, ipv6_test);
    Ok(())
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