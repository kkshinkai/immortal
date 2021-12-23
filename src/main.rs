use crate::symbol_table::SymbolTable;

mod symbol_table;
mod symbol;

fn main() {
    let s1 = SymbolTable::of("a".to_string());
    let s2 = SymbolTable::of("b".to_string());
    let s3 = SymbolTable::of("c".to_string());
    let s4 = SymbolTable::of("a".to_string());

    println!("s1 = {}, s2 = {}, s3 = {}, s4 = {}",
             SymbolTable::get(s1), SymbolTable::get(s2), SymbolTable::get(s3), SymbolTable::get(s4));
    println!("s1.id = {}, s2.id = {}, s3.id = {}, s4.id = {}", s1.id(), s2.id(), s3.id(), s4.id());
}
