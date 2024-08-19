use super::utils::add_round_key;
use super::utils::create_round_key;
use super::utils::glsmult;
use super::utils::KeyLength;
use super::utils::{self};

pub fn decrypt(key_length: &KeyLength, input_vec: Vec<u8>) -> String {
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

    let expanded_key: Vec<u8> = expand_key(&byte_key);

    column_block = decrypt_main(column_block, rounds, &expanded_key);
    let mut dec_result: Vec<u8> = vec![0; 16];
    for i in 0..4 {
        for j in 0..4 {
            dec_result[(i * 4) + j] = column_block[i + (j * 4)];
        }
    }
    return String::from_utf8(dec_result.clone()).unwrap();
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
    word[0] = word[0] ^ utils::get_rcon_val(iteration as u8)
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
        if cur_size % base_key.len() == 0 {
            key_core(&mut temp, rcon_iter);
            rcon_iter += 1;
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
    }

    return expanded_key;
}
