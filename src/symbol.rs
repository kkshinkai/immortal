use std::collections::HashMap;
use std::sync::{Once, Mutex};
use std::fmt::Debug;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Symbol(u32);

impl Symbol {
    fn new(id: u32) -> Symbol { Symbol(id) }
    pub fn id(self) -> u32 { self.0 }

    pub fn from(string: String) -> Symbol {
        SymbolTable::intern(string)
    }
}

struct SymbolTable {
    table: HashMap<&'static String, Symbol>,
    store: Vec<String>,
}

static mut SYMBOL_TABLE: Option<Mutex<SymbolTable>> = None;
static SYMBOL_TABLE_ONCE: Once = Once::new();

impl SymbolTable {
    const DEFAULT_CAPACITY: usize = 4096;

    fn new() -> Self {
        Self {
            table: HashMap::with_capacity(Self::DEFAULT_CAPACITY),
            store: Vec::with_capacity(Self::DEFAULT_CAPACITY),
        }
    }

    fn global_symbol_table() -> &'static mut Mutex<SymbolTable> {
        unsafe {
            SYMBOL_TABLE_ONCE.call_once(|| {
                SYMBOL_TABLE = Some(Mutex::new(SymbolTable::new()));
            });
            SYMBOL_TABLE.as_mut().unwrap()
        }
    }

    fn intern(contents: String) -> Symbol {
        let mut symbol_table = Self::global_symbol_table().lock().unwrap();
        if let Some(&id) = symbol_table.table.get(&contents) {
            return id
        }

        let id = Symbol::new(symbol_table.table.len() as u32);

        symbol_table.store.push(contents);

        let str_ref = unsafe {
            let ptr: *const String = symbol_table.store.last().unwrap();
            &*ptr
        } as &'static String;

        symbol_table.table.insert(str_ref, id);
        id
    }
}

impl Debug for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol_list =
            self.store.iter().enumerate()
                .map(|(i, s)| format!("#{} = \"{}\"", i, s))
                .collect::<Vec<_>>()
                .join("\n");
        write!(f, "Symbol Table:\n{}\n", symbol_list)
    }
}
