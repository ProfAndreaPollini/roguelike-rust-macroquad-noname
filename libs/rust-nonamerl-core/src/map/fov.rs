use std::collections::HashSet;

use crate::{IntVector2, Map, Tile, Vec2};

// pub struct MapFovIter<'a, T: Tile> {
//     map: &'a Map<T>,
//     center: IntVector2,
//     radius: i32,
//     points: Vec<IntVector2>,
// }

// impl<'a, T: Tile> MapFovIter<'a, T> {
//     pub fn new(map: &'a Map<T>, center: IntVector2, radius: i32) -> Self {
//         let mut targets = Vec::new();

//         for i in -radius..=radius {
//             for j in -radius..=radius {
//                 let v = IntVector2::new(i, j);
//                 // if v.() <= radius as f32 {
//                 targets.push(v);
//                 // }
//             }
//         }

//         let mut points = HashSet::new();
//         for target in targets.iter() {
//             let path = map.line(center, *target);
//             for point in path {
//                 points.insert(point);
//             }
//         }
//         // let path = self.map.line(
//         //     IntVector2::new(self.center.x(), self.center.y()),
//         //     IntVector2::new(current_target.x(), current_target.y()),
//         // );

//         Self {
//             map,
//             center,
//             radius,
//             points: points.into_iter().collect(),
//         }
//     }
// }

// impl<'a, T: Tile> Iterator for MapFovIter<'a, T> {
//     type Item = (IntVector2, &'a T);

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut next = None;

//         if let Some(target) = self.targets.pop() {
//             let current_target = target;
//         }

//         next
//     }
// }
