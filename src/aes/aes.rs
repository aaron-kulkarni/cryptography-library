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
        return;
    }
}

fn encryption_round(_state: [[char; 16]; 16]) {
    // sub_bytes();
    // shift_row_left();
    // mix_column();
    // add_round_key();

    return;
}

pub fn decrypt() {}

fn add_round_key(state: &mut [char; 16], round_key: &[char; 16]) {
    //in-place edit of state.
    // for (i, row) in state.iter_mut().enumerate() {
    //     for (y, col) in row.iter_mut().enumerate() {
    //         col = col ^ round_key[i][y];
    //     }
    // }

    for (i, val) in state.iter_mut().enumerate() {
        *val = utils::xor_chars(*val, round_key[i]);
    }
}

fn shift_rows(state: &mut [char; 16], left: bool) {
    let mut cur_row = 1;
    if left == true {
        loop {
            if cur_row == 4 {
                break;
            }
            shift_row_left(state, cur_row);
            cur_row += 1;
        }
    } else {
        loop {
            if cur_row == 4 {
                break;
            }
            // shift_row_right(state, curRow);
            cur_row += 1;
        }
    }
}

fn shift_row_left(state: &mut [char; 16], row: u8) {
    if row == 1 {
        return;
    }
    let start = ((row - 1) * 4) as usize;
    let mut shifts = 0;
    loop {
        if shifts == 3 {
            break;
        }
        let temp: char = state[start];
        for j in 0..3 {
            state[j] = state[j + 1];
        }
        state[3] = temp;
        shifts += 1;
    }
}
