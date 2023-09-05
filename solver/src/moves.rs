use std::{fs::File, io::Write};
use std::io::prelude::*;

use strum::IntoEnumIterator;
use crate::{common::{N_TWIST, N_MOVE, Color}, cubie::{CubieCube, BASIC_MOVES}};

/// Generate the twist move table
fn gen_twist_move_table() -> [u32; N_TWIST*N_MOVE] {
    let mut twist_move = [0; N_TWIST*N_MOVE];
    let mut a = CubieCube::new(None, None, None, None);

    for i in 0..N_TWIST {
        a.set_twist(i.clone() as u32);
        //println!("{}", i as u32);
        for j in Color::iter() {
            for k in 0..3 {
                a.corner_multiply_simple(&BASIC_MOVES[j as usize]);
                twist_move[N_MOVE*i + 3*j as usize + k] = a.get_twist();
            }
            a.corner_multiply_simple(&BASIC_MOVES[j as usize]);
        }
    }

    twist_move
}

/// Load the twist move table, generating it if it doesn't exist
pub fn load_twist_move_table() -> [u32; N_TWIST*N_MOVE] {
    match File::open("twist_moves") {
        Ok(mut f) => {
            let mut buffer: Vec::<u8> = Vec::<u8>::new();
            let size = f.read_to_end(&mut buffer).unwrap();
            let bytes: [u8; 4*N_TWIST*N_MOVE] = buffer.try_into().unwrap();
            let moves_bytes: [[u8; 4]; N_TWIST*N_MOVE];
            
            for i in 0..N_TWIST*N_MOVE {
                
            }

            [0; 39366]
        },
        Err(_) => {
            let mut f = File::create("twist_moves").unwrap();
            let moves = gen_twist_move_table();
            // okay
            let bytes: [u8; N_TWIST*N_MOVE*4] = (&moves[..])
                .into_iter()
                .map(|i| i.to_ne_bytes())
                .flatten()
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();

            f.write_all(&bytes[..]);
            moves
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file() {
        load_twist_move_table();
    }
}

