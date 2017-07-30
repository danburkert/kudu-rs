use std::io;

use bytes::BytesMut;
use tokio_io::codec::{
    Encoder,
    Decoder,
};

struct Codec {

}

impl Encoder for Codec {
    type Item = ();
    type Error = io::Error;
    fn encode(&mut self,
              item: Self::Item,
              dst: &mut BytesMut)
              -> Result<(), io::Error> {
        unimplemented!()
    }
}

impl Decoder for Codec {
    type Item = ();
    type Error = io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        unimplemented!()
    }
}
