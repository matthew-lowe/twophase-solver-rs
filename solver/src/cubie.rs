use strum::IntoEnumIterator;
use std::fmt::Display;
use std::ops::Mul;
use crate::{common::{Corner, Edge, CORNER_FACELET, CORNER_COLOR, EDGE_COLOR, EDGE_FACELET}, face::FaceCube};

// Type aliases 
type CPerm = [Corner; 8]; // corner permuations
type COrie = [i8; 8]; // corner orientations
type EPerm = [Edge; 12]; // edge permuations
type EOrie = [i8; 12]; // edge orientations

// The six moves defined by cubie orientations and permutations
// These type aliases are just used for consts so it fits on my screen
type Co = Corner;
type Ed = Edge;

const CP_U: CPerm = [Co::UBR, Co::UFR, Co::UFL, Co::UBL, Co::DFR, Co::DFL, Co::DBL, Co::DBR];
const CO_U: COrie = [0, 0, 0, 0, 0, 0, 0, 0,];
const EP_U: EPerm = [Ed::UB, Ed::UR, Ed::UF, Ed::UL, Ed::DR, Ed::DF, Ed::DL, Ed::DB, Ed::FR, Ed::FL, Ed::BL, Ed::BR,];
const EO_U: EOrie = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const CP_R: CPerm = [Co::DFR, Co::UFL, Co::UBL, Co::UFR, Co::DBR, Co::DFL, Co::DBL, Co::UBR];
const CO_R: COrie = [2, 0, 0, 1, 1, 0, 0, 2];
const EP_R: EPerm = [Ed::FR, Ed::UF, Ed::UL, Ed::UB, Ed::BR, Ed::DF, Ed::DL, Ed::DB, Ed::DR, Ed::FL, Ed::BL, Ed::UR];
const EO_R: EOrie = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const CP_F: CPerm = [Co::UFL, Co::DFL, Co::UBL, Co::UBR, Co::UFR, Co::DFR, Co::DBL, Co::DBR];
const CO_F: COrie = [1, 2, 0, 0, 2, 1, 0, 0];
const EP_F: EPerm = [Ed::UR, Ed::FL, Ed::UL, Ed::UB, Ed::DR, Ed::FR, Ed::DL, Ed::DB, Ed::UF, Ed::DF, Ed::BL, Ed::BR];
const EO_F: EOrie = [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0];

const CP_D: CPerm = [Co::UFR, Co::UFL, Co::UBL, Co::UBR, Co::DFL, Co::DBL, Co::DBR, Co::DFR];
const CO_D: COrie = [0, 0, 0, 0, 0, 0, 0, 0];
const EP_D: EPerm = [Ed::UR, Ed::UF, Ed::UL, Ed::UB, Ed::DF, Ed::DL, Ed::DB, Ed::DR, Ed::FR, Ed::FL, Ed::BL, Ed::BR];
const EO_D: EOrie = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const CP_L: CPerm = [Co::UFR, Co::UBL, Co::DBL, Co::UBR, Co::DFR, Co::UFL, Co::DFL, Co::DBR];
const CO_L: COrie = [0, 1, 2, 0, 0, 2, 1, 0];
const EP_L: EPerm = [Ed::UR, Ed::UF, Ed::BL, Ed::UB, Ed::DR, Ed::DF, Ed::FL, Ed::DB, Ed::FR, Ed::UL, Ed::DL, Ed::BR];
const EO_L: EOrie = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const CP_B: CPerm = [Co::UFR, Co::UFL, Co::UBR, Co::DBR, Co::DFR, Co::DFL, Co::UBL, Co::DBL];
const CO_B: COrie = [0, 0, 1, 2, 0, 0, 2, 1];
const EP_B: EPerm = [Ed::UR, Ed::UF, Ed::UL, Ed::BR, Ed::DR, Ed::DF, Ed::DL, Ed::BL, Ed::FR, Ed::FL, Ed::UB, Ed::DB];
const EO_B: EOrie = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1];

// Default, solved, cube permutations. Solved orientation states are 0 for all
const CP_S: CPerm = [Co::UFR, Co::UFL, Co::UBL, Co::UBR, Co::DFR, Co::DFL, Co::DBL, Co::DBR];
const EP_S: EPerm = [Ed::UR, Ed::UF, Ed::UL, Ed::UB, Ed::DR, Ed::DF, Ed::DL, Ed::DB, Ed::FR, Ed::FL, Ed::BL, Ed::BR];

// Basic move cubes, solved cubes with each of the basic moves applied
pub const BASIC_MOVES: [CubieCube; 6] = [
    CubieCube {cp: CP_U, co: CO_U, ep: EP_U, eo: EO_U},
    CubieCube {cp: CP_R, co: CO_R, ep: EP_R, eo: EO_R},
    CubieCube {cp: CP_F, co: CO_F, ep: EP_F, eo: EO_F},
    CubieCube {cp: CP_D, co: CO_D, ep: EP_D, eo: EO_D},
    CubieCube {cp: CP_L, co: CO_L, ep: EP_L, eo: EO_L},
    CubieCube {cp: CP_B, co: CO_B, ep: EP_B, eo: EO_B},
];

// Cube defined in terms of cubie permutations and orientations
#[derive(PartialEq, Clone)]
pub struct CubieCube {
    cp: CPerm,
    co: COrie,
    ep: EPerm,
    eo: EOrie,
}

impl CubieCube {
    pub fn new(cp: Option<CPerm>, co: Option<COrie>, ep: Option<EPerm>, eo: Option<EOrie>) -> Self {
        CubieCube {
            cp: cp.unwrap_or(CP_S),
            co: co.unwrap_or([0; 8]),
            ep: ep.unwrap_or(EP_S),
            eo: eo.unwrap_or([0; 12]),
        }
    }

    // Converts to a new FaceCube object
    pub fn to_new_facelet(&self) -> FaceCube {
        let mut faces = FaceCube::solved_colors();
        
        // Corners
        for i in 0..8 { // For each corner on the cube
            let p = self.cp[i] as u8; // The cubie that is at that corner
            let o = self.co[i]; // Orientation index, 0 = matches cubie ref, 1 = CW, 2 = CCW

            for k in 0..3 { // For each face on the corner going CW
                // LHS: 1st index is the corner (i), 2nd is face going CW from ref ((k+o)%3)
                // RHS: 1st index is the cubie at the corner, 2nd index is the face of the cubie
                faces[CORNER_FACELET[i][(k+o) as usize % 3] as usize] = CORNER_COLOR[p as usize][k as usize];
            }
        }
        for i in 0..12 { // For each edge
            let p = self.ep[i] as i8; // Permutation index
            let o = self.eo[i]; // Orientation index, 0 = matches cubie ref, 1 = doesn't match

            for k in 0..2 { // For each face on the edge
                // LHS: 1st index is the edge (i), 2nd is face ((k+o)%2)
                // RHS: 1st index is the cubie at the edge, 2nd index is the face of the cubie
                faces[EDGE_FACELET[i][(k+o) as usize % 2] as usize] = EDGE_COLOR[p as usize][k as usize];
            }
        }

        // Edges

        FaceCube { faces }
    }

    // (A*B)(x).c = A(B(x).c).c
    // (A*B)(x).o = (A(B(x).c).o + B(x).o
    // Multiply self by another CubieCube
    pub fn corner_multiply(&mut self, b: &Self) {
        let mut c_perm = [Corner::UFR; 8]; // Final corner permutations
        let mut c_orie = [0; 8]; // Final corner orientation

        for c in Corner::iter() { // Multiply corner by corner
            let c_i = c as usize;

            c_perm[c_i] = self.cp[b.cp[c_i] as usize]; // Set the c_perm as the product of
                                                              // self's perm and b's perm
            let ori_a = self.co[b.cp[c_i] as usize]; // product of self's and b's orientation
            let ori_b = b.co[c_i]; // self's orientation

            // Since reflections of the cube must also be considered, this uses a D3 group
            // with 3,4,5 representing the 3 different mirrored states a corner could be in.
            // There's probably a more readable way to achieve this...
            c_orie[c_i] = match(ori_a, ori_b) {
                (a, b) if a < 3 && b < 3 => if a + b >= 3 { a + b - 3 } else { a + b }, // reg cubes (between 0 and 2)
                (a, b) if a < 3 && b >= 3 => if a + b >= 6 { a + b - 3 } else {a + b }, // b mirrored, result is mirrored (between 3 and 5)
                (a, b) if a >= 3 && b < 3 => if a - b < 3 { a - b + 3 } else { a - b }, // a mirrored, result is mirrored (between 3 and 5)
                (a, b) => if a - b < 0 { a - b + 3 } else { a - b }, // a >= 3 && b >= 3, both mirrored, result is reg (between 0 and 2)
            }
        }; 
        
        for c in Corner::iter() {
            let c_i = c as usize;

            self.cp[c_i] = c_perm[c_i];
            self.co[c_i] = c_orie[c_i];
        }
    }

    pub fn corner_multiply_simple(&mut self, b: &Self) {
        let mut c_perm = [Corner::UFR; 8];
        let mut c_orie = [0; 8];

        for c in Corner::iter() {
            let c_i = c as usize;
            c_perm[c_i] = self.cp[b.cp[c_i] as usize];
            c_orie[c_i] = (b.co[c_i] + self.co[b.cp[c_i] as usize]) % 3;
        }

        self.cp = c_perm;
        self.co = c_orie;
    }

    // Edges can't be mirrored so can be multiplied normally
    pub fn edge_multiply(&mut self, b: &Self) {
        let mut e_perm = [Edge::UF; 12];
        let mut e_orie = [0; 12];

        for e in Edge::iter() {
            let e_i = e as usize;
            e_perm[e_i] = self.ep[b.ep[e_i] as usize];
            e_orie[e_i] = (b.eo[e_i] + self.eo[b.ep[e_i] as usize]) % 2;

        }

        self.ep = e_perm;
        self.eo = e_orie;
    }

    // Corner orientation coord, 0..2186, convert orientation in order to ternary number
    pub fn get_twist(&self) -> u16 {
        let mut total: u16 = 0;
        for i in 0..7 { // Ignore DBR since it can be calculated from others
            total = 3 * total + self.co[i] as u16;
        }
        total
    }

    pub fn set_twist(&mut self, mut twist: u16) {
        let mut tp: u16 = 0;

        for i in (0..7).rev() {
            // Have to -1 inside because usize can't be negative and the compiler doesn't
            // understand range is exclusive at the end
            self.co[i as usize] = (twist % 3) as i8;
            tp += self.co[i as usize] as u16;
            twist = twist / 3;
        }

        self.co[Corner::DBR as usize] = ((3 - tp % 3) % 3) as i8;
    }
}

// Multiply is the group operation
impl Mul for CubieCube {
    type Output = CubieCube;

    fn mul(self, rhs: Self) -> Self::Output {
        let c = self.clone();
        c.clone().corner_multiply(&rhs);
        c.clone().edge_multiply(&rhs);
        c
    }
}

impl Display for CubieCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            f.write_str(format!("({:?}, {:?})", self.cp[i], self. co[i]).as_str()).unwrap();
        };

        f.write_str("\n").unwrap();

        for i in 0..12 {
            f.write_str(format!("({:?}, {:?})", self.ep[i], self. eo[i]).as_str()).unwrap();
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice() {
        let mut my_thing = CubieCube::new(None, None, None, None);
        my_thing.set_twist(1494);
    }
}
