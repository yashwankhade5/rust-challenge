

use std::marker::PhantomData;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};



pub trait Serializer<T> {
     type Error: std::fmt::Debug + std::error::Error;
    fn to_bytes(&self, value: &T) -> Result<Vec<u8>, Self::Error>;
    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Self::Error>;
}
#[derive(Debug, PartialEq)]
pub struct Borsh;

#[derive(Debug, PartialEq)]
pub struct Wincode;

#[derive(Debug, PartialEq)]
pub struct Json;

impl<T> Serializer<T> for Borsh
where
    T: BorshSerialize + BorshDeserialize,
{
    type Error = std::io::Error;

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Self::Error> {
        borsh::to_vec(data).map_err(|e| e.into())
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        borsh::from_slice(bytes).map_err(|e| e.into())
    }
}

impl<T> Serializer<T> for Json
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    type Error = serde_json::Error;

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(data).map_err(|e| e.into())
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        serde_json::from_slice(bytes).map_err(|e| e.into())
    }
}



pub struct Storage<T, S: Serializer<T>> {
    serializer: S,
    data: Option<Vec<u8>>,
    _marker: PhantomData<T>,
}

impl<T, S: Serializer<T>> Storage<T, S> {
    pub fn new(serializer: S) -> Self {
        Self {
            serializer,
            data: None,
            _marker: PhantomData,
        }
    }

    pub fn save(&mut self, value: &T) -> Result<(), Box<dyn std::error::Error>> {
   let bytes = self.serializer.to_bytes(value).unwrap();
        self.data = Some(bytes);
        Ok(())
    }

    pub fn load(&self) -> Result<T, Box<dyn std::error::Error>> {
         if let Some(bytes) = &self.data {
            return Ok(self.serializer.from_bytes(bytes).unwrap());
        }

        Err("error".into())
    }


    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        self.data.as_deref()
    }
    
    }

    #[derive(
    Debug,
    PartialEq,
    BorshSerialize,
    BorshDeserialize,
     Serialize,
    Deserialize,

)]
pub struct DataStruct {
    pub data: String,
  
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borsh() {
        let person = DataStruct {
            data: "shirnath".to_string(),    
        };

        let mut storage = Storage::new(Borsh);
        storage.save(&person).unwrap();

        assert!(storage.has_data());
        assert_eq!(storage.load().unwrap(), person);
    }
 #[test]
    fn test_json() {
        let person = DataStruct {
            data:"apaar".to_string()
        };

        let mut storage = Storage::new(Json);
        storage.save(&person).unwrap();

        assert!(storage.has_data());
        assert_eq!(storage.load().unwrap(), person);
    }
}