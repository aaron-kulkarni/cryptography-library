use super::utils::add_round_key;
use super::utils::create_round_key;
use super::utils::glsmult;
use super::utils::KeyLength;
use super::utils::{self};
pub fn encrypt(key_length: &KeyLength, input: String) -> Vec<u8> {
    let input_vec: Vec<u8> = input.as_bytes().to_vec();
    let mut column_block: Vec<u8> = vec![0; 16];
    for i in 0..4 {
        for j in 0..4 {
            column_block[i + (j * 4)] = input_vec[(i * 4) + j];
        }
    }

    let rounds: u8 = match key_length {
        KeyLength::Len16 => 10,
        KeyLength::Len24 => 12,
        KeyLength::Len32 => 14,
    };

    //TODO: replace this with a randomly generated key
    let char_key: Vec<char> = vec![
        'k', 'k', 'k', 'k', 'e', 'e', 'e', 'e', 'y', 'y', 'y', 'y', '.', '.', '.', '.',
    ];
    let byte_key: Vec<u8> = char_key.iter().map(|c| *c as u8).collect::<Vec<_>>();

    let expanded_key: Vec<u8> = utils::expand_key(&byte_key);

    column_block = encrypt_main(column_block, rounds, &expanded_key);
    let mut enc_result: Vec<u8> = vec![0; 16];
    for i in 0..4 {
        for j in 0..4 {
            enc_result[(i * 4) + j] = column_block[i + (j * 4)];
        }
    }
    return enc_result;
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
    println!("After final round, state is length {}", state.len());
}

fn sub_bytes(state: &mut Vec<u8>) {
    for val in state.iter_mut() {
        *val = utils::get_sbox_val(*val);
    }
}

fn shift_rows(state: &mut Vec<u8>) {
    for cur_row in 1..4 {
        shift_row_left(state, cur_row);
    }
}

fn shift_row_left(state: &mut Vec<u8>, row: u8) {
    let start = (row * 4) as usize;
    for _ in 0..row {
        let temp: u8 = state[start];
        for j in 0..3 {
            state[start + j] = state[start + j + 1];
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
