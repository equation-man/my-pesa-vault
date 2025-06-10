#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

// MyPesa modules
mod mypesa_accounts;
use mypesa_accounts::*;

declare_id!("FEoQe51trDQ5v4C6qDH97FmreGi1Ls2fGqkcYSzG9urc");

#[program]
pub mod mypesa_vault {
    use super::*;

    pub fn initialize_vault(_ctx: Context<InitializeMypesaVault>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_to_vault(ctx: Context<MypesaVaultActions>, deposit_amount: u64) -> Result<()> {
        require!(deposit_amount>=0, MyPesaVaultError::InvalidAmount);

        msg!("Depositing {} amount to MyPesa vault", deposit_amount);
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.sender_wallet_token_account.to_account_info(),
                    to: ctx.accounts.mypesa_vault.to_account_info(),
                    authority: ctx.accounts.sender_wallet_token_account.to_account_info(),
                }
            ),
            deposit_amount,
        )?;
        Ok(())
    }

    pub fn withdraw_from_vault(ctx: Context<MypesaVaultActions>, withdraw_amount: u64) -> Result<()> {
        require_gt!(amount, ctx.accounts.mypesa_vault.to_account_info(), MyPesaVaultError::WithdrawLimit);

        msg!("Withdrawing {} amount from MyPesa vault", withdraw_amount);
        let mint_keys = ctx.accounts.mint_of_the_token_being_sent.key();
        let signers = &[b"mypesa_vault", mint_keys.as_ref(), &ctx.bumps.mypesa_vault];
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.mypesa_vault.to_account_info(),
                    to: ctx.accounts.sender_wallet_token_account.to_account_info(),
                    authority: ctx.accounts.mypesa_vault.to_account_info(),
                },
                &[&signers[..]],
            ),
            withdraw_amount,
        )?;
        Ok(())
    }

    // IMPLEMENT READING ACCOUNT BALANCE, WITHDRAWAL AND DEPOSIT INFO LOGS.
}
