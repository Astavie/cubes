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
            return match model.data.binary_search(&(x, y, z)) {
                Err(pos) => {
                    model.data.insert(pos, (x, y, z));
                    Some(model)
                }
                Ok(_) => None,
            };
        }

        let coord = (
            x.wrapping_add_signed(xo),
            y.wrapping_add_signed(yo),
            z.wrapping_add_signed(zo),
        );

        let mut data = self.data.clone();
        match data.binary_search(&coord) {
            Err(pos) => {
                data.insert(pos, coord);
                Some(Model {
                    max: (
                        u8::max(self.max.0, coord.0),
                        u8::max(self.max.1, coord.1),
                        u8::max(self.max.2, coord.2),
                    ),
                    data,
                })
            }
            Ok(_) => None,
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
    fn rotations(&self) -> Vec<Model> {
        let mut rotations = Vec::with_capacity(24);

        // Define rotation functions for each axis
        let rotate_coord_x = |(x, y, z): Coord, (_, my, _): Coord| (x, z, my - y);
        let rotate_coord_y = |(x, y, z): Coord, (_, _, mz): Coord| (mz - z, y, x);
        let rotate_coord_z = |(x, y, z): Coord, (mx, _, _): Coord| (y, mx - x, z);

        let sort = |mut v: Vec<Coord>| {
            v.sort_unstable();
            v
        };

        // TODO: can this be done quicker without having to resort?
        let rotate_x = |m: &Model| Model {
            max: (m.max.0, m.max.2, m.max.1),
            data: sort(m.data.iter().map(|&c| rotate_coord_x(c, m.max)).collect()),
        };
        let rotate_y = |m: &Model| Model {
            max: (m.max.2, m.max.1, m.max.0),
            data: sort(m.data.iter().map(|&c| rotate_coord_y(c, m.max)).collect()),
        };
        let rotate_z = |m: &Model| Model {
            max: (m.max.1, m.max.0, m.max.2),
            data: sort(m.data.iter().map(|&c| rotate_coord_z(c, m.max)).collect()),
        };

        // 4 rotations around x-axis
        let rot0 = self.clone();
        let rot1 = rotate_x(&rot0);
        let rot2 = rotate_x(&rot1);
        let rot3 = rotate_x(&rot2);
        rotations.push(rot0);
        rotations.push(rot1);
        rotations.push(rot2);
        rotations.push(rot3);

        // rotate 180 around y-axis
        // 4 rotations around x-axis
        let rot0 = rotate_y(&rotate_y(self));
        let rot1 = rotate_x(&rot0);
        let rot2 = rotate_x(&rot1);
        let rot3 = rotate_x(&rot2);
        rotations.push(rot0);
        rotations.push(rot1);
        rotations.push(rot2);
        rotations.push(rot3);

        // rotate 90/270 around y-axis
        // 8 rotations around z-axis
        let rot0 = rotate_y(self);
        let rot1 = rotate_z(&rot0);
        let rot2 = rotate_z(&rot1);
        let rot3 = rotate_z(&rot2);
        rotations.push(rot0);
        rotations.push(rot1);
        rotations.push(rot2);
        rotations.push(rot3);
        let rot0 = rotate_y(&rotate_y(&rotate_y(self)));
        let rot1 = rotate_z(&rot0);
        let rot2 = rotate_z(&rot1);
        let rot3 = rotate_z(&rot2);
        rotations.push(rot0);
        rotations.push(rot1);
        rotations.push(rot2);
        rotations.push(rot3);

        // rotate 90/270 around z-axis
        // 8 rotations around y-axis
        let rot0 = rotate_z(self);
        let rot1 = rotate_y(&rot0);
        let rot2 = rotate_y(&rot1);
        let rot3 = rotate_y(&rot2);
        rotations.push(rot0);
        rotations.push(rot1);
        rotations.push(rot2);
        rotations.push(rot3);
        let rot0 = rotate_z(&rotate_z(&rotate_z(self)));
        let rot1 = rotate_y(&rot0);
        let rot2 = rotate_y(&rot1);
        let rot3 = rotate_y(&rot2);
        rotations.push(rot0);
        rotations.push(rot1);
        rotations.push(rot2);
        rotations.push(rot3);

        rotations
    }
    fn hash(&self) -> u64 {
        // TODO: rotations do not have to be stored seperately
        // TODO: can the voxel vec be hashed without being sorted?
        self.rotations()
            .iter()
            .map(|rot| {
                let mut hash = DefaultHasher::new();
                rot.data.hash(&mut hash);
                hash.finish()
            })
            .min()
            .unwrap()
    }
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

fn main() {
    let mut map = HashMap::new();
    for n in 0.. {
        // println!("\n{:?}", map);
        println!("{}: {}", n, map.len().max(1));
        map = next(&map);
    }
}
