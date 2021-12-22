use std::collections::HashMap;
use crate::symbol::Symbol;

pub struct SymbolTable {
    table: HashMap<&'static String, Symbol>,
    store: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self { Self::with_capacity(4096) }

    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two();
        Self {
            table: HashMap::with_capacity(capacity),
            store: Vec::with_capacity(capacity),
        }
    }

    pub fn intern(&mut self, contents: String) -> Symbol {
        if let Some(&id) = self.table.get(&contents) {
            return id
        }

        let id = Symbol::new(self.table.len() as u32);

        self.table.insert(unsafe {
            let ptr: *const String = &contents;
            &*ptr
        } as &'static _, id);
        self.store.push(contents);
        id
    }
}