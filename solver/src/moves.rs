use std::error::Error;
use std::{fs::File, io::Write};
use std::io::{prelude::*, SeekFrom};

use strum::IntoEnumIterator;
use bytemuck;
use crate::{common::{N_TWIST, N_MOVE, Color}, cubie::{CubieCube, BASIC_MOVES}};

const MT_SIZE: usize = N_TWIST*N_MOVE;
const BYTE_SIZE: usize = MT_SIZE*4;

/// Generate the twist move table
fn gen_twist_move_table() -> [u32; MT_SIZE] {
    let mut twist_move = [0; MT_SIZE];
    let mut a = CubieCube::new(None, None, None, None);

    for i in 0..N_TWIST {
        a.set_twist(i as u16);
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
pub fn load_twist_move_table() -> Result<[u32; MT_SIZE], Box<dyn Error>> {
    match File::open("twist_moves") {
        Ok(mut f) => {
            let mut buffer = [0u8; BYTE_SIZE];
            
            // f.read_to_end() requires a vector, smh
            for i in 0..(BYTE_SIZE/4) {
                let b = 4*i; // Every 4 bytes
                f.seek(SeekFrom::Start(b as u64))?;
                
                let mut buf = [0u8; 4];
                f.read_exact(&mut buf)?;

                buffer[b..b+4].copy_from_slice(&buf);
            }

            let mut bytes: [[u8; 4]; MT_SIZE] = [[0; 4]; MT_SIZE];
            
            for i in 0..MT_SIZE {
                let j = 4*i;
                for k in 0..4 {
                    bytes[i][k] = buffer[j + k];
                }
            };


            let new_bytes: &[u32; MT_SIZE] = bytemuck::cast_ref(&bytes);

            Ok(*new_bytes)
        },
        Err(_) => {
            let mut f = File::create("twist_moves")?;
            let moves = gen_twist_move_table(); // [u16; MT_SIZE]


            // The unsafe one is cooler but worse :(
            /*
            let mut bytes = [0u8; BYTE_SIZE];
            
            unsafe {
                bytes = transmute(moves);
            }
            */

            let bytes: &[u8; BYTE_SIZE] = bytemuck::cast_ref(&moves);
            f.write_all(bytes)?;

            Ok(moves)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file() {
        let _ = load_twist_move_table().unwrap(); // Ensure the table is created and stored in a file
        assert_eq!(load_twist_move_table().unwrap(), gen_twist_move_table());
    }
}

