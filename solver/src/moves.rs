use std::error::Error;
use std::{fs::File, io::Write};
use std::io::{prelude::*, SeekFrom};

use strum::IntoEnumIterator;
use bytemuck;
use crate::common::N_FLIP;
use crate::{common::{N_TWIST, N_MOVE, Color}, cubie::{CubieCube, BASIC_MOVES}};

const BYTES_PER_U16: usize = 2;

const TWIST_SIZE: usize = N_TWIST*N_MOVE;
const TWIST_BYTES_SIZE: usize = TWIST_SIZE*BYTES_PER_U16;

const FLIP_SIZE: usize = N_FLIP*N_MOVE;
const FLIP_BYTES_SIZE: usize = FLIP_SIZE*BYTES_PER_U16;

/// Generate the twist move table
fn gen_twist_move_table() -> [u16; TWIST_SIZE] {
    let mut twist_move = [0; TWIST_SIZE];
    let mut a = CubieCube::new(None, None, None, None);

    for i in 0..N_TWIST { // For each possible twist set
        a.set_twist(i as u16); // Set the cube to that twist
        for j in Color::iter() { // And for each face
            for k in 0..3 { // For each move that can be done to that face
                // Apply the move only to the corners
                a.corner_multiply(&BASIC_MOVES[j as usize]);
                // Save the twist result in the table
                twist_move[N_MOVE*i + 3*j as usize + k] = a.get_twist();
            }
            a.corner_multiply_simple(&BASIC_MOVES[j as usize]);
        }
    }

    twist_move
}

// Generate edge flip move table
fn gen_flip_move_table() -> [u16; FLIP_SIZE] {
    let mut flip_move = [0; FLIP_SIZE];
    let mut a = CubieCube::new(None, None, None, None);

    for i in 0..N_FLIP {
        a.set_flip(i as u16);
        for j in Color::iter() {
            for k in 0..3 {
                a.edge_multiply(&BASIC_MOVES[j as usize]);
                flip_move[N_MOVE*i + 3*j as usize + k] = a.get_flip();
            }
            a.edge_multiply(&BASIC_MOVES[j as usize]);
        }
    }

    flip_move
}

/// Read from `f` into `buffer`, length of `BUFFER_SIZE` must be > length of `f`
fn read_by_byte<const BUFFER_SIZE: usize>(f: &mut File, buffer: &mut [u8; BUFFER_SIZE]) {
    for i in 0..(BUFFER_SIZE/2) {
        let b = BYTES_PER_U16*i; // Every 2 bytes
        let _ = f.seek(SeekFrom::Start(b as u64));

        let mut buf = [0u8; BYTES_PER_U16];
        let _ = f.read_exact(&mut buf).unwrap();

        buffer[b..b+BYTES_PER_U16].copy_from_slice(&buf);
    }
}

/// Combine a byte array into an array of byte arrays (groups)
fn combine_byte_groups<const BUFFER_SIZE: usize, const OUT_SIZE: usize>(buffer: [u8; BUFFER_SIZE]) -> [[u8; BYTES_PER_U16]; OUT_SIZE] {
    let mut bytes = [[0u8; BYTES_PER_U16]; OUT_SIZE];

    for i in 0..OUT_SIZE {
        let j = BYTES_PER_U16*i;
        for k in 0..BYTES_PER_U16 {
            bytes[i][k] = buffer[j + k];
        }
    };

    bytes
}

/// Load the twist move table, generating it if it doesn't exist
/// Errors are just returned if generated
pub fn load_twist_move_table() -> Result<[u16; TWIST_SIZE], Box<dyn Error>> {
    match File::open("twist_moves") {
        Ok(mut f) => {
            let mut buffer = [0u8; TWIST_BYTES_SIZE];

            read_by_byte::<TWIST_BYTES_SIZE>(&mut f, &mut buffer);
            let bytes: [[u8; 2]; TWIST_SIZE] = combine_byte_groups(buffer);
            let new_bytes: &[u16; TWIST_SIZE] = bytemuck::cast_ref(&bytes);
            Ok(*new_bytes)

        },
        Err(_) => {
            let mut f = File::create("twist_moves")?;
            let moves = gen_twist_move_table(); // [u16; MT_SIZE]
            let bytes: &[u8; TWIST_BYTES_SIZE] = bytemuck::cast_ref(&moves);
            f.write_all(bytes)?;

            Ok(moves)
        },
    }
}

pub fn load_flip_move_table() -> Result<[u16; FLIP_SIZE], Box<dyn Error>> {
    match File::open("flip_moves") {
        Ok(mut f) => {
            let mut buffer = [0u8; FLIP_BYTES_SIZE];

            read_by_byte::<FLIP_BYTES_SIZE>(&mut f, &mut buffer);
            let bytes: [[u8; 2]; FLIP_SIZE] = combine_byte_groups(buffer);
            let new_bytes: &[u16; FLIP_SIZE] = bytemuck::cast_ref(&bytes);
            Ok(*new_bytes)
        },
        Err(_) => {
            let mut f = File::create("flip_moves")?;
            let moves = gen_flip_move_table(); // [u16; MT_SIZE]
            let bytes: &[u8; FLIP_BYTES_SIZE] = bytemuck::cast_ref(&moves);
            f.write_all(bytes)?;

            Ok(moves)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twist_file_saves_and_loads() {
        let _ = load_twist_move_table().unwrap(); // Ensure the table is created and stored in a file if not already
        assert_eq!(load_twist_move_table().unwrap(), gen_twist_move_table());
    }

    #[test]
    fn flip_file_saves_and_loads() {
        let _ = load_flip_move_table().unwrap(); // Ensure the table is created and stored in a file if not already
        assert_eq!(load_flip_move_table().unwrap(), gen_flip_move_table());

    }
}

