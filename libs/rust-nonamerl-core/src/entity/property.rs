#![allow(dead_code)]
use std::collections::HashMap;

use crate::IntVector2;

use super::world::ItemKey;

#[derive(Debug, Clone, Copy)]
pub struct HealthData {
    pub health: i32,
}

pub trait PropertyValue {
    type ValueType;
    fn value(&self) -> Value<Self::ValueType>;
}

#[derive(Debug, Clone)]
pub enum Property {
    Health(HealthData),
    Xp(i32),
    Name(String),
    Equip(ItemKey),
    Position(IntVector2),
    Gold(i32),
}

impl Property {
    pub const XP: &'static str = "xp";
    pub const HEALTH: &'static str = "health";
    pub const NAME: &'static str = "name";
    pub const EQUIP: &'static str = "equip";
    pub const POSITION: &'static str = "position";
    pub const GOLD: &'static str = "gold";

    pub fn name(&self) -> &'static str {
        match self {
            Property::Health(_) => Property::HEALTH,
            Property::Xp(_) => Property::XP,
            Property::Name(_) => Property::NAME,
            Property::Equip(_) => Property::EQUIP,
            Property::Position(_) => Property::POSITION,
            Property::Gold(_) => Property::GOLD,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Value<T>(T);

impl PropertyValue for HealthData {
    type ValueType = i32;

    fn value(&self) -> Value<Self::ValueType> {
        let HealthData { health } = self;
        Value(*health)
    }
}

// pub trait ValueProperty {
//     type ValueType;
//     // const NAME: &'static str;
//     fn value(&self) -> &Self::ValueType;
//     // fn name(&self) -> &'static str {
//     //     Self::NAME
//     // }
// }

// trait PropertyContainer {
//     fn get_property<T>(&self, name: &str) -> Option<&dyn ValueProperty<ValueType = T>>;

//     fn add_property(&mut self, property: Property);
// }

// impl PropertyContainer for Vec<&dyn ValueProperty> {
//     fn get_property<T>(&self, name: &str) -> Option<&dyn ValueProperty<ValueType = T>> {
//         self.iter()
//             .find(|p| p.name() == name)
//             .map(|p| p as &dyn ValueProperty<ValueType = T>)
//     }

//     fn add_property(&mut self, property: Property) {
//         self.push(property);
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Position {
//     pub position: Value<IntVector2>,
// }

// #[derive(Debug, Clone)]
// pub struct Value<T>(T);

// impl<T> ValueProperty for Value<T> {
//     type ValueType = T;

//     fn value(&self) -> &Self::ValueType {
//         &self.0
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Name {
//     pub name: Value<String>,
// }

// #[cfg(test)]

// mod tests {

//     #[test]
//     fn test_value_property() {
//         use super::*;

//         let position = Value(IntVector2::new(1, 2));

//         assert_eq!(*position.value(), IntVector2::new(1, 2));
//     }

//     #[test]
//     fn test_two_property_vector() {
//         use super::*;

//         let position = Value(IntVector2::new(1, 2));

//         let name = Value("test".to_string());
//         //create vector of box implementing ValueProperty trait
//         // let properties: Vec<Box<dyn ValueProperty>> = vec![Box::new(position), Box::new(name)];

//         // assert_eq!(properties.len(), 2); ??????????????????????????????????????????????????????????
//     }
// }
