#![feature(test)]

extern crate test;

use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    fs::{self, File},
    hash::{Hash, Hasher},
    io::{BufReader, BufWriter, Read, Write},
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
    fn add_cubes(&self, set: &mut HashSet<u64>, file: &mut impl Write) {
        let mut insert = |o: Option<Model>| match o {
            Some(m) => {
                if set.insert(m.hash()) {
                    for coord in m.data {
                        file.write(&[coord.0, coord.1, coord.2]).unwrap();
                    }
                }
            }
            None => {}
        };

        for &cube in self.data.iter() {
            insert(self.add_cube(cube, (1, 0, 0)));
            insert(self.add_cube(cube, (-1, 0, 0)));
            insert(self.add_cube(cube, (0, 1, 0)));
            insert(self.add_cube(cube, (0, -1, 0)));
            insert(self.add_cube(cube, (0, 0, 1)));
            insert(self.add_cube(cube, (0, 0, -1)));
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

fn next(prev: impl Iterator<Item = Model>, n: u64) -> (u64, String) {
    let path = "model".to_string() + &n.to_string();
    let file = File::create(path.clone()).unwrap();
    let mut file = BufWriter::new(file);

    let mut set = HashSet::new();
    for model in prev {
        model.add_cubes(&mut set, &mut file);
    }

    file.flush().unwrap();
    drop(file);

    (fs::metadata(path.clone()).unwrap().len() / (n * 3), path)
}

struct ModelReader(BufReader<File>, u64);

impl Iterator for ModelReader {
    type Item = Model;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.1;
        let mut model = Model {
            max: (0, 0, 0),
            data: Vec::new(),
        };

        for _ in 0..size {
            let mut buf = [0; 3];
            match self.0.read_exact(&mut buf) {
                Ok(_) => (),
                Err(_) => None?,
            }
            model.max.0 = model.max.0.max(buf[0]);
            model.max.1 = model.max.1.max(buf[1]);
            model.max.2 = model.max.2.max(buf[2]);
            model.data.push((buf[0], buf[1], buf[2]));
        }

        Some(model)
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[bench]
    fn seven(b: &mut Bencher) {
        b.iter(|| {
            let vec = vec![Model::default()];
            let (_, mut path) = next(vec.into_iter(), 2);
            for n in 3..7 {
                let f = ModelReader(BufReader::new(File::open(path).unwrap()), n - 1);
                let d = next(f, n);
                path = d.1;
            }
            let f = ModelReader(BufReader::new(File::open(path).unwrap()), 6);
            let d = next(f, 7);
            assert_eq!(d.0, 1023);
        });
    }
}

fn main() {
    let vec = vec![Model::default()];
    let (_, mut path) = next(vec.into_iter(), 2);
    for n in 3.. {
        let f = ModelReader(BufReader::new(File::open(path).unwrap()), n - 1);
        let d = next(f, n);
        path = d.1;
        println!("{}: {}", n, d.0);
    }
}
