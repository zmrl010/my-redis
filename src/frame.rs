use bytes::Bytes;
use std::io::Cursor;

#[derive(Clone, Debug)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}

#[derive(Debug)]
pub enum Error {
    /// Not enough data is available to parse a message
    Incomplete,

    Other(crate::Error),
}

impl Frame {
    /// Returns empty array
    pub(crate) fn array() -> Frame {
        Frame::Array(vec![])
    }

    /// Push a "bulk" frame onto the array `self` must be an Array frame.
    pub(crate) fn push_bulk(&mut self, bytes: Bytes) {
        match self {
            Frame::Array(vec) => {
                vec.push(Frame::Bulk(bytes));
            }
            _ => panic!("not an array frame"),
        }
    }

    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        todo!()
    }
}
