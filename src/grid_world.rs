
use ndarray::Array2;
use std::{
    cmp,
    fmt::Debug,
    fs::File,
    io::{Error as IOError, Read},
    str::FromStr,
};

#[derive(Clone, Copy)]
pub enum Motion {
    North(usize),
    East(usize),
    South(usize),
    West(usize),

    NorthEast(usize),
    NorthWest(usize),
    SouthEast(usize),
    SouthWest(usize),
}

impl Motion {
    pub fn from_usize(i: usize, n: usize) -> Motion {
        match i {
            0 => Motion::North(n),
            1 => Motion::East(n),
            2 => Motion::South(n),
            3 => Motion::West(n),
            _ => panic!("Unknown motion {}!", i),
        }
    }
}

pub struct GridWorld<T> {
    layout: Array2<T>,
}

impl<T> GridWorld<T> {
    pub fn new(layout: Array2<T>) -> GridWorld<T> { GridWorld { layout } }

    pub fn from_str(layout: &str) -> GridWorld<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        let m: Vec<Vec<T>> = layout
            .lines()
            .map(|l| {
                l.split(char::is_whitespace)
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();

        let shape = (m.len(), m[0].len());
        let mvals = m.into_iter().flat_map(|v| v).collect();

        GridWorld {
            layout: Array2::from_shape_vec(shape, mvals).unwrap(),
        }
    }

    pub fn from_file(path: &str) -> Result<GridWorld<T>, IOError>
    where
        T: FromStr,
        T::Err: Debug,
    {
        let mut f = File::open(path).unwrap();
        let mut buffer = String::new();

        match f.read_to_string(&mut buffer) {
            Ok(_) => Ok(GridWorld::<T>::from_str(&buffer)),
            Err(e) => Err(e),
        }
    }

    pub fn height(&self) -> usize { self.layout.shape()[0] }

    pub fn width(&self) -> usize { self.layout.shape()[1] }

    pub fn get(&self, loc: [usize; 2]) -> Option<&T> { self.layout.get(loc) }

    pub fn get_mut(&mut self, loc: [usize; 2]) -> Option<&mut T> { self.layout.get_mut(loc) }

    pub fn move_north(&self, loc: [usize; 2], n: usize) -> [usize; 2] {
        [
            loc[0],
            cmp::max(
                0,
                cmp::min(
                  loc[1].saturating_sub(n),
                  self.height() - 1
                ),
            ),
        ]
    }

    pub fn move_south(&self, loc: [usize; 2], n: usize) -> [usize; 2] {
        [
            loc[0],
            cmp::max(
                0,
                cmp::min(
                  loc[1].saturating_add(n),
                  self.height() - 1
                ),
            ),
        ]
    }

    pub fn move_east(&self, loc: [usize; 2], n: usize) -> [usize; 2] {
        [
            cmp::max(
                0,
                cmp::min(
                  loc[0].saturating_add(n),
                  self.width() - 1
                ),
            ),
            loc[1],
        ]
    }

    pub fn move_west(&self, loc: [usize; 2], n: usize) -> [usize; 2] {
        [
            cmp::max(
                0,
                cmp::min(
                  loc[0].saturating_sub(n),
                  self.width() - 1
                ),
            ),
            loc[1],
        ]
    }

    pub fn perform_motion(&self, loc: [usize; 2], motion: Motion) -> [usize; 2] {
        match motion {
            Motion::North(n) => self.move_north(loc, n),
            Motion::South(n) => self.move_south(loc, n),
            Motion::East(n) => self.move_east(loc, n),
            Motion::West(n) => self.move_west(loc, n),

            Motion::NorthEast(n) => (0..n).fold(loc, |new_loc, _| {
                self.move_east(self.move_north(new_loc, 1), 1)
            }),
            Motion::NorthWest(n) => (0..n).fold(loc, |new_loc, _| {
                self.move_west(self.move_north(new_loc, 1), 1)
            }),

            Motion::SouthEast(n) => (0..n).fold(loc, |new_loc, _| {
                self.move_east(self.move_south(new_loc, 1), 1)
            }),
            Motion::SouthWest(n) => (0..n).fold(loc, |new_loc, _| {
                self.move_west(self.move_south(new_loc, 1), 1)
            }),
        }
    }

    pub fn valid_motion(&self, loc: [usize; 2], motion: Motion) -> bool {
        match motion {
            Motion::North(n) => loc[1] <= self.height() - 1 - n,
            Motion::South(n) => loc[1] >= n,
            Motion::East(n) => loc[0] <= self.width() - 1 - n,
            Motion::West(n) => loc[0] >= n,

            Motion::NorthEast(n) => {
                self.valid_motion(loc, Motion::North(n)) && self.valid_motion(loc, Motion::East(n))
            },
            Motion::NorthWest(n) => {
                self.valid_motion(loc, Motion::North(n)) && self.valid_motion(loc, Motion::West(n))
            },
            Motion::SouthEast(n) => {
                self.valid_motion(loc, Motion::South(n)) && self.valid_motion(loc, Motion::East(n))
            },
            Motion::SouthWest(n) => {
                self.valid_motion(loc, Motion::South(n)) && self.valid_motion(loc, Motion::West(n))
            },
        }
    }
}
