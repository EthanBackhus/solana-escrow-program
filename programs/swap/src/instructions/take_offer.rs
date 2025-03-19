use anchor_lang:: prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{
        transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked, close_account, CloseAccount
    }
};


use crate::Offer;

use super::transfer_tokens;

#[derive(Accounts)]
// need: token mint a, 
#[instruction(id: u64)]
pub struct TakeOffer <'info> {
    
    #[account(mut)]         // mutable because the taker is the one signing for the transaction?
    pub taker: Signer <'info>,
    
    #[account(mut)]
    pub maker: SystemAccount <'info>,

    pub token_mint_a: InterfaceAccount<'info, Mint>,
    
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,    // mutable because we are going to close this account when the offer is taken
        close = maker,     // maker made it, when it's closed it needs to go back to maker
        has_one = maker,
        has_one = token_mint_a,
        has_one = token_mint_b,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,        // authority of the vault is controlled by the offer, which the offer account can sign for things
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // need system program if we're creating something
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    // program that maps associated token accounts to their owners
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_wanted_tokens_to_maker(ctx: &Context<TakeOffer>                            
) -> Result<()> {

    transfer_tokens(
        &ctx.accounts.taker_token_account_b,
        &ctx.accounts.maker_token_account_b,
        &ctx.accounts.offer.token_b_wanted_amount,
        &ctx.accounts.token_mint_b,
        &ctx.accounts.taker,
        &ctx.accounts.token_program,
    )?;

    Ok(())
}

pub fn withdraw_and_close_vault(ctx: Context<TakeOffer>
) -> Result<()> {

    let seeds = &[
        b"offer",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.offer.id.to_le_bytes()[..],
        &[ctx.accounts.offer.bump],
    ];

    let signer_seeds = [&seeds[..]];

    let accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.taker_token_account_a.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        accounts,
        &signer_seeds
    );

    transfer_checked(
        cpi_context,
        ctx.accounts.vault.amount,
        ctx.accounts.token_mint_a.decimals,
    )?;

    // now we need to close the vault

    let accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
        destination: ctx.accounts.taker.to_account_info()
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        accounts, 
        &signer_seeds
    );

    close_account(cpi_context)?;

    Ok(())
}