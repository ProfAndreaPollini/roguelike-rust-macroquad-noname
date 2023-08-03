#![allow(dead_code)]
use crate::{
    entity::world::{EntityKey, World},
    property::{self, Property},
    IntVector2, Vec2,
};

use super::Action;
#[derive(Debug)]
pub struct MoveAction {
    pub dx: IntVector2,
    pub target: EntityKey,
}

impl MoveAction {
    pub fn new(dx: IntVector2, target: EntityKey) -> Self {
        Self { dx, target }
    }
}

impl Action for MoveAction {
    fn perform(&self, world: &World) {
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
            *pos.x_mut() = pos.x() + self.dx.x();
            *pos.y_mut() = pos.y() + self.dx.y();
        }
        // let position = property.downcast_ref::<property::Position>().unwrap();

        // if let Some(Property::Position(pos)) =
        //     entities.unwrap().get_property_mut(Property::POSITION)
        // {
        //     *pos.x_mut() = pos.x() + self.dx.x();
        // }
    }
}
