pub mod sdk;

use std::mem::size_of;

use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// The pre/post guard instructions should be separate transactions or instructions wrapping the inner contents of a bundle or transaction.
#[program]
pub mod jito_protecc {
    use super::*;

    pub fn close_token_guarded_state(_ctx: Context<CloseTokenGuardedState>) -> Result<()> {
        Ok(())
    }

    pub fn pre_token_guard(ctx: Context<PreTokenGuard>, bump: u8) -> Result<()> {
        let token_guarded_state = &mut ctx.accounts.token_guarded_state;
        token_guarded_state.token_mint = ctx.accounts.token_account.mint;
        token_guarded_state.pre_balance = ctx.accounts.token_account.amount;
        token_guarded_state.bump = bump;

        Ok(())
    }

    pub fn post_token_guard(ctx: Context<PostTokenGuard>) -> Result<()> {
        if ctx.accounts.token_account.amount < ctx.accounts.token_guarded_state.pre_balance {
            Err(Error::AnchorError(AnchorError {
                error_name: "spl_token_state guard failure".to_string(),
                error_code_number: 69,
                error_msg: format!(
                    "negative balance change: pre_balance: {}, post_balance: {}",
                    ctx.accounts.token_guarded_state.pre_balance, ctx.accounts.token_account.amount,
                ),
                error_origin: None,
                compared_values: None,
            }))
        } else {
            Ok(())
        }
    }
}

#[derive(Accounts)]
pub struct CloseTokenGuardedState<'info> {
    #[account(
        mut,
        seeds = [
            TokenGuardedState::SEED,
            token_account.key().as_ref(),
            signer.key().as_ref(),
        ],
        bump = token_guarded_state.bump,
        close = signer
    )]
    pub token_guarded_state: Account<'info, TokenGuardedState>,

    pub token_account: Account<'info, TokenAccount>,

    /// Anyone can crank this instruction.
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct PreTokenGuard<'info> {
    #[account(
        init_if_needed,
        seeds = [
            TokenGuardedState::SEED,
            token_account.key().as_ref(),
            signer.key().as_ref(),
        ],
        bump,
        space = TokenGuardedState::SIZE,
        payer = signer
    )]
    pub token_guarded_state: Account<'info, TokenGuardedState>,

    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostTokenGuard<'info> {
    #[account(
        mut,
        seeds = [
            TokenGuardedState::SEED,
            token_account.key().as_ref(),
            signer.key().as_ref(),
        ],
        bump = token_guarded_state.bump,
        close = signer
    )]
    pub token_guarded_state: Account<'info, TokenGuardedState>,

    #[account(
        constraint = token_account.mint == token_guarded_state.token_mint,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct TokenGuardedState {
    pub token_mint: Pubkey,
    pub pre_balance: u64,
    pub bump: u8,
}

impl TokenGuardedState {
    pub const SEED: &'static [u8] = b"TOKEN_GUARDED_STATE";
    pub const SIZE: usize = 8 + size_of::<Self>();
}
