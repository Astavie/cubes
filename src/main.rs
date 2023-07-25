#![feature(test)]

extern crate test;

use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

type Coord = (u8, u8, u8);

#[derive(Debug, Clone)]
struct Model {
    max: Coord,
    data: Vec<Coord>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            max: (0, 0, 0),
            data: vec![(0, 0, 0)],
        }
    }
}

impl Model {
    fn translate(&self, (xo, yo, zo): (i8, i8, i8)) -> Self {
        Model {
            max: (
                self.max.0.wrapping_add_signed(xo),
                self.max.1.wrapping_add_signed(yo),
                self.max.2.wrapping_add_signed(zo),
            ),
            data: self
                .data
                .iter()
                .map(|(x, y, z)| {
                    (
                        x.wrapping_add_signed(xo),
                        y.wrapping_add_signed(yo),
                        z.wrapping_add_signed(zo),
                    )
                })
                .collect(),
        }
    }
    fn add_cube(&self, (x, y, z): Coord, (xo, yo, zo): (i8, i8, i8)) -> Option<Self> {
        if x == 0 && xo < 0 || y == 0 && yo < 0 || z == 0 && zo < 0 {
            let mut model = self.translate((-xo, -yo, -zo));
            return match model.data.contains(&(x, y, z)) {
                false => {
                    model.data.push((x, y, z));
                    Some(model)
                }
                true => None,
            };
        }

        let coord = (
            x.wrapping_add_signed(xo),
            y.wrapping_add_signed(yo),
            z.wrapping_add_signed(zo),
        );

        let mut data = self.data.clone();
        match data.contains(&coord) {
            false => {
                data.push(coord);
                Some(Model {
                    max: (
                        u8::max(self.max.0, coord.0),
                        u8::max(self.max.1, coord.1),
                        u8::max(self.max.2, coord.2),
                    ),
                    data,
                })
            }
            true => None,
        }
    }
    fn add_cubes(&self, set: &mut HashMap<u64, Model>) {
        let insert = |s: &mut HashMap<u64, Model>, o: Option<Model>| match o {
            Some(m) => {
                s.insert(m.hash(), m);
            }
            None => {}
        };

        for &cube in self.data.iter() {
            insert(set, self.add_cube(cube, (1, 0, 0)));
            insert(set, self.add_cube(cube, (-1, 0, 0)));
            insert(set, self.add_cube(cube, (0, 1, 0)));
            insert(set, self.add_cube(cube, (0, -1, 0)));
            insert(set, self.add_cube(cube, (0, 0, 1)));
            insert(set, self.add_cube(cube, (0, 0, -1)));
        }
    }
    #[rustfmt::skip]
    fn hash(&self) -> u64 {
        let mut hash = u64::MAX;

        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.0, p.1, p.2))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.0, self.max.2 - p.2, p.1))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.0, self.max.1 - p.1, self.max.2 - p.2))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.0, p.2, self.max.1 - p.1))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.0 - p.0, p.1, self.max.2 - p.2))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.0 - p.0, p.2, p.1))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.0 - p.0, self.max.1 - p.1, p.2))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.0 - p.0, self.max.2 - p.2, self.max.1 - p.1))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.1, p.0, self.max.2 - p.2))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.1, p.2, p.0))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.1, self.max.0 - p.0, p.2))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.1, self.max.2 - p.2, self.max.0 - p.0))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.1 - p.1, p.0, p.2))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.1 - p.1, self.max.2 - p.2, p.0))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.1 - p.1, self.max.0 - p.0, self.max.2 - p.2))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.1 - p.1, p.2, self.max.0 - p.0))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.2, p.1, self.max.0 - p.0))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.2, p.0, p.1))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.2, self.max.1 - p.1, p.0))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (p.2, self.max.0 - p.0, self.max.1 - p.1))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.2 - p.2, p.1, p.0))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.2 - p.2, self.max.0 - p.0, p.1))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.2 - p.2, self.max.1 - p.1, self.max.0 - p.0))));
        hash = hash.min(hash_coords(self.data.iter().map(|p| (self.max.2 - p.2, p.0, self.max.1 - p.1))));

        hash
    }
}

fn hash_coords(coords: impl Iterator<Item = Coord>) -> u64 {
    let mut hash = 0;
    for coord in coords {
        hash ^= {
            let mut hash = DefaultHasher::new();
            coord.hash(&mut hash);
            hash.finish()
        };
    }
    hash
}

fn next(prev: &HashMap<u64, Model>) -> HashMap<u64, Model> {
    let mut set = HashMap::new();
    if prev.is_empty() {
        set.insert(Model::default().hash(), Model::default());
    }
    for model in prev.values() {
        model.add_cubes(&mut set);
    }
    set
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[bench]
    fn seven(b: &mut Bencher) {
        b.iter(|| {
            let mut map = HashMap::new();
            for _ in 0..7 {
                map = next(&map);
            }
            assert!(map.len() == 1023);
        });
    }
}

fn main() {
    let mut map = HashMap::new();
    for n in 0.. {
        // println!("\n{:?}", map);
        println!("{}: {}", n, map.len().max(1));
        map = next(&map);
    }
}
