use bincode;
use bincode::rustc_serialize;
use bincode::rustc_serialize::{EncodingResult, DecodingResult};

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Message {
    num: u64,
    body: String,
}

impl Message {
    pub fn new(n: u16, s: &str) -> Message {
        Message { num: n, body: s.to_string() }
    }

    pub fn encode(msg: Message) -> EncodingResult<Vec<u8>> {
        rustc_serialize::encode(&msg, bincode::SizeLimit::Infinite)
    }

    pub fn decode(bytes: &[u8]) -> DecodingResult<Message> {
        rustc_serialize::decode(bytes)
    }
}
