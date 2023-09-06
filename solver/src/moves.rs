use std::error::Error;
use std::{fs::File, io::Write};
use std::io::prelude::*;

use strum::IntoEnumIterator;
use arrayvec::ArrayVec;
use crate::{common::{N_TWIST, N_MOVE, Color}, cubie::{CubieCube, BASIC_MOVES}};

const MT_SIZE: usize = N_TWIST*N_MOVE;

/// Generate the twist move table
fn gen_twist_move_table() -> [u16; MT_SIZE] {
    let mut twist_move = [0; MT_SIZE];
    let mut a = CubieCube::new(None, None, None, None);

    for i in 0..N_TWIST {
        a.set_twist(i.clone() as u16);
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
/// Errors are just returned if generated
pub fn load_twist_move_table() -> Result<[u16; MT_SIZE], Box<dyn Error>> {
    match File::open("twist_moves") {
        Ok(mut f) => {
            let mut buffer: Vec::<u8> = Vec::<u8>::new();
            let _ = f.read_to_end(&mut buffer)?; // we know the size duh
            println!("buffer size: {:?}", buffer.len());
            let bytes: [u8; 2*MT_SIZE] = buffer.try_into().unwrap();
            let mut moves_bytes: [[u8; 2]; MT_SIZE] = [[0; 2]; MT_SIZE];
            
            for i in 0..MT_SIZE {
                let j = 2*i;
                for k in 0..2 {
                    moves_bytes[i][k] = bytes[j + k];
                }
            }

            Ok(moves_bytes
                .into_iter()
                .map(|i| u16::from_ne_bytes(i))
                .collect::<Vec<u16>>()
                .try_into()
                .unwrap()
               )
        },
        Err(_) => {
            let mut f = File::create("twist_moves")?;
            let moves = gen_twist_move_table(); // [u32; MT_SIZE]

            const BYTE_SIZE: usize = MT_SIZE*2;
            let bytes: [u8; MT_SIZE*2] = (&moves[..])
                .into_iter()
                .map(|i| i.to_ne_bytes())
                .flatten()
                .collect::<ArrayVec<u8, BYTE_SIZE>>()
                .into_inner().unwrap();

            f.write_all(&bytes[..])?;

            Ok(moves)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file() {
        println!("{}", MT_SIZE);
        let _ = load_twist_move_table(); // Ensure the table is created and stored in a file
        assert_eq!(load_twist_move_table().unwrap(), gen_twist_move_table());
    }
}

