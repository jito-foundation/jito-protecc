use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};

pub struct PreGuardArgs {
    pub should_guard_lamports: bool,
    pub bump: u8,
}
pub struct PreGuardAccounts {
    pub guarded_account: Pubkey,
    pub guarded_state: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
    pub token_account: Pubkey,
}
pub fn pre_guard_ix(
    program_id: Pubkey,
    args: PreGuardArgs,
    accounts: PreGuardAccounts,
) -> Instruction {
    let PreGuardArgs {
        should_guard_lamports,
        bump,
    } = args;

    let PreGuardAccounts {
        guarded_account,
        guarded_state,
        signer,
        system_program,
        token_account,
    } = accounts;

    Instruction {
        program_id,
        data: crate::instruction::PreGuard {
            should_guard_lamports,
            bump,
        }
        .data(),
        accounts: crate::accounts::PreGuard {
            guarded_account,
            guarded_state,
            signer,
            system_program,
            token_account,
        }
        .to_account_metas(None),
    }
}

pub struct PostGuardArgs;
pub struct PostGuardAccounts {
    pub guarded_account: Pubkey,
    pub guarded_state: Pubkey,
    pub signer: Pubkey,
    pub token_account: Pubkey,
}
pub fn post_guard_ix(
    program_id: Pubkey,
    _args: PostGuardArgs,
    accounts: PostGuardAccounts,
) -> Instruction {
    let PostGuardAccounts {
        guarded_account,
        guarded_state,
        signer,
        token_account,
    } = accounts;

    Instruction {
        program_id,
        data: vec![],
        accounts: crate::accounts::PostGuard {
            guarded_account,
            guarded_state,
            signer,
            token_account,
        }
        .to_account_metas(None),
    }
}

pub struct CloseGuardedStateArgs;
pub struct CloseGuardedStateAccounts {
    pub guarded_account: Pubkey,
    pub guarded_state: Pubkey,
    pub signer: Pubkey,
    pub token_account: Pubkey,
}
pub fn close_guarded_state_ix(
    program_id: Pubkey,
    _args: CloseGuardedStateArgs,
    accounts: CloseGuardedStateAccounts,
) -> Instruction {
    let CloseGuardedStateAccounts {
        guarded_account,
        guarded_state,
        signer,
        token_account,
    } = accounts;

    Instruction {
        program_id,
        data: vec![],
        accounts: crate::accounts::CloseGuardedState {
            guarded_account,
            guarded_state,
            signer,
            token_account,
        }
        .to_account_metas(None),
    }
}
