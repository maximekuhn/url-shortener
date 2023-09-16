pub mod in_memory;

pub trait DBHelper: Send + Sync {
    fn print(&self);

    fn increment(&mut self);
}