use super::utils;
use super::utils::KeyLength;

pub fn encrypt(key_length: KeyLength, input: [u8; 16]) -> [u8; 16] {
    let rounds: u8 = match key_length {
        KeyLength::Bits128 => 10,
        KeyLength::Bits192 => 12,
        KeyLength::Bits256 => 14,
    };

    let expanded_key_size = 16 * (rounds + 1);
    let expanded_key = expand_key(&input);
    encrypt_main(input, expanded_key, rounds);
}

fn encrypt_main(mut state: [u8; 16], expanded_key: [u8; N], rounds: u8) {
    let round_key = create_round_key(expanded_key[0..16]);

    for i in 1..rounds + 1 {
        let round_key = create_round_key(expanded_key[16 * i..(16 * i) + 1]);
        if i != rounds {
            encryption_round(&mut state, &mut round_key);
        } else {
            final_encryption_round(&mut state, &mut round_key);
        }
    }
}

fn encryption_round(state: &mut [u8; 16], round_key: &mut [u8; 16]) {
    sub_bytes(state);
    shift_rows(state);
    mix_columns(state);
    add_round_key(state, round_key);
}

fn final_encryption_round(state: &mut [u8; 16], round_key: &mut [u8; 16]) {
    sub_bytes(state);
    shift_rows(state);
    add_round_key(state, round_key);
}

fn create_round_key<const N: usize>(expanded_key: &[u8; N]) -> [u8; 16]
where
    [(); N]: Sized,
{
    let mut round_key: [u8; 16] = [0; 16];
    for i in 0..4 {
        for j in 0..4 {
            round_key[i + (j * 4)] = expanded_key[(i * 4) + j];
        }
    }
    return round_key;
}

fn add_round_key(state: &mut [u8; 16], round_key: &[u8; 16]) {
    for (i, val) in state.iter_mut().enumerate() {
        *val ^= round_key[i];
    }
}

fn sub_bytes(state: &mut [u8; 16]) {
    for val in state.iter_mut() {
        *val = utils::get_sbox_val(*val);
    }
}

fn shift_rows(state: &mut [u8; 16]) {
    //this is for encryption operation
    for cur_row in 1..4 {
        shift_row_left(state, cur_row);
    }
}

fn shift_row_left(state: &mut [u8; 16], row: u8) {
    if row == 1 {
        return;
    }
    let start = ((row - 1) * 4) as usize;
    for _ in 0..3 {
        let temp: u8 = state[start];
        for j in 0..3 {
            state[j] = state[j + 1];
        }
        state[start + 3] = temp;
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

fn rotate(word: &mut [u8; 4]) {
    let temp: u8 = word[0];
    for j in 0..3 {
        word[j] = word[j + 1];
    }
    word[3] = temp;
}

fn key_core(word: &mut [u8; 4], iteration: usize) {
    rotate(word);
    for i in 0..4 {
        word[i] = utils::get_sbox_val(word[i]);
    }
    word[0] = word[0] ^ utils::get_rcon_val(word[iteration])
}

/////////////////////////// KEY EXPANSION //////////////////////////////

fn expand_key<const N: usize, const M: usize>(base_key: &[u8; N]) -> [u8; M]
where
    [(); N]: Sized,
{
    let mut cur_size: usize = 0;
    let mut rcon_iter: usize = 1;
    let mut temp: [u8; 4] = [0; 4];
    let mut expanded_key: [u8; M] = [0; M];

    //copy over base key into expanded
    for i in 0..N {
        expanded_key[i] = base_key[i];
    }
    cur_size += N;

    while cur_size < M {
        for k in 0..4 {
            temp[k] = expanded_key[(cur_size - 4) + k];
        }
    }

    if cur_size % N == 0 {
        rcon_iter += 1;
        key_core(&mut temp, rcon_iter);
    }

    if N == 32 && cur_size % N == 16 {
        for m in 0..4 {
            temp[m] = utils::get_sbox_val(temp[m]);
        }
    }

    for a in 0..4 {
        expanded_key[cur_size] = expanded_key[cur_size - N] ^ temp[a];
        cur_size += 1;
    }

    return expanded_key;
}
