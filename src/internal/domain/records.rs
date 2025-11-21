use std;

use super::filter_key::FilterKey;
use super::record::Record;

#[derive(thiserror::Error, Debug)]
pub enum RecordsError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

#[derive(Clone)]
pub struct Records {
    data: Vec<Record>,
    filter_key: FilterKey,
}

impl Default for Records {
    fn default() -> Self {
        Self::new()
    }
}

impl Records {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            filter_key: FilterKey::default(),
        }
    }

    pub fn add(&mut self, val: Record) {
        self.data.push(val);
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn len_filtered(&self) -> usize {
        self.data
            .iter()
            .filter(|&r| r.to_string().contains(&self.filter_key.to_string()))
            .count()
    }

    pub fn filter_key(&self) -> &FilterKey {
        &self.filter_key
    }
    pub fn set_filter_key(&mut self, key: FilterKey) {
        self.filter_key = key;
    }

    pub fn iter(&self) -> RecordsIterator<'_> {
        RecordsIterator {
            data: &self.data,
            filter_key: &self.filter_key,
            index: 0,
        }
    }
}

pub struct RecordsIterator<'a> {
    data: &'a Vec<Record>,
    filter_key: &'a FilterKey,
    index: usize,
}
impl<'a> Iterator for RecordsIterator<'a> {
    type Item = &'a Record;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.data.is_empty() {
                return None;
            }

            if self.index == self.data.len() {
                return None;
            }

            let result = &self.data[self.index];

            self.index += 1;

            if self.filter_key.as_ref() == "" {
                return Some(result);
            }

            if result
                .as_ref()
                .to_lowercase()
                .contains(&self.filter_key.as_ref().to_lowercase())
            {
                return Some(result);
            }
        }
    }
}
