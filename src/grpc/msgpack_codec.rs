use bytes::{Buf, BufMut};
use std::marker::PhantomData;
use tonic::{
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
    Status,
};

#[derive(Debug)]
pub struct MsgPackEncoder<T>(PhantomData<T>);

impl<T: serde::Serialize> Encoder for MsgPackEncoder<T> {
    type Error = Status;
    type Item = T;

    fn encode(
        &mut self,
        item: Self::Item,
        buf: &mut EncodeBuf<'_>,
    ) -> Result<(), Self::Error> {
        rmp_serde::encode::write(&mut buf.writer(), &item)
            .map_err(|e| Status::internal(e.to_string()))
    }
}

#[derive(Debug)]
pub struct MsgPackDecoder<U>(PhantomData<U>);

impl<U: serde::de::DeserializeOwned> Decoder for MsgPackDecoder<U> {
    type Error = Status;
    type Item = U;

    fn decode(
        &mut self,
        buf: &mut DecodeBuf<'_>,
    ) -> Result<Option<Self::Item>, Self::Error> {
        if !buf.has_remaining() {
            return Ok(None);
        }
        // does not allocate
        let bytes = buf.copy_to_bytes(buf.remaining());
        let item: Self::Item = rmp_serde::decode::from_slice(&bytes)
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Some(item))
    }
}

/// A [`Codec`] that implements `application/grpc+msgpack` via the serde library.
#[derive(Debug, Clone)]
pub struct MsgPackCodec<T, U>(PhantomData<(T, U)>);

impl<T, U> Default for MsgPackCodec<T, U> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T, U> Codec for MsgPackCodec<T, U>
where
    T: serde::Serialize + Send + 'static,
    U: serde::de::DeserializeOwned + Send + 'static,
{
    type Decode = U;
    type Decoder = MsgPackDecoder<U>;
    type Encode = T;
    type Encoder = MsgPackEncoder<T>;

    fn encoder(&mut self) -> Self::Encoder {
        MsgPackEncoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        MsgPackDecoder(PhantomData)
    }
}
