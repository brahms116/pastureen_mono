use csv::DeserializeRecordsIntoIter;
use serde::de::DeserializeOwned;
use std::fs::File;

#[derive(Debug)]
pub enum CsvIteratorError {
    FileError(String),
    CsvDeserializeError(String),
}

impl From<std::io::Error> for CsvIteratorError {
    fn from(error: std::io::Error) -> Self {
        CsvIteratorError::FileError(format!("Failed to open specified file: {}", error))
    }
}

impl From<csv::Error> for CsvIteratorError {
    fn from(error: csv::Error) -> Self {
        CsvIteratorError::CsvDeserializeError(format!("Failed to deserialize csv: {}", error))
    }
}

impl std::fmt::Display for CsvIteratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CsvIteratorError::FileError(msg) => write!(f, "{}", msg),
            CsvIteratorError::CsvDeserializeError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for CsvIteratorError {}

pub struct CsvIterator<K: DeserializeOwned> {
    reader_iter: DeserializeRecordsIntoIter<File, K>,
}

impl<K: DeserializeOwned> CsvIterator<K> {
    pub fn new(path: &str) -> Result<CsvIterator<K>, CsvIteratorError> {
        let file = File::open(path)?;
        let reader = csv::Reader::from_reader(file);
        let reader_iter = reader.into_deserialize::<K>();
        Ok(CsvIterator { reader_iter })
    }
}

impl<K: DeserializeOwned> Iterator for CsvIterator<K> {
    type Item = Result<K, CsvIteratorError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader_iter.next().map(|res| res.map_err(|e| e.into()))
    }
}
