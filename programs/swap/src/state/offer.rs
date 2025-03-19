
// we need to build an offer
// if someone makes an offer, we n

use anchor_lang::prelude::*;

// when we MAKE offer, we are given tokens in exchange for other tokens.
// INFO THAT WE NEED:
// offer ID
// offering mint token address
// offering int token amount
// receiving mint token address
// receiving mint token amount
// receiving mint token address

// we need an anchor account:
// THIS IS A PDA!!!!!!!!!
#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,
}
