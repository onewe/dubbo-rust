use std::{marker::PhantomData, pin::Pin};

use async_stream::stream;
use bytes::Bytes;
use futures::Stream;
use prost::DecodeError;

use crate::StdError;

pub trait Serializable {
       
    fn serialize(&self) -> Result<Box<dyn Stream<Item = Bytes> + Send>, StdError>;
}


pub trait Deserializable {

    type Target;
    
    fn deserialize(&self, data: Box<dyn Stream<Item = Bytes> + Send + Unpin>) -> Result<Box<dyn Stream<Item = Result<Self::Target, StdError>>>, StdError>;
}



pub struct SerdeJsonSerialization<T> {
    data: T,
}

impl<T> SerdeJsonSerialization<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
        }
    }
}

impl<T> Serializable for SerdeJsonSerialization<T>
where
    T: serde::Serialize,
{
    fn serialize(&self) -> Result<Box<dyn Stream<Item = Bytes> + Send>, StdError> {
        let vec = serde_json::to_vec(&self.data)?;
        Ok(Box::new(futures::stream::once(async move { Bytes::from(vec) }))
            as Box<dyn Stream<Item = Bytes> + Send>)
    }
}


pub struct SerdeJsonDeserialization<T> {
    convert: fn(&[u8]) -> Result<T, serde_json::Error>,
    _phantom: PhantomData<T>,
}

impl<T> SerdeJsonDeserialization<T> 
where
    T: for<'a> serde::Deserialize<'a>,
{
    pub fn new() -> Self {
        
        let convert = |data: &[u8]| serde_json::from_slice::<T>(data);

        SerdeJsonDeserialization {
            convert,
            _phantom: PhantomData,
        }
    }
}


impl<T> Deserializable for SerdeJsonDeserialization<T>
where
    T: for<'a> serde::Deserialize<'a>,
    T: 'static,
{
    type Target = T;

    fn deserialize(&self, items: Box<dyn Stream<Item = Bytes> + Send + Unpin>) -> Result<Box<dyn Stream<Item = Result<Self::Target, StdError>>>, StdError> {

        let convert = self.convert;
        let items = Pin::new(items);
        let stream = stream! {
            for await value in items {
                let value = convert(value.as_ref());
                let value = match value {
                    Ok(value) => {
                        Ok(value)
                    },
                    Err(e) => {
                      
                        Err(StdError::from(e))
                    }
                };
                yield value;
                
            }
        };
        Ok(Box::new(stream) as Box<dyn Stream<Item = Result<T, StdError>>>)
    }
}


pub struct ProstSerialization<T> {
    data: T,
}


impl<T> ProstSerialization<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
        }
    }
}


impl<T> Serializable for ProstSerialization<T>
where
    T: prost::Message,
{
    fn serialize(&self) -> Result<Box<dyn Stream<Item = Bytes> + Send>, StdError> {
        let buf = self.data.encode_to_vec();
        Ok(Box::new(futures::stream::once(async move { Bytes::from(buf) }))
            as Box<dyn Stream<Item = Bytes> + Send>)
    }
}


pub struct ProstDeserialization<T> {
    convert: fn(&[u8]) -> Result<T, DecodeError>,
    _phantom: PhantomData<T>,
}


impl<T> ProstDeserialization<T> 
where
    T: prost::Message + Default,
{
    pub fn new() -> Self {
        
        let convert = |data: &[u8]| T::decode(data);

        ProstDeserialization {
            convert,
            _phantom: PhantomData,
        }
    }
}


impl<T> Deserializable for ProstDeserialization<T>
where
    T: prost::Message + Default,
    T: 'static,
{

    type Target = T;

    fn deserialize(&self, items: Box<dyn Stream<Item = Bytes> + Send + Unpin>) -> Result<Box<dyn Stream<Item = Result<Self::Target, StdError>>>, StdError> {
        let convert = self.convert;
        let items = Pin::new(items);
        let stream = stream! {
            for await value in items {
                let value = convert(value.as_ref());
                let value = match value {
                    Ok(value) => {
                        Ok(value)
                    },
                    Err(e) => {
                        Err(StdError::from(e))
                    }
                };
                yield value;
            }
        };
        Ok(Box::new(stream) as Box<dyn Stream<Item = Result<T, StdError>>>)
    }
}