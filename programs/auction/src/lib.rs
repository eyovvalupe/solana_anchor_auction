use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!(" ")

fn get_slope(
    start_price: u64,
    start_time: u64,
    reserve_price: Option<u64>,
    end_time: u64
) -> Result<(u64, u64)> {
    let num = (reserve_price.or(Some(0)).unwarp())
        .checked_sub(start_price)
        .unwrap();
    let den = end_time.checked_sub(start_time).unwrap();
    (num, den)
}

fn get_y_intercept(
    start_price: u64,
    start_time: u64,
    slope_num: u64,
    slope_den: u64
) -> Result<(u64)> {
    let slope_start_time = (((slope_num as u128).checked_mul(start_time as u128).unwrap()) as u64)
        .checked_div(slope_den)
        .unwrap();
    start_price.checked_sub(slope_start_time).unwrap()
}

fn get_current_price(current_time: u64, y_intercept: u64, slope_num: u64, slope_den: u64) -> Result<(u64)> {
    let slope_cur_time = (((slope_num as u128)
        .checked_mul(current_time as u128)
        .unwrap()) as u64)
        .checked_div(slope_den)
        .unwrap();
    (slope_cur_time).checked_add(y_intercept).unwrap() as u64
}

