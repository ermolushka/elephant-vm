use std::collections::HashMap;

use crate::value::{ObjType, Value};

#[derive(Debug, Clone)]
pub struct Table {
    pub entries: HashMap<ObjType, Value>,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub key: ObjType,
    pub value: Value,
}

// I will not implement HashMap from scratch
// as it doesn't make much sense and it's not the goal here
impl Table {
    pub fn init_table() -> Table {
        Table {
            entries: HashMap::new(),
        }
    }

    pub fn free_table(&mut self) {
        self.entries.clear();
    }

    pub fn table_set(&mut self, key: ObjType, value: Value) -> bool {
        let is_new_key = !self.entries.contains_key(&key);
        self.entries.insert(key, value);
        is_new_key
    }

    pub fn table_get(&self, key: &ObjType) -> Option<Value> {
        self.entries.get(key).cloned()
    }

    pub fn table_delete(&mut self, key: &ObjType) -> bool {
        self.entries.remove(key).is_some()
    }

    pub fn table_add_all(&mut self, from: &Table) {
        self.entries.extend(from.entries.clone());
    }
}
