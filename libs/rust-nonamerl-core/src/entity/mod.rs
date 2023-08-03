pub mod action;
pub mod activator;
pub mod entity;
pub mod item;
pub mod property;
pub mod world;

pub trait WithId<K: slotmap::Key, V> {
    fn id(&self) -> K;
    fn create_with_id(id: K, name: &str) -> V;
}
