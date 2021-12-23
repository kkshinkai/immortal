use crate::symbol::Symbol;

mod symbol;

fn main() {
    let s1 = Symbol::from("a".to_string());
    let s2 = Symbol::from("b".to_string());
    let s3 = Symbol::from("c".to_string());
    let s4 = Symbol::from("a".to_string());

    println!("s1 = {}, s2 = {}, s3 = {}, s4 = {}", s1.to_string(), s2.to_string(), s3.to_string(), s4.to_string());
    println!("s1.id = {}, s2.id = {}, s3.id = {}, s4.id = {}", s1.id(), s2.id(), s3.id(), s4.id());
}
