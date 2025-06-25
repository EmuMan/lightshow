use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SimpleHandle<T> {
    index: u32,
    generation: u32,
    _marker: PhantomData<T>,
}

impl<T> Clone for SimpleHandle<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            generation: self.generation,
            _marker: PhantomData,
        }
    }
}

impl<T> Copy for SimpleHandle<T> {}

impl<T> SimpleHandle<T> {
    fn new(index: u32, generation: u32) -> Self {
        SimpleHandle {
            index,
            generation,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct SimpleStoreEntry<T> {
    item: Option<T>,
    current_generation: u32,
}

impl<T> Default for SimpleStoreEntry<T> {
    fn default() -> Self {
        SimpleStoreEntry {
            item: None,
            current_generation: 0,
        }
    }
}

impl<T> SimpleStoreEntry<T> {
    fn new(item: T) -> Self {
        SimpleStoreEntry {
            item: Some(item),
            current_generation: 0,
        }
    }
}

#[derive(Resource, Debug)]
pub struct SimpleStore<T> {
    pub entries: Vec<SimpleStoreEntry<T>>,
    open_indices: Vec<usize>,
}

impl<T> Default for SimpleStore<T> {
    fn default() -> Self {
        SimpleStore {
            entries: Vec::new(),
            open_indices: Vec::new(),
        }
    }
}

impl<T> SimpleStore<T> {
    fn get_open_entry_mut(&mut self) -> Option<(usize, &mut SimpleStoreEntry<T>)> {
        self.open_indices.pop().and_then(|entry_index| {
            self.entries
                .get_mut(entry_index)
                .map(|entry| (entry_index, entry))
        })
    }

    pub fn add(&mut self, item: T) -> SimpleHandle<T> {
        if let Some((entry_index, open_entry)) = self.get_open_entry_mut() {
            if open_entry.item.is_some() {
                panic!("SimpleStore attempted to override existing layer.")
            }
            open_entry.item = Some(item);
            open_entry.current_generation += 1;
            SimpleHandle::new(entry_index as u32, open_entry.current_generation)
        } else {
            self.entries.push(SimpleStoreEntry::new(item));
            SimpleHandle::new(self.entries.len() as u32 - 1, 0)
        }
    }

    pub fn get(&self, handle: SimpleHandle<T>) -> Option<&T> {
        let entry = self.entries.get(handle.index as usize)?;
        if handle.generation == entry.current_generation {
            entry.item.as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, handle: SimpleHandle<T>) -> Option<&mut T> {
        let entry = self.entries.get_mut(handle.index as usize)?;
        if handle.generation == entry.current_generation {
            entry.item.as_mut()
        } else {
            None
        }
    }

    pub fn remove(&mut self, handle: SimpleHandle<T>) -> Result<(), &'static str> {
        let entry = self
            .entries
            .get_mut(handle.index as usize)
            .ok_or("entry not found at index")?;
        if handle.generation != entry.current_generation {
            Err("entry generation does not match")
        } else {
            entry.item = None;
            self.open_indices.push(handle.index as usize);
            Ok(())
        }
    }
}
