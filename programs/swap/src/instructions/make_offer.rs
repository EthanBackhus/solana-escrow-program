use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::{Offer, ANCHOR_DISCRIMINATOR};

use super::shared::transfer_tokens;

// when we save stuff to blockchain, anchor will need 8 bytes plus
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer <'info> {
    #[account(mut)]
    pub maker: Signer <'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    
    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR_SIZE + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,   // Offer is an instance of that offer account, not offer struct

    // need vault account

    #[account(
        init,
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,        // THIS MEANS THIS ISN'T OWNER BY A USER, BUT RATHER A PROGRAM!!!!!!!
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // need system program if we're creating something
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    // program that maps associated token accounts to their owners
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_offered_tokens_to_vault(
    ctx: &Context<MakeOffer>,
    token_a_offered_amount: u64,
) -> Result<()> {
    
    transfer_tokens(
        &ctx.accounts.maker_token_account_a,
        &ctx.accounts.vault,
        &token_a_offered_amount,
        &ctx.accounts.token_mint_a,
        &ctx.accounts.maker,
        &ctx.accounts.token_program
    )?;

    Ok(())
}

pub fn save_offer(
    ctx: Context<MakeOffer>,
    id: u64,
    token_b_wanted_amount: u64,
) -> Result<()> {
    ctx.accounts.offer.set_inner(Offer {
        id: id,
        token_mint_a: ctx.accounts.token_mint_a.key(),
        token_mint_b: ctx.accounts.token_mint_b.key(),
        maker: ctx.accounts.maker.key(),
        token_b_wanted_amount: token_b_wanted_amount,
        bump: ctx.bumps.offer,
    });
    Ok(())
}