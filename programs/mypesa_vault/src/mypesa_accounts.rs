/// CONTAINS THE ACCOUNTS USED IN THE PROGRAM
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct InitializeMypesaVault<'info> {
    /// CHECK: We are passing here ourselves.
    // This pda signs the transactions for the Vault.
    // Since pdas are owned by the program.
    #[account(
        init,
        payer=signer,
        seeds=[b"mypesa_vault_account_pda"],
        bump,
        space=TransactionLog::INIT_SPACE,
    )]
    pub mypesa_vault_account_pda: Account<'info, TransactionLog>,
    // Holds the tokens being stored in the vault
    // Owned by the program via pda.
    #[account(
        init,
        payer=signer,
        seeds=[b"mypesa_vault", signer.key().as_ref()],
        bump,
        token::mint=mint_of_the_token_being_sent,
        token::authority=mypesa_vault_account_pda,
    )]
    pub mypesa_vault: Account<'info, TokenAccount>,
    // Mint for the token sent to be stored in the vault.
    pub mint_of_the_token_being_sent: Account<'info, Mint>,

    // Signer
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MypesaVaultActions<'info> {
    /// CHECK: we are passing here ourselves
    #[account(
        mut,
        seeds=[b"mypesa_vault_account_pda", signer.key().as_ref()],
        bump
    )]
    pub mypesa_vault_account_pda: Account<'info, TransactionLog>,
    #[account(
        mut,
        associated_token::mint=mint_of_the_token_being_sent,
        associated_token::authority=mypesa_vault_account_pda,
        associated_token::token_program=token_program,
    )]
    pub mypesa_vault: Account<'info, TokenAccount>,

    // Token account sending tokens to the vault.
    #[account(
        mut,
        token::mint=mint_of_the_token_being_sent,
        token::authority=sender_wallet_token_account,
        seeds=[b"sender_wallet"],
        bump
    )]
    pub sender_wallet_token_account: Account<'info, TokenAccount>,
    pub mint_of_the_token_being_sent: Account<'info, Mint>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

/// Transaction logs or history.
#[account]
#[derive(InitSpace)]
pub struct TransactionLog {
    pub amount: u64, // Amount transacted.
    pub updated_at: i64, // Timestamp for the transaction time.
    #[max_len(6)]
    pub trans_type: String, // Transaction type
}

// PROGRAM ERRORS
#[error_code]
pub enum MyPesaVaultError {
    #[msg("The amount entered is not valid. Should be greater than 0.")]
    InvalidAmount,
    #[msg("You have insuffiecient balance to perform the withdrawal")]
    WithdrawLimit,
}
