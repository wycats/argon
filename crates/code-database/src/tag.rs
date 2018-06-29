pub trait Tag {
    fn revision(&self) -> u64;
    fn is_valid(&self, snapshot: u64) -> bool;
}

pub struct Tagged<I, T: Tag> {
    item: I,
    tag: T,
}
