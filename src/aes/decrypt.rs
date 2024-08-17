// use super::utils;
// use super::utils::glsmult;

// pub fn decrypt() {}

// fn inv_sub_bytes(state: &mut Vec<u8>) {
//     for val in state.iter_mut() {
//         *val = utils::get_inverse_sbox_val(*val);
//     }
// }

// fn shift_rows(state: &mut Vec<u8>) {
//     //this is for encryption operation
//     for cur_row in 1..4 {
//         shift_row_right(state, cur_row);
//     }
// }

// fn shift_row_right(state: &mut Vec<u8>, row: u8) {
//     if row == 1 {
//         return;
//     }
//     let start = ((row - 1) * 4) as usize;
//     for _ in 0..3 {
//         let temp: u8 = state[start];
//         for j in 3..0 {
//             state[j] = state[j - 1];
//         }
//         state[start + 3] = temp;
//     }
// }

// fn inv_mix_columns(state: &mut Vec<u8>) {
//     let mut col: [u8; 4] = [0; 4];

//     for i in 0..4 {
//         for j in 0..4 {
//             col[j] = state[(j * 4) + i];
//         }
//         inv_mix_col(&mut col);
//         for k in 0..4 {
//             state[(k * 4) + i] = col[k];
//         }
//     }
// }

// fn inv_mix_col(col: &mut [u8; 4]) {
//     let copy: [u8; 4] = col.clone();
//     col[0] =
//         glsmult(copy[0], 14) ^ glsmult(copy[3], 9) ^ glsmult(copy[2], 13) ^ glsmult(copy[1], 11);
//     col[1] =
//         glsmult(copy[1], 14) ^ glsmult(copy[0], 9) ^ glsmult(copy[3], 13) ^ glsmult(copy[2], 11);
//     col[2] =
//         glsmult(copy[2], 14) ^ glsmult(copy[1], 9) ^ glsmult(copy[0], 13) ^ glsmult(copy[3], 11);
//     col[3] =
//         glsmult(copy[3], 14) ^ glsmult(copy[2], 9) ^ glsmult(copy[1], 13) ^ glsmult(copy[0], 11);
// }

// fn decryption_round(state: &mut Vec<u8>, round_key: &Vec<u8>) {
//     inv_shift_rows(state);
//     inv_sub_bytes(state);
//     add_round_key(state, round_key);
//     inv_mix_col(state);
// }
