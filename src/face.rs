use std::fmt::Display;
use crate::common::Color;

pub struct FaceCube {
    pub faces: [Color; 54],
}



impl FaceCube {
    pub fn new() -> Self {
        Self {
            faces: Self::solved_colors(),
        }
    }

    pub fn solved_colors() -> [Color; 54] {
        [
            [Color::U; 9],
            [Color::R; 9],
            [Color::F; 9],
            [Color::D; 9],
            [Color::L; 9],
            [Color::B; 9],
        ].concat().try_into().unwrap_or_else(|_| panic!("nice"))
    }
}

impl Display for FaceCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.faces.iter() {
            f.write_str(match c {
                Color::U => "U",
                Color::R => "R",
                Color::F => "F",
                Color::D => "D",
                Color::L => "L",
                Color::B => "B",
            }).unwrap();
        }
        
        Ok(())
    }
}



