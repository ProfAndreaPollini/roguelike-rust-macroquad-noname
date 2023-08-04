#![allow(dead_code)]
use crate::{
    entity::world::{EntityKey, World},
    property::{self, Property},
    IntVector2, Map, Tile, Vec2,
};

use super::Action;
#[derive(Debug)]
pub struct MoveAction<T: Tile> {
    pub dx: IntVector2,
    pub target: EntityKey,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Tile> MoveAction<T> {
    pub fn new(dx: IntVector2, target: EntityKey) -> Self {
        Self {
            dx,
            target,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Tile> Action<T> for MoveAction<T> {
    fn perform(&self, world: &World<T>, map: &mut Map<T>) {
        if self.target == Default::default() {
            panic!("Target not set");
        }
        let mut entities = world.entities.borrow_mut();
        let target = entities.get_mut(self.target);
        let target = target.unwrap();
        let property = target.get_property_mut(Property::POSITION);

        if property.is_none() {
            panic!("Target does not have position property");
        }

        if let Some(Property::Position(pos)) = property {
            let desired_pos = IntVector2::new(pos.x() + self.dx.x(), pos.y() + self.dx.y());

            if let Some(tile) = map.get(desired_pos.x(), desired_pos.y()) {
                if tile.is_walkable() {
                    *pos.x_mut() = pos.x() + self.dx.x();
                    *pos.y_mut() = pos.y() + self.dx.y();
                }
            }

            // *pos.x_mut() = pos.x() + self.dx.x();
            // *pos.y_mut() = pos.y() + self.dx.y();
        }
        // let position = property.downcast_ref::<property::Position>().unwrap();

        // if let Some(Property::Position(pos)) =
        //     entities.unwrap().get_property_mut(Property::POSITION)
        // {
        //     *pos.x_mut() = pos.x() + self.dx.x();
        // }
    }
}
