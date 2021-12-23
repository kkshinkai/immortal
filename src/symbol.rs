use std::collections::HashMap;
use std::sync::Once;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Symbol(u32);

impl Symbol {
    fn new(id: u32) -> Self { Self(id) }
    pub fn id(self) -> u32 { self.0 }

    pub fn from(string: String) -> Symbol {
        SymbolTable::default().intern(string)
    }

    pub fn to_string(self) -> &'static str {
        SymbolTable::default().store[self.id() as usize].as_str()
    }
}

struct SymbolTable {
    table: HashMap<&'static String, Symbol>,
    store: Vec<String>,
}

impl SymbolTable {
    fn new() -> Self { Self::with_capacity(4096) }

    fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two();
        Self {
            table: HashMap::with_capacity(capacity),
            store: Vec::with_capacity(capacity),
        }
    }

    fn intern(&mut self, contents: String) -> Symbol {
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
    fn default() -> &'static mut SymbolTable {
        unsafe {
            ONCE.call_once(|| {
                SYMBOL_TABLE = Some(SymbolTable::new());
            });
            SYMBOL_TABLE.as_mut().unwrap()
        }
    }
}