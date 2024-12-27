use std::error::Error;
use std::{fs::File, io::Write, path::Path};
use std::io::{prelude::*, Read, SeekFrom};

use strum::IntoEnumIterator;
use bytemuck::{self, bytes_of};

use crate::{common::{N_SLICE_SORTED, N_FLIP, N_TWIST, N_MOVE, Color}, cubie::{CubieCube, BASIC_MOVES}};

const BYTES_PER_U16: usize = 2;

const TWIST_SIZE: usize = N_TWIST*N_MOVE;
const TWIST_BYTES_SIZE: usize = TWIST_SIZE*BYTES_PER_U16;

const FLIP_SIZE: usize = N_FLIP*N_MOVE;
const FLIP_BYTES_SIZE: usize = FLIP_SIZE*BYTES_PER_U16;

const UD_SIZE: usize = N_SLICE_SORTED*N_MOVE;
const UD_BYTES_SIZE: usize = UD_SIZE*BYTES_PER_U16;

/// Generate the twist move table
fn gen_twist_move_table() -> Vec<u16> {
    let mut twist_move = vec![0u16; TWIST_SIZE];
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
            a.corner_multiply(&BASIC_MOVES[j as usize]); // TODO: is simple?
        }
    }

    twist_move
}

/// Generate edge flip move table
fn gen_flip_move_table() -> Vec<u16> {
    let mut flip_move = vec![0; FLIP_SIZE];
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

fn gen_ud_move_table() -> Vec<u16> {
    let mut slice_sorted_move = vec![0; UD_SIZE];
    let mut a = CubieCube::new(None, None, None, None);

    for i in 0..N_SLICE_SORTED {
        a.set_slice_sorted(i as u16);
        for j in Color::iter() {
            for k in 0..3 {
                a.edge_multiply(&BASIC_MOVES[j as usize]);
                slice_sorted_move[N_MOVE*i + 3*j as usize + k] = a.get_slice_sorted();
            }
            a.edge_multiply(&BASIC_MOVES[j as usize]);
        }
    }

    slice_sorted_move
}

/// Read from `f` into `buffer`, length of `BUFFER_SIZE` must be > length of `f`
fn read_by_byte<const BUFFER_SIZE: usize>(f: &mut File, buffer: &mut Vec<u8>) {
    for i in 0..(BUFFER_SIZE/2) {
        let b = BYTES_PER_U16*i; // Every 2 bytes
        let _ = f.seek(SeekFrom::Start(b as u64));

        let mut buf = [0u8; BYTES_PER_U16];
        let _ = f.read_exact(&mut buf).unwrap();

        buffer[b..b+BYTES_PER_U16].copy_from_slice(&buf);
    }
}

/// Combine a byte array into an array of byte arrays (groups)
fn combine_byte_groups<const OUT_SIZE: usize>(buffer: Vec<u8>) -> Vec<[u8; BYTES_PER_U16]> {
    let mut bytes = vec![[0u8; BYTES_PER_U16]; OUT_SIZE];

    for i in 0..OUT_SIZE {
        let j = BYTES_PER_U16*i;
        for k in 0..BYTES_PER_U16 {
            bytes[i][k] = buffer[j + k];
        }
    };
 
    bytes
}

/// Generic function to load/generate a move table
fn load_move_table<const T_SIZE: usize, const T_BYTES_SIZE: usize>(f_name: &str, gen: impl Fn() -> Vec<u16>) -> Result<Vec<u16>, Box<dyn Error>> {
    let dir = &format!("{}{}", "tables/", f_name);
    let path = Path::new(dir);
    match File::open(path) {
        Ok(mut f) => {
            let mut buffer = vec![0u8; T_BYTES_SIZE];
            read_by_byte::<T_BYTES_SIZE>(&mut f, &mut buffer);
            let grouped_bytes: Vec<[u8; 2]> = combine_byte_groups::<T_SIZE>(buffer);
            let new_bytes: &[u16] = bytemuck::cast_slice(&grouped_bytes);
            let r = new_bytes.to_vec();
            Ok(r)
        },
        Err(_) => {
            let mut f = File::create(path)?;
            let moves = gen();
            let bytes: Vec<[u8; 2]> = bytemuck::cast_slice(&moves).to_vec();
            for i in &bytes {
                f.write_all(i).expect("Unable to write");
            }

            Ok(moves)
        }
    }
}

/// Load the twist move table, generating it if it doesn't exist
/// Errors are just returned if generated
/// `dir` can be optional path to the file 
pub fn load_twist_move_table(dir: Option<&Path>) -> Result<Vec<u16>, Box<dyn Error>> {
    match dir {
        Some(p) => load_move_table::<TWIST_SIZE, TWIST_BYTES_SIZE>(p.join("move_twist").to_str().unwrap(), gen_twist_move_table),
        None => load_move_table::<TWIST_SIZE, TWIST_BYTES_SIZE>("move_twist", gen_twist_move_table),
    }
}

pub fn load_flip_move_table(dir: Option<&Path>) -> Result<Vec<u16>, Box<dyn Error>> {
    match dir {
        Some(p) => load_move_table::<FLIP_SIZE, FLIP_BYTES_SIZE>(p.join("move_flip").to_str().unwrap(), gen_flip_move_table),
        None => load_move_table::<FLIP_SIZE, FLIP_BYTES_SIZE>("move_flip", gen_flip_move_table),
    }
}

pub fn load_ud_move_table(dir: Option<&Path>) -> Result<Vec<u16>, Box<dyn Error>> {
    match dir {
        Some(p) => load_move_table::<UD_SIZE, UD_BYTES_SIZE>(p.join("move_slice_sorted").to_str().unwrap(), gen_ud_move_table),
        None => load_move_table::<UD_SIZE, UD_BYTES_SIZE>("move_slice_sorted", gen_ud_move_table),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Return an empty array to ensure the test fails
    fn zoinks<T, const S: usize>() -> Vec<T> where T: Default + Copy {
        vec![T::default(); S]
    }

    #[test]
    fn penis() {
        //load_move_table::<UD_SIZE, UD_BYTES_SIZE>("move_slice_sorted", gen_ud_move_table);
        let flips = load_ud_move_table(None).unwrap();
        //let test = gen_ud_move_table();
    }

    /// Ensure that files are saved and loaded with the same data
    #[test]
    fn file_saves_and_loads() {
        let _ = load_twist_move_table(None).unwrap();
        assert_eq!(load_twist_move_table(None).unwrap(), gen_twist_move_table());
        let _ = load_flip_move_table(None).unwrap();
        assert_eq!(load_flip_move_table(None).unwrap(), gen_flip_move_table());
        let _ = load_ud_move_table(None).unwrap();
        assert_eq!(load_ud_move_table(None).unwrap(), gen_ud_move_table());
    }

    // Compare the twist data to a known good and ensure they match
    #[test]
    fn twist_file_correct() {
        let twists = gen_twist_move_table();
        let good_twists = load_move_table::<TWIST_SIZE, TWIST_BYTES_SIZE>("known_good/move_twist", gen_twist_move_table).unwrap();
        assert_eq!(twists, good_twists);
    }

    // Compare the flip data to a known good and ensure they match
    #[test]
    fn flip_file_correct() {
        let flips = gen_flip_move_table();
        let good_flips = load_move_table::<FLIP_SIZE, FLIP_BYTES_SIZE>("known_good/move_flip", gen_flip_move_table).unwrap();
        assert_eq!(good_flips, flips);
    }

    #[test]
    fn slice_sorted_file_correct() {
        let slice_sorted = gen_ud_move_table();
        let good_flips = load_move_table::<UD_SIZE, UD_BYTES_SIZE>("known_good/move_slice_sorted", gen_ud_move_table).unwrap();
        assert_eq!(good_flips, slice_sorted);
    }
}
