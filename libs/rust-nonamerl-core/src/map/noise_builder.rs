use crate::{MapBuilder, MapBuilderAlgorithm, Tile};

use noise::NoiseFn;

#[derive(Debug, Copy, Clone)]
pub struct BuilderAlgoWithNoise<T, N, F>
where
    T: Tile,
    N: NoiseFn<f64, 2>,
    F: Fn(i32, i32, f64) -> Option<T>,
{
    noise: N,
    f: F,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Tile, N: NoiseFn<f64, 2>, F: Fn(i32, i32, f64) -> Option<T>> BuilderAlgoWithNoise<T, N, F> {
    pub fn new(noise_fn: N, f: F) -> Self {
        Self {
            noise: noise_fn,
            f,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Tile, N: NoiseFn<f64, 2>, F: Fn(i32, i32, f64) -> Option<T>> MapBuilderAlgorithm<T>
    for BuilderAlgoWithNoise<T, N, F>
{
    fn build<'a>(&self, map_builder: &'a mut MapBuilder<T>) -> &'a mut MapBuilder<T> {
        for x in map_builder.map.extent.left()..map_builder.map.extent.right() {
            for y in map_builder.map.extent.top()..map_builder.map.extent.bottom() {
                let value = self.noise.get([x as f64 * 5., y as f64 * 5.]);
                let tile = (self.f)(x, y, value);

                if let Some(tile) = tile {
                    map_builder.map.set(x, y, tile);
                }
            }
        }

        map_builder
    }
}

#[cfg(test)]

mod tests {

    use noise::{Fbm, Perlin};

    use crate::{
        map, Dimension2D, FovOccluder, IntExtent2D, ItemContainer, Visible, Visited, Walkable,
    };

    use super::*;

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct TestTile {}
    impl Tile for TestTile {}
    impl Visible for TestTile {}
    impl Visited for TestTile {}
    impl FovOccluder for TestTile {}
    impl Walkable for TestTile {}
    impl ItemContainer for TestTile {}

    #[test]
    fn test_map_builder() {
        let mut map_builder =
            MapBuilder::<TestTile>::new(IntExtent2D::new(0, 0, 10, 10), Dimension2D::new(24, 24));

        map_builder.add_tile("grass", TestTile::default());
        map_builder.add_tile("water", TestTile::default());

        let noise = Fbm::<Perlin>::default();
        let f = |x: i32, y: i32, value: f64| {
            if value > 0.0 {
                Some(TestTile::default())
            } else {
                None
            }
        };

        map_builder.add_step(&BuilderAlgoWithNoise::new(noise, f));
        let map = map_builder.map;

        assert_eq!(map.len(), 53);
    }
}
