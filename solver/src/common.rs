use strum_macros::EnumIter;

// Consts
type C = Color;
pub const CORNER_COLOR: [[Color; 3]; 8] = [
    [C::U, C::R, C::F], [C::U, C::F, C::L], [C::U, C::L, C::B], [C::U, C::B, C::R],
    [C::D, C::F, C::R], [C::D, C::L, C::F], [C::D, C::B, C::L], [C::D, C::R, C::B],
]; 

pub const EDGE_COLOR: [[Color; 2]; 12] = [
    [C::U, C::R], [C::U, C::F], [C::U, C::L], [C::U, C::B], [C::D, C::R], [C::D, C::F],
    [C::D, C::L], [C::D, C::B], [C::F, C::R], [C::F, C::L], [C::B, C::L], [C::B, C::R],
];

type F = Facelet;
pub const CORNER_FACELET: [[Facelet; 3]; 8] = [
    [F::U9, F::R1, F::F3], [F::U7, F::F1, F::L3], [F::U1, F::L1, F::B3], [F::U3, F::B1, F::R3],
    [F::D3, F::F9, F::R7], [F::D1, F::L9, F::F7], [F::D7, F::B9, F::L7], [F::D9, F::R9, F::B7],
];

pub const EDGE_FACELET: [[Facelet; 2]; 12] = [
    [F::U6, F::R2], [F::U8, F::F2], [F::U4, F::L2], [F::U2, F::B2], [F::D6, F::R8], [F::D2, F::F8],
    [F::D4, F::L8], [F::D8, F::B8], [F::F6, F::R4], [F::F4, F::L6], [F::B6, F::L4], [F::B4, F::R6],
];

pub const N_MOVE: usize = 18; // Possible face moves
pub const N_TWIST: usize = 2187; // Possible corner orientations, 3^7 (ignore 1 corner)
pub const N_FLIP: usize = 2048; // Possible edge flips, 2^11 (ignore 1 edge)
pub const N_SLICE_SORTED: usize = 11880; // Possible variations of the UD slice


// Enums
#[derive(Clone, Copy, EnumIter)]
pub enum Color {
    U = 0,
    R,
    F,
    D,
    L,
    B,
}

// Done in order U/D, L/R, F/B, pretty arbitrary though
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Corner {
    UFR = 0,
    UFL,
    UBL,
    UBR,
    DFR,
    DFL,
    DBL,
    DBR,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Edge {
    INV = -1,
    UR = 0,
    UF,
    UL,
    UB,
    DR,
    DF,
    DL,
    DB,
    FR,
    FL,
    BL,
    BR,
}

// Matches FaceCube order
#[derive(Clone, Copy)]
pub enum Facelet {
    U1 = 0,
    U2,
    U3,
    U4,
    U5,
    U6,
    U7,
    U8,
    U9,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
}

