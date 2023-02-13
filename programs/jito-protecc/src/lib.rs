pub mod sdk;

use std::mem::size_of;

use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// The pre/post guard instructions should be separate transactions or instructions wrapping the inner contents of a bundle or transaction.
#[program]
pub mod jito_protecc {
    use super::*;

    pub fn close_guarded_state(_ctx: Context<CloseGuardedState>) -> Result<()> {
        Ok(())
    }

    pub fn pre_guard(
        ctx: Context<PreGuard>,
        // If true, then performs a negative balance change check on the guarded account's lamports.
        should_guard_lamports: bool,
        bump: u8,
    ) -> Result<()> {
        let guarded_state = &mut ctx.accounts.guarded_state;

        if should_guard_lamports {
            guarded_state.maybe_pre_lamports = Some(ctx.accounts.guarded_account.lamports());
        }

        guarded_state.spl_token_state = SplTokenState {
            mint: ctx.accounts.token_account.mint,
            pre_balance: ctx.accounts.token_account.amount,
        };

        guarded_state.bump = bump;

        Ok(())
    }

    pub fn post_guard(ctx: Context<PostGuard>) -> Result<()> {
        if let Some(pre_lamports) = ctx.accounts.guarded_state.maybe_pre_lamports {
            if ctx.accounts.guarded_account.lamports() < pre_lamports {
                return Err(Error::AnchorError(AnchorError {
                    error_name: "pre_lamports guard failure".to_string(),
                    error_code_number: 69,
                    error_msg: format!(
                        "negative balance change: pre_lamports: {pre_lamports}, post_lamports: {}",
                        ctx.accounts.guarded_account.lamports(),
                    ),
                    error_origin: None,
                    compared_values: None,
                }));
            }
        }

        if ctx.accounts.token_account.amount
            < ctx.accounts.guarded_state.spl_token_state.pre_balance
        {
            Err(Error::AnchorError(AnchorError {
                error_name: "spl_token_state guard failure".to_string(),
                error_code_number: 69,
                error_msg: format!(
                    "negative balance change: pre_balance: {}, post_balance: {}",
                    ctx.accounts.guarded_state.spl_token_state.pre_balance,
                    ctx.accounts.token_account.amount,
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
pub struct CloseGuardedState<'info> {
    /// CHECK: We just care about the account's lamports.
    pub guarded_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            GuardedState::SEED,
            guarded_account.key().as_ref(),
            signer.key().as_ref(),
            token_account.mint.as_ref(),
        ],
        bump = guarded_state.bump,
        close = signer
    )]
    pub guarded_state: Account<'info, GuardedState>,

    pub token_account: Account<'info, TokenAccount>,

    /// Anyone can crank this instruction.
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct PreGuard<'info> {
    /// CHECK: We just care about the account's lamports.
    pub guarded_account: AccountInfo<'info>,

    #[account(
        init_if_needed,
        seeds = [
            GuardedState::SEED,
            guarded_account.key().as_ref(),
            signer.key().as_ref(),
            token_account.mint.as_ref(),
        ],
        bump,
        space = GuardedState::SIZE,
        payer = signer
    )]
    pub guarded_state: Account<'info, GuardedState>,

    #[account(constraint = token_account.owner == guarded_account.key())]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostGuard<'info> {
    /// CHECK: We just care about the account's lamports.
    pub guarded_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            GuardedState::SEED,
            guarded_account.key().as_ref(),
            signer.key().as_ref(),
            token_account.mint.as_ref(),
        ],
        bump = guarded_state.bump,
        close = signer
    )]
    pub guarded_state: Account<'info, GuardedState>,

    #[account(
        constraint = token_account.owner == guarded_state.key(),
        constraint = token_account.mint == guarded_state.spl_token_state.mint,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct GuardedState {
    /// Optionally check a negative balance change of the account's lamports.
    /// If set to None, then no balance change is checked.
    pub maybe_pre_lamports: Option<u64>,

    /// Specifies the account's owned SPL token balance.
    pub spl_token_state: SplTokenState,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct SplTokenState {
    pub mint: Pubkey,
    pub pre_balance: u64,
}

impl GuardedState {
    pub const SEED: &'static [u8] = b"GUARDED_STATE";
    pub const SIZE: usize = 8 + size_of::<Self>();
}
