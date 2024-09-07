use super::utils::add_round_key;
use super::utils::create_round_key;
use super::utils::glsmult;
use super::utils::{self};
use std::process;

pub fn decrypt(input_vec: Vec<u8>, key: Vec<u8>) -> String {
    let rounds: u8 = match key.len() {
        16 => 10,
        24 => 12,
        32 => 14,
        _ => panic!("unexpected error with key length."),
    };

    let expanded_key: Vec<u8> = utils::expand_key(&key);

    if input_vec.len() % 16 != 0 {
        panic!("Message was not encrypted properly. The encrypted message byte size is not divisible by 16.");
    }
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
        column_block = decrypt_main(column_block, rounds, &expanded_key);
        let mut block_result: Vec<u8> = vec![0; 16];
        for i in 0..4 {
            for j in 0..4 {
                block_result[(i * 4) + j] = column_block[i + (j * 4)];
            }
        }
        total_output.extend(block_result);
    }
    remove_padding(&mut total_output);
    return String::from_utf8(total_output.clone()).unwrap();
}

fn remove_padding(text: &mut Vec<u8>) {
    /* Padding scheme example:
       User passes in a plaintext string which is 20 bytes.
       We need to add 12 bytes so that the string length is divisible by 16.
       We add 12 bytes of the byte representation of 12.
       This way, it's clear to the decryption algorithm which bytes are padding
       and which are not.
    */

    let last_byte: usize = match text.last() {
        Some(a) => *a as usize,
        None => {
            println!("Decryption key seems to be invalid.");
            process::exit(1)
        }
    };
    //let last_byte = *text.last().ok_or("Failed to get last byte.")? as usize;

    if text
        .iter()
        .rev()
        .take(last_byte)
        .all(|&byte| byte == last_byte as u8)
    {
        text.truncate(text.len() - last_byte);
    }
}

fn decrypt_main(mut state: Vec<u8>, rounds: u8, expanded_key: &Vec<u8>) -> Vec<u8> {
    let round_key = create_round_key(&expanded_key[(16 * rounds) as usize..]);
    add_round_key(&mut state, &round_key);

    for i in (0..rounds).rev() {
        let start_idx: usize = 16 * i as usize;
        let end_idx: usize = 16 * (i + 1) as usize;
        let round_key = create_round_key(&expanded_key[start_idx..end_idx]);
        if i > 0 {
            decryption_round(&mut state, &round_key);
        } else {
            final_decryption_round(&mut state, &round_key);
        }
    }
    return state;
}

fn decryption_round(state: &mut Vec<u8>, round_key: &Vec<u8>) {
    inv_shift_rows(state);
    inv_sub_bytes(state);
    add_round_key(state, round_key);
    inv_mix_columns(state);
}

fn final_decryption_round(state: &mut Vec<u8>, round_key: &Vec<u8>) {
    inv_shift_rows(state);
    inv_sub_bytes(state);
    add_round_key(state, round_key);
}

fn inv_sub_bytes(state: &mut Vec<u8>) {
    for val in state.iter_mut() {
        *val = utils::get_inverse_sbox_val(*val);
    }
}

fn inv_shift_rows(state: &mut Vec<u8>) {
    for cur_row in 1..4 {
        shift_row_right(state, cur_row);
    }
}

fn shift_row_right(state: &mut Vec<u8>, row: u8) {
    let start = (row * 4) as usize;
    for _ in 0..row {
        let temp: u8 = state[start + 3];
        for j in (0..4).rev() {
            state[start + j] = state[start + j - 1];
        }
        state[start] = temp;
    }
}

fn inv_mix_columns(state: &mut Vec<u8>) {
    let mut col: [u8; 4] = [0; 4];

    for i in 0..4 {
        for j in 0..4 {
            col[j] = state[(j * 4) + i];
        }
        inv_mix_col(&mut col);
        for k in 0..4 {
            state[(k * 4) + i] = col[k];
        }
    }
}

fn inv_mix_col(col: &mut [u8; 4]) {
    let copy: [u8; 4] = col.clone();
    col[0] =
        glsmult(copy[0], 14) ^ glsmult(copy[3], 9) ^ glsmult(copy[2], 13) ^ glsmult(copy[1], 11);
    col[1] =
        glsmult(copy[1], 14) ^ glsmult(copy[0], 9) ^ glsmult(copy[3], 13) ^ glsmult(copy[2], 11);
    col[2] =
        glsmult(copy[2], 14) ^ glsmult(copy[1], 9) ^ glsmult(copy[0], 13) ^ glsmult(copy[3], 11);
    col[3] =
        glsmult(copy[3], 14) ^ glsmult(copy[2], 9) ^ glsmult(copy[1], 13) ^ glsmult(copy[0], 11);
}
