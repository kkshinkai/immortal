use std::collections::HashMap;
use crate::symbol::Symbol;
use std::sync::Once;

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

static mut SYMBOL_TABLE: Option<SymbolTable> = None;
static ONCE: Once = Once::new();

impl SymbolTable {
    pub fn default() -> &'static mut SymbolTable {
        unsafe {
            ONCE.call_once(|| {
                SYMBOL_TABLE = Some(SymbolTable::new());
            });
            SYMBOL_TABLE.as_mut().unwrap()
        }
    }

    pub fn of(contents: String) -> Symbol {
        Self::default().intern(contents)
    }

    pub fn get(symbol: Symbol) -> &'static str {
        Self::default().store[symbol.id() as usize].as_str()
    }
}