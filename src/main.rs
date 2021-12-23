use crate::symbol::Symbol;
use std::iter::FromIterator;

mod symbol;

fn main() {
    let worker_count = 100;
    let handles =
        (0..worker_count).map(|_n| {
            std::thread::spawn(|| {
                for i in 1..5 {
                    Symbol::from(String::from_iter(std::iter::repeat("a").take(i)));
                }
            })
        }).collect::<Vec<_>>();

    handles.into_iter().for_each(|h| { let _ = h.join(); });

   symbol::debug_print_global_symbol_table();
}
