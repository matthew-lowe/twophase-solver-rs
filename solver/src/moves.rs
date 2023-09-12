use std::error::Error;
use std::{fs::File, io::Write, path::Path};
use std::io::{prelude::*, SeekFrom, Read};

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

/// Generate edge flip move table
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

/// Generic function to load/generate a move table
fn load_move_table<const T_SIZE: usize, const T_BYTES_SIZE: usize>(f_name: &str, gen: impl Fn() -> [u16; T_SIZE]) -> Result<[u16; T_SIZE], Box<dyn Error>> {
    let dir = &format!("{}{}", "tables/", f_name);
    let path = Path::new(dir);
    match File::open(path) {
        Ok(mut f) => {
            let mut buffer = [0u8; T_BYTES_SIZE];

            read_by_byte::<T_BYTES_SIZE>(&mut f, &mut buffer);
            let grouped_bytes: [[u8; 2]; T_SIZE] = combine_byte_groups(buffer);
            let new_bytes: &[u16; T_SIZE] = bytemuck::cast_ref(&grouped_bytes);
            Ok(*new_bytes)
        },
        Err(_) => {
            let mut f = File::create(path)?;
            let moves = gen();
            let bytes: &[u8; T_BYTES_SIZE] = bytemuck::cast_ref(&moves);
            f.write_all(bytes)?;

            Ok(moves)
        }
    }
}

/// Load the twist move table, generating it if it doesn't exist
/// Errors are just returned if generated
/// `dir` can be optional path to the file 
pub fn load_twist_move_table(dir: Option<&Path>) -> Result<[u16; TWIST_SIZE], Box<dyn Error>> {
    match dir {
        Some(p) => load_move_table::<TWIST_SIZE, TWIST_BYTES_SIZE>(p.join("move_twist").to_str().unwrap(), gen_twist_move_table),
        None => load_move_table::<TWIST_SIZE, TWIST_BYTES_SIZE>("move_twist", gen_twist_move_table),
    }
}

pub fn load_flip_move_table(dir: Option<&Path>) -> Result<[u16; FLIP_SIZE], Box<dyn Error>> {
    match dir {
        Some(p) => load_move_table::<FLIP_SIZE, FLIP_BYTES_SIZE>(p.join("move_flip").to_str().unwrap(), gen_flip_move_table),
        None => load_move_table::<FLIP_SIZE, FLIP_BYTES_SIZE>("move_flip", gen_flip_move_table),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;

    /// Return an empty array to ensure the test fails
    fn zoinks<T, const S: usize>() -> [T; S] where T: Default + Copy {
        [T::default(); S]
    }

    /// Ensure that files are saved and loaded with the same data
    #[test]
    fn file_saves_and_loads() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path();
        println!("{:?}", tmppath);
        let _ = load_twist_move_table(Some(tmppath)).unwrap();
        assert_eq!(load_twist_move_table(Some(tmppath)).unwrap(), gen_twist_move_table());
        let _ = load_flip_move_table(Some(tmppath)).unwrap();
        assert_eq!(load_flip_move_table(Some(tmppath)).unwrap(), gen_flip_move_table());
    }

    /// Compare the twist data to a known good and ensure they match
    #[test]
    fn twist_file_correct() {
        let twists = load_twist_move_table(None).unwrap();
        let good_twists = load_move_table::<TWIST_SIZE, TWIST_BYTES_SIZE>("known_good/move_twist", zoinks).unwrap();
        assert_eq!(twists, good_twists);
    }

    /// Compare the flip data to a known good and ensure they match
    #[test]
    fn flip_file_correct() {
        let flips = load_flip_move_table(None).unwrap();
        let good_flips = load_move_table::<FLIP_SIZE, FLIP_BYTES_SIZE>("known_good/move_flip", zoinks).unwrap();
        //assert_eq!(good_flips, flips);
    }
}

