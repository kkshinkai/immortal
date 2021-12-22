#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Symbol(u32);

impl Symbol {
    pub fn new(id: u32) -> Self { Self(id) }
    pub fn id(self) -> u32 { self.0 }
}