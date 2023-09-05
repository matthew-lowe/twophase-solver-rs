use strum::IntoEnumIterator;
use crate::{common::{N_TWIST, N_MOVE, Color}, cubie::{CubieCube, BASIC_MOVES}};

// Store move tables that describe what happens to coordinats when moves happen

pub fn move_twist() {
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

    println!("{:?}", &twist_move);
    println!("ngfdguydfgbdf");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_move_twists() {
        move_twist();
    }
}
