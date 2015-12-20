use std::fmt;
use std::str;

use bincode;
use bincode::rustc_serialize;
use bincode::rustc_serialize::{EncodingResult, DecodingResult};

#[derive(Clone, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Message {
    num: u32,
    body: Vec<u8>,
}

impl Message {
    pub fn new<'a>(n: u32, s: &str) -> Message {
        Message { num: n, body: s.as_bytes().to_vec() }
    }

    pub fn encode(msg: &Message) -> EncodingResult<Vec<u8>> {
        rustc_serialize::encode(msg, bincode::SizeLimit::Infinite)
    }

    pub fn decode(bytes: &[u8]) -> DecodingResult<Message> {
        rustc_serialize::decode(bytes)
    }
}
impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let txt = str::from_utf8(&self.body).unwrap();
        write!(f, "Message: {{{}, {}}}", self.num, txt)
    }
}
