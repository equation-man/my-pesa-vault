#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_interface::{ transfer_checked, TransferChecked };

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
        require_gt!(deposit_amount, 0, MyPesaVaultError::InvalidAmount);

        msg!("CREDIT: {} deposited to your MyPesa vault", deposit_amount);
        let trans_log = &mut ctx.accounts.mypesa_vault_account_pda;
        let clock = Clock::get().unwrap();
        trans_log.amount = deposit_amount;
        trans_log.updated_at = clock.unix_timestamp;
        trans_log.trans_type = "CREDIT".to_string();

        let decimals = ctx.accounts.mint_of_the_token_being_sent.decimals;
        transfer_checked(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                TransferChecked {
                    mint: ctx.accounts.mint_of_the_token_being_sent.to_account_info(),
                    from: ctx.accounts.sender_wallet_token_account.to_account_info(),
                    to: ctx.accounts.mypesa_vault.to_account_info(),
                    authority: ctx.accounts.sender_wallet_token_account.to_account_info(),
                }
            ),
            deposit_amount,
            decimals,
        )?;
        Ok(())
    }

    pub fn withdraw_from_vault(ctx: Context<MypesaVaultActions>, withdraw_amount: u64) -> Result<()> {
        require_gt!(withdraw_amount, ctx.accounts.mypesa_vault.to_account_info().lamports(), MyPesaVaultError::WithdrawLimit);

        msg!("DEBIT: {} withdrawn from your MyPesa vault", withdraw_amount);
        let trans_log = &mut ctx.accounts.mypesa_vault_account_pda;
        let clock = Clock::get().unwrap();
        trans_log.amount = withdraw_amount;
        trans_log.updated_at = clock.unix_timestamp;
        trans_log.trans_type = "DEBIT".to_string();


        let mint_keys = ctx.accounts.mint_of_the_token_being_sent.key();
        let signers = &[b"mypesa_vault", mint_keys.as_ref(), &[ctx.bumps.mypesa_vault]];
        let decimals = ctx.accounts.mint_of_the_token_being_sent.decimals;
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                TransferChecked {
                    mint: ctx.accounts.mint_of_the_token_being_sent.to_account_info(),
                    from: ctx.accounts.mypesa_vault.to_account_info(),
                    to: ctx.accounts.sender_wallet_token_account.to_account_info(),
                    authority: ctx.accounts.mypesa_vault_account_pda.to_account_info(),
                },
                &[&signers[..]],
            ),
            withdraw_amount,
            decimals
        )?;
        Ok(())
    }

    // IMPLEMENT READING ACCOUNT BALANCE, WITHDRAWAL AND DEPOSIT INFO LOGS.
}
