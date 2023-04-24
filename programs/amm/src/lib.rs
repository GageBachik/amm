use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Data Logics
#[program]
pub mod amm {
    use super::*;

    pub fn create_config(ctx: Context<CreateConfig>, fee: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let authority = &mut ctx.accounts.authority;
        let treasury = &mut ctx.accounts.treasury;

        config.authority = authority.key();
        config.treasury = treasury.key();
        config.fee = fee;
        config.bump = *ctx.bumps.get("config").unwrap();

        Ok(())
    }

    pub fn create_pool(ctx: Context<CreatePool>, init_liquidity: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let mint_one = &mut ctx.accounts.mint_one;
        let mint_two = &mut ctx.accounts.mint_two;

        pool.mint_one = mint_one.key();
        pool.mint_two = mint_two.key();
        pool.token_one = 0;
        pool.token_two = 0;
        pool.bump = *ctx.bumps.get("pool").unwrap();

        // Potentially pool this out into a utility funciton to also be used in deposit
        // TODO: Tranfer initial liquidity amount to pool + small amount for slippage
        // ex
        // anchor_spl::token::transfer(
        //     CpiContext::new_with_signer(
        //         token_program.to_account_info(),
        //         anchor_spl::token::Transfer {
        //             from: wager_vault_token_account.to_account_info(),
        //             to: treasury_token_account.to_account_info(),
        //             authority: wager_vault_token_account.to_account_info(),
        //         },
        //         &[&[wager.key().as_ref(), &[wager.vault_bump]]],
        //     ),
        //     total_fee,
        // )?;

        Ok(())
    }
    // pub fn deposit_pool(ctx: Context<DepositPool>) -> Result<()> {
    //     Ok(())
    // }
    // pub fn withdraw_pool(ctx: Context<WithdawPool>) -> Result<()> {
    //     Ok(())
    // }
    // pub fn swap_pool(ctx: Context<SwapPool>) -> Result<()> {
    //     Ok(())
    // }
}

// Data Validators
#[derive(Accounts)]
pub struct CreateConfig<'info> {
    #[account(init, seeds = [b"config"], bump, payer = authority, space = 8 + Config::INIT_SPACE)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: passing in our own treasury on creation
    pub treasury: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(seeds = [b"pool"], bump = config.bump, has_one = authority)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, seeds = [mint_one.key().as_ref(), mint_two.key().as_ref()], bump, payer = authority, space = 8 + Pool::INIT_SPACE)]
    pub pool: Account<'info, Pool>,
    pub mint_one: Account<'info, Mint>,
    pub mint_two: Account<'info, Mint>,
    #[account(init, seeds = [pool.key().as_ref(), mint_one.key().as_ref()], bump, token::mint = mint_one, token::authority = pool_token_account_one, payer = authority)]
    pub pool_token_account_one: Account<'info, TokenAccount>,
    #[account(init, seeds = [pool.key().as_ref(), mint_two.key().as_ref()], bump, token::mint = mint_two, token::authority = pool_token_account_one, payer = authority)]
    pub pool_token_account_two: Account<'info, TokenAccount>,
    #[account(mut, associated_token::mint = mint_one, associated_token::authority = authority)]
    pub auth_token_account_one: Account<'info, TokenAccount>,
    #[account(mut, associated_token::mint = mint_two, associated_token::authority = authority)]
    pub auth_token_account_two: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// // TODO: Implement similiar feature to create for admins but also init a user deposit account
// #[derive(Accounts)]
// pub struct DepositPool<'info> {
//     #[account(seeds = [b"pool"], bump = config.bump)]
//     pub config: Account<'info, Config>,
//     #[account(mut)]
//     pub authority: Signer<'info>,
// }
// #[derive(Accounts)]

// pub struct WithdawPool<'info> {}
// #[derive(Accounts)]
// pub struct SwapPool<'info> {}

// Data Structs

// Config fee is in basis points
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub fee: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub mint_one: Pubkey,
    pub mint_two: Pubkey,
    pub token_one: u64,
    pub token_two: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Deposit {
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
