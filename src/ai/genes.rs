use super::Gene;
use crate::game::{StandardGame, HEIGHT, WIDTH};

pub struct Holes;
pub struct MaxHeight;
pub struct Bumpiness;

impl Gene for Holes {
    fn evaluate(&self, state: &StandardGame) -> f64 {
        let mut holes = 0;
        for x in 0..WIDTH {
            let mut found = false;
            for y in 0..HEIGHT {
                if state.grid[y][x].is_none() {
                    found = true;
                } else if found {
                    holes += 1;
                }
            }
        }
        holes as f64
    }
}

impl Gene for MaxHeight {
    fn evaluate(&self, state: &StandardGame) -> f64 {
        let mut max_height = 0;
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if state.grid[y][x].is_some() {
                    max_height = max_height.max(HEIGHT - y);
                    break;
                }
            }
        }
        max_height as f64
    }
}

impl Gene for Bumpiness {
    fn evaluate(&self, state: &StandardGame) -> f64 {
        let mut bumpiness = 0;
        let mut prev_height = 0;
        for x in 0..WIDTH {
            let mut height = 0;
            for y in 0..HEIGHT {
                if state.grid[y][x].is_some() {
                    height = (HEIGHT - y) as i32;
                    break;
                }
            }
            if x > 0 {
                bumpiness += (prev_height - height).abs();
            }
            prev_height = height;
        }
        bumpiness as f64
    }
}

// TODO: unit tests
