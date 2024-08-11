use super::utils;

pub fn encrypt(key_length: u16) {
    //for every block of the plaintext input
    // key generation: generate a set of round keys from a secret key
    //addition of first round key to input
    //series of rounds, defined by length of secret key
    //1. subsitute bytes
    //2. shift rows
    //3. mix columns
    //4. adding round key
    //final round: skip mix columns step
    // key_expansion();
    // add_round_key();
    let rounds: u8 = match key_length {
        128 => 10,
        192 => 12,
        256 => 14,
        _ => 0,
    };

    if rounds == 0 {
        //throw error, panic, whatever.
    }
}

fn encryption_round(_state: [[u8; 16]; 16]) {
    // sub_bytes();
    // shift_row_left();
    // mix_column();
    // add_round_key();
}

pub fn decrypt() {}

fn add_round_key(state: &mut [u8; 16], round_key: &[u8; 16]) {
    //in-place edit of state.
    // for (i, row) in state.iter_mut().enumerate() {
    //     for (y, col) in row.iter_mut().enumerate() {
    //         col = col ^ round_key[i][y];
    //     }
    // }

    for (i, val) in state.iter_mut().enumerate() {
        *val ^= round_key[i];
    }
}

fn sub_bytes(state: &mut [u8; 16]) {
    for val in state.iter_mut() {
        *val = utils::get_sbox_val(*val);
    }
}

fn shift_rows(state: &mut [u8; 16], left: bool) {
    if left {
        for cur_row in 1..4 {
            shift_row_left(state, cur_row);
        }
    } else {
        for cur_row in 1..4 {
            //shift_row_right(state, cur_row);
        }
    }
}

fn shift_row_left(state: &mut [u8; 16], row: u8) {
    if row == 1 {
        return;
    }
    let start = ((row - 1) * 4) as usize;
    let mut shifts = 0;
    loop {
        if shifts == 3 {
            break;
        }
        let temp: u8 = state[start];
        for j in 0..3 {
            state[j] = state[j + 1];
        }
        state[3] = temp;
        shifts += 1;
    }
}

fn mix_columns(state: &mut [u8; 16]) {
    let mut col: [u8; 4] = [0; 4];

    for i in 0..4 {
        for j in 0..4 {
            col[j] = state[(j * 4) + i];
        }
        mix_col(&mut col);
        for k in 0..4 {
            state[(k * 4) + i] = col[k];
        }
    }
}

fn mix_col(col: &mut [u8; 4]) {
    let copy: [u8; 4] = col.clone();
    col[0] = glsmult(copy[0], 2) ^ glsmult(copy[3], 1) ^ glsmult(copy[2], 1) ^ glsmult(copy[1], 3);
    col[1] = glsmult(copy[1], 2) ^ glsmult(copy[0], 1) ^ glsmult(copy[3], 1) ^ glsmult(copy[2], 3);
    col[2] = glsmult(copy[2], 2) ^ glsmult(copy[1], 1) ^ glsmult(copy[0], 1) ^ glsmult(copy[3], 3);
    col[3] = glsmult(copy[3], 2) ^ glsmult(copy[2], 1) ^ glsmult(copy[1], 1) ^ glsmult(copy[0], 3);
}

fn glsmult(mut a: u8, mut b: u8) -> u8 {
    //Galois multiplication
    let mut p = 0;
    let mut hi_bit;

    for _ in 0..8 {
        if b & 1 == 1 {
            p ^= a;
        }
        hi_bit = (a & 0x80);
        a <<= 1;
        if hi_bit == 0x80 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    return p;
}
