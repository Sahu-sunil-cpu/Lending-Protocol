use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked}};
use std::f32::consts::E;
use crate::state::{Bank, User};



#[derive(Accounts)]
pub struct Withdraw<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [mint.key().as_ref()],
        bump,
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        mut,
        seeds = [b"treasury", mint.key().as_ref()],
        bump,
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [signer.key().as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}



pub fn process_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {

    let user = &mut ctx.accounts.user;

    let deposited_value;

   if ctx.accounts.mint.to_account_info().key() == user.usdc_address {
          deposited_value = user.deposited_usdc;
    }else {
        deposited_value = user.deposited_sol;
    }

  let bank = &mut ctx.accounts.bank;

  let time_diff = user.last_updated - Clock::get()?.unix_timestamp;

  bank.total_deposits = (bank.total_deposits as f64 * E.powf(bank.interest_rate as f32 * time_diff as f32) as f64) as u64;

    let transfer_cpi_accounts = TransferChecked{
        from: ctx.accounts.bank_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.bank_token_account.to_account_info()
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let mint_key = ctx.accounts.mint.key();

    let signer_seeds: &[&[&[u8]]] = &[
        &[
            b"treasury",
            mint_key.as_ref(),
            &[ctx.bumps.bank_token_account]
        ]
    ];


    let cpi_ctx = CpiContext::new(cpi_program, transfer_cpi_accounts)
    .with_signer(signer_seeds);

    let decimals = ctx.accounts.mint.decimals;

    token_interface::transfer_checked(cpi_ctx, amount, decimals)?;

    let bank = &mut ctx.accounts.bank;

    let share_to_remove = (amount as f64/ bank.total_deposits as f64 ) * bank.total_deposit_shares as f64;

    let user  = &mut ctx.accounts.user;

    if ctx.accounts.mint.to_account_info().key() == user.usdc_address {
        user.borrowed_usdc += amount;
        user.borrowed_usdc_shares += share_to_remove as u64;
    }else {
        user.borrowed_sol += amount;
        user.borrowed_sol_shares += share_to_remove as u64;
    }

    bank.total_deposits -= amount;
    bank.total_deposit_shares -= share_to_remove as u64;

    Ok(())
}