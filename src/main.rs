use crate::symbol_table::SymbolTable;

mod symbol_table;
mod symbol;

fn main() {
    let mut symbol_table = SymbolTable::new();

    let s1 = symbol_table.intern("a".to_string());
    let s2 = symbol_table.intern("b".to_string());
    let s3 = symbol_table.intern("c".to_string());
    let s4 = symbol_table.intern("a".to_string());

    println!("s1 = {}, s2 = {}, s3 = {}, s4 = {}", s1.id(), s2.id(), s3.id(), s4.id());
}
