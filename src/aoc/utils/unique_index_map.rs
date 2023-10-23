use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Default)]
pub struct UniqueIndexMap<T> {
    inner: HashMap<T, u32>,
}

impl<T: Hash + Eq + PartialEq> UniqueIndexMap<T> {
    fn next_id(&self) -> u32 {
        self.inner.len() as u32
    }

    pub fn obtain_id(&mut self, value: T) -> u32 {
        let next_id = self.next_id();
        self.inner.entry(value).or_insert(next_id);

        next_id
    }

    pub fn find_by_id(&self, id: u32) -> Option<&T> {
        for (k, v) in self.inner.iter() {
            if *v != id {
                continue;
            }

            return Some(k);
        }

        None
    }
}
