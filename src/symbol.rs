use std::collections::HashMap;
use std::sync::{Once, Mutex};
use std::fmt::Debug;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Symbol(u32);

impl Symbol {
    fn new(id: u32) -> Self { Self(id) }
    pub fn id(self) -> u32 { self.0 }

    pub fn from(string: String) -> Symbol {
        SymbolTable::intern(string)
    }
}

pub struct SymbolTable {
    table: HashMap<&'static String, Symbol>,
    store: Vec<String>,
}

impl Debug for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.store.iter().enumerate()
            .map(|(i, s)| format!("#{} = \"{}\"", i, s))
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

impl SymbolTable {
    fn intern(contents: String) -> Symbol {
        let mut symbol_table = Self::default().lock().unwrap();
        if let Some(&id) = symbol_table.table.get(&contents) {
            return id
        }

        let id = Symbol::new(symbol_table.table.len() as u32);

        symbol_table.store.push(contents);

        let str_ref = unsafe {
            let ptr: *const String = symbol_table.store.last().unwrap();
            &*ptr
        } as &'static _;
        symbol_table.table.insert(str_ref, id);

        id
    }
}

pub /* Just for debug use */ static mut SYMBOL_TABLE: Option<Mutex<SymbolTable>> = None;
static ONCE: Once = Once::new();

impl SymbolTable {
    const DEFAULT_SIZE: usize = 4096;

    fn new() -> Self {
        let capacity = Self::DEFAULT_SIZE.next_power_of_two();
        Self {
            table: HashMap::with_capacity(capacity),
            store: Vec::with_capacity(capacity),
        }
    }

    pub fn default() -> &'static mut Mutex<SymbolTable> {
        unsafe {
            ONCE.call_once(|| {
                SYMBOL_TABLE = Some(Mutex::new(SymbolTable::new()));
            });
            SYMBOL_TABLE.as_mut().unwrap()
        }
    }
}