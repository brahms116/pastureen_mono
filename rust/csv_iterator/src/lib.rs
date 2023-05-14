use csv::DeserializeRecordsIntoIter;
use serde::de::DeserializeOwned;
use std::fs::File;

pub struct CsvIterator<K: DeserializeOwned> {
    reader_iter: DeserializeRecordsIntoIter<File, K>,
}

impl<K: DeserializeOwned> CsvIterator<K> {
    pub fn new(path: &str) -> CsvIterator<K> {
        let file = File::open(path).unwrap();
        let reader = csv::Reader::from_reader(file);
        let reader_iter = reader.into_deserialize::<K>();
        CsvIterator { reader_iter }
    }
}

impl<K: DeserializeOwned> Iterator for CsvIterator<K> {
    type Item = Result<K, csv::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader_iter.next()
    }
}
