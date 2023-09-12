use std::fmt::{Display, Debug};
use crate::{common::{Color, CORNER_FACELET, CORNER_COLOR, EDGE_FACELET, EDGE_COLOR}, cubie::CubieCube};

pub struct FaceCube {
    pub faces: [Color; 54],
}



impl FaceCube {
    pub fn new() -> Self {
        Self {
            faces: Self::solved_colors(),
        }
    }

    pub fn from_cubie(other: &CubieCube) -> FaceCube {
        let mut faces = FaceCube::solved_colors();
        
        // Corners
        for i in 0..8 { // For each corner on the cube
            let p = other.cp[i] as u8; // The cubie that is at that corner
            let o = other.co[i]; // Orientation index, 0 = matches cubie ref, 1 = CW, 2 = CCW

            for k in 0..3 { // For each face on the corner going CW
                // LHS: 1st index is the corner (i), 2nd is face going CW from ref ((k+o)%3)
                // RHS: 1st index is the cubie at the corner, 2nd index is the face of the cubie
                faces[CORNER_FACELET[i][(k+o) as usize % 3] as usize] = CORNER_COLOR[p as usize][k as usize];
            }
        }
        for i in 0..12 { // For each edge
            let p = other.ep[i] as i8; // Permutation index
            let o = other.eo[i]; // Orientation index, 0 = matches cubie ref, 1 = doesn't match

            for k in 0..2 { // For each face on the edge
                // LHS: 1st index is the edge (i), 2nd is face ((k+o)%2)
                // RHS: 1st index is the cubie at the edge, 2nd index is the face of the cubie
                faces[EDGE_FACELET[i][(k+o) as usize % 2] as usize] = EDGE_COLOR[p as usize][k as usize];
            }
        }

        // Edges

        FaceCube { faces }
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

/// Just prints the list of facelets
impl Debug for FaceCube {
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

/// User-friendly(er) 2D representation
impl Display for FaceCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        let r: String = format!("   {}\n   {}\n   {}\n{}{}{}{}\n{}{}{}{} \n{}{}{}{}\n   {}\n   {}\n   {}\n",
                        &s[0..3], &s[3..6], &s[6..9], 
                        &s[36..39], &s[18..21], &s[9..12], &s[45..48],
                        &s[39..42], &s[21..24], &s[12..15], &s[48..51],
                        &s[42..45], &s[24..27], &s[15..18], &s[51..54],
                        &s[27..30], &s[30..33], &s[33..36]);

        f.write_str(&r)
    }
}



