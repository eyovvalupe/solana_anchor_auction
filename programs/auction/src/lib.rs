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
) -> (u64, u64) {
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
) -> (u64) {
  let slope_start_time = (((slope_num as u128).checked_mul(start_time as u128).unwrap()) as u64)
        .checked_div(slope_den)
        .unwrap();
    start_price.checked_sub(slope_start_time).unwrap()
}

fn get_current_price(current_time: u64, y_intercept: u64, slope_num: u64, slope_den: u64) -> (u64) {
  let slope_cur_time = (((slope_num as u128)
        .checked_mul(current_time as u128)
        .unwrap()) as u64)
        .checked_div(slope_den)
        .unwrap();
    (slope_cur_time).checked_add(y_intercept).unwrap() as u64
}

#[program]
pub mod auction {
    use super::*;
    pub fn initialize(
        ctx: Context<Initailize>,
        mint_bump: u8,
        start_time: u64,
        end_time: u64,
        start_price: u64,
        reserve_price: Option<u64>
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.authority = ctx.accounts.authority.key();
        auction.start_time = start_time;
        auction.end_time = end_time;
        auction.start_price = start_price;
        auction.reserve_den = reserve_den;
        auction.is_ended = false;

        let (num, den) = get_slope(start_price, start_time, reserve_price, end_time);
        let y_intercept = get_y_intercept(start_price, start_time, num, den);

        auction.slope_num = num;
        auction.slope_den = den;
        auction.y_intercept = y_intercept;

        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.authoirty.to_account_info()
                },
                $[$[$[], $[mint_bump]]],
            ),
            100
        )?;

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        
    }
}

#[derive(Accounts)]
#[instruction(mint_bump: u8)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 64 + 64)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        init,
        payer = authority,
        seeds = [],
        bump,
        mint::decimals = 0,
        mint::authority = authority,
    )]
    pub mint: Account<'info, Mint>,

    pub rent: Syscar<'info, Token>,
    pub token_program: Program<'info, Token>,

    #[account(init_if_needed, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub destination: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub purchaser: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>
}

#[account]
pub struct Auction {
    pub authority: Pubkey,
    pub start_time: u64,
    pub end_time: u64,
    pub start_price: u64,
    pub reserve_den: u64,
    pub slope_num: u64,
    pub slope_den: u64,
    pub y_intercept: u64,
    pub is_ended: bool
}