pub mod sdk;

use std::mem::size_of;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// The pre/post guard instructions should be separate transactions or instructions wrapping the inner contents of a bundle or transaction.
#[program]
pub mod jito_protecc {
    use super::*;

    pub fn close_guarded_state(_ctx: Context<CloseGuardedState>) -> Result<()> {
        Ok(())
    }

    pub fn pre_guard(ctx: Context<PreGuard>, bump: u8) -> Result<()> {
        let guarded_state = &mut ctx.accounts.guarded_state;
        guarded_state.lamports = ctx.accounts.guarded_account.lamports();
        guarded_state.bump = bump;

        Ok(())
    }

    pub fn post_guard(ctx: Context<PostGuard>) -> Result<()> {
        if ctx.accounts.guarded_account.lamports() < ctx.accounts.guarded_state.lamports {
            Err(Error::AnchorError(AnchorError {
                error_name: "guard failure".to_string(),
                error_code_number: 69,
                error_msg: format!(
                    "negative balance change: pre lamports: {}, post lamports: {}",
                    ctx.accounts.guarded_account.lamports(),
                    ctx.accounts.guarded_state.lamports
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
        ],
        bump = guarded_state.bump,
        close = signer
    )]
    pub guarded_state: Account<'info, GuardedState>,

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
        ],
        bump,
        space = GuardedState::SIZE,
        payer = signer
    )]
    pub guarded_state: Account<'info, GuardedState>,

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
        ],
        bump = guarded_state.bump,
        close = signer
    )]
    pub guarded_state: Account<'info, GuardedState>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct GuardedState {
    pub lamports: u64,
    pub bump: u8,
}

impl GuardedState {
    pub const SEED: &'static [u8] = b"GUARDED_STATE";
    pub const SIZE: usize = 8 + size_of::<Self>();
}
