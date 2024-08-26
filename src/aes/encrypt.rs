use super::utils::add_round_key;
use super::utils::create_round_key;
use super::utils::glsmult;
use super::utils::{self};
pub fn encrypt(input_vec: &mut Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let rounds: u8 = match key.len() {
        16 => 10,
        24 => 12,
        32 => 14,
        _ => panic!("unexpected error with key length."),
    };

    let expanded_key: Vec<u8> = utils::expand_key(&key);

    ////padding////
    //The padding scheme is explained in aes/decrypt.rs:remove_padding()
    let mut padding = 16 - (input_vec.len() % 16);
    if padding == 16 {
        padding = 0;
    }

    for _ in 0..padding {
        input_vec.push(padding as u8);
    }
    ////end-padding////

    let num_blocks = input_vec.len() / 16;
    let mut total_output: Vec<u8> = vec![];
    for block_num in 0..num_blocks {
        let start_pos = block_num * 16;
        let mut column_block: Vec<u8> = vec![0; 16];
        for i in 0..4 {
            for j in 0..4 {
                column_block[i + (j * 4)] = input_vec[start_pos + (i * 4) + j];
            }
        }
        column_block = encrypt_main(column_block, rounds, &expanded_key);
        let mut block_result: Vec<u8> = vec![0; 16];
        for i in 0..4 {
            for j in 0..4 {
                block_result[(i * 4) + j] = column_block[i + (j * 4)];
            }
        }
        total_output.extend(block_result);
    }
    return total_output;
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
