use super::utils::KeyLength;
use super::utils::{self};

pub fn encrypt(key_length: KeyLength, input: String) -> String {
    let input_vec: Vec<u8> = input.as_bytes().to_vec();

    let rounds: u8 = match key_length {
        KeyLength::Len16 => 10,
        KeyLength::Len24 => 12,
        KeyLength::Len32 => 14,
    };

    let expanded_key: Vec<u8> = expand_key(&input_vec);
    let temp_result = encrypt_main(input_vec, rounds, &expanded_key);
    return String::from_utf8(temp_result).expect("Some other error message");
}

fn encrypt_main(mut state: Vec<u8>, rounds: u8, expanded_key: &Vec<u8>) -> Vec<u8> {
    let round_key = create_round_key(&expanded_key[0..16]);
    add_round_key(&mut state, &round_key);

    for i in 1..rounds + 1 {
        let start_idx: usize = 16 * i as usize;
        let end_idx: usize = 16 * (i + 1) as usize;
        let round_key = create_round_key(&expanded_key[start_idx..end_idx]);
        if i != rounds {
            encryption_round(&mut state, &round_key);
        } else {
            final_encryption_round(&mut state, &round_key);
        }
    }
    return state;
}

fn encryption_round(state: &mut Vec<u8>, round_key: &Vec<u8>) {
    sub_bytes(state);
    shift_rows(state);
    mix_columns(state);
    add_round_key(state, round_key);
}

fn final_encryption_round(state: &mut Vec<u8>, round_key: &Vec<u8>) {
    sub_bytes(state);
    shift_rows(state);
    add_round_key(state, round_key);
}

fn create_round_key(expanded_key: &[u8]) -> Vec<u8> {
    let mut round_key: Vec<u8> = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            round_key[i + (j * 4)] = expanded_key[(i * 4) + j];
        }
    }
    return round_key;
}

fn add_round_key(state: &mut Vec<u8>, round_key: &Vec<u8>) {
    for (i, val) in state.iter_mut().enumerate() {
        *val ^= round_key[i];
    }
}

fn sub_bytes(state: &mut Vec<u8>) {
    for val in state.iter_mut() {
        *val = utils::get_sbox_val(*val);
    }
}

fn shift_rows(state: &mut Vec<u8>) {
    //this is for encryption operation
    for cur_row in 1..4 {
        shift_row_left(state, cur_row);
    }
}

fn shift_row_left(state: &mut Vec<u8>, row: u8) {
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

fn mix_columns(state: &mut Vec<u8>) {
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
        hi_bit = a & 0x80;
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

fn expand_key(base_key: &Vec<u8>) -> Vec<u8> {
    let mut cur_size: usize = 0;
    let mut rcon_iter: usize = 1;
    let mut temp: [u8; 4] = [0; 4];

    let expanded_key_len: usize = match base_key.len() {
        16 => 176,
        24 => 208,
        32 => 240,
        _ => panic!("what is going on"),
    };

    let mut expanded_key = vec![0; expanded_key_len];

    //copy over base key into expanded
    for i in 0..base_key.len() {
        expanded_key[i] = base_key[i];
    }
    cur_size += base_key.len();

    while cur_size < expanded_key_len {
        for k in 0..4 {
            temp[k] = expanded_key[(cur_size - 4) + k];
        }
        //INFINITE LOOP HERE. FIX
    }

    if cur_size % base_key.len() == 0 {
        rcon_iter += 1;
        key_core(&mut temp, rcon_iter);
    }

    if base_key.len() == 32 && cur_size % base_key.len() == 16 {
        for m in 0..4 {
            temp[m] = utils::get_sbox_val(temp[m]);
        }
    }

    for a in 0..4 {
        expanded_key[cur_size] = expanded_key[cur_size - base_key.len()] ^ temp[a];
        cur_size += 1;
    }

    return expanded_key;
}
