#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
//use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::{TokenAccount, Mint, Token};

declare_id!("FEoQe51trDQ5v4C6qDH97FmreGi1Ls2fGqkcYSzG9urc");

#[program]
pub mod mypesa_vault {
    use super::*;

    pub fn initialize_vault(_ctx: Context<InitializeMypesaVault>) -> Result<()> {
        msg!("The vault has been initialized");
        Ok(())
    }

    pub fn deposit_to_vault(_ctx: Context<MypesaVaultActions>, amount: u64) -> Result<()> {
        msg!("Deposited {} amount to MyPesa vault", amount);
        Ok(())
    }

    pub fn withdraw_from_vault(_ctx: Context<MypesaVaultActions>, amount: u64) -> Result<()> {
        msg!("Withdrawn {} amount from MyPesa vault", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMypesaVault<'info> {
    /// CHECK: no check here, we are passing here ourselves.
    // This pda signs the transactions for the Vault.
    // Since pdas are owned by the program.
    #[account(
        init,
        payer=signer,
        seeds=[b"mypesa_vault_account_pda"],
        bump,
        space=8
    )]
    mypesa_vault_account_pda: AccountInfo<'info>,
    // Holds the tokens being stored in the vault
    // Owned by the program via pda.
    #[account(
        init,
        payer=signer,
        seeds=[b"mypesa_vault", mint_of_the_token_being_sent.key().as_ref()],
        bump,
        token::mint=mint_of_the_token_being_sent,
        token::authority=mypesa_vault_account_pda,
    )]
    mypesa_vault: Account<'info, TokenAccount>,
    // Mint for the token sent to be stored in the vault.
    mint_of_the_token_being_sent: Account<'info, Mint>,

    // Signer
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MypesaVaultActions<'info> {
    /// CHECK: No check, we are passing here ourselves
    #[account(
        mut,
        seeds=[b"mypesa_vault_account_pda"],
        bump
    )]
    mypesa_vault_account_pda: AccountInfo<'info>,
    #[account(
        mut,
        seeds=[b"mypesa_vault", mint_of_the_token_being_sent.key().as_ref()],
        bump,
        token::mint=mint_of_the_token_being_sent,
        token::authority=mypesa_vault_account_pda,
    )]
    mypesa_vault: Account<'info, TokenAccount>,

    // Token account sending tokens to the vault.
    #[account(mut)]
    sender_token_account: Account<'info, TokenAccount>,

    mint_of_the_token_being_sent: Account<'info, Mint>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}
