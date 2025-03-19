pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5mgkZWHW46PL1vgYuCuy4y2upQDyXcphDivMEwToj46D");


#[program]
pub mod swap {

    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>,
                      id: u64,  // we need to come up with some logic to ensure that multiple ID's aren't used
                      token_a_offered_amount: u64,
                      token_b_wanted_amount: u64,
    ) -> Result<()> {
    
        instructions::make_offer::send_offered_tokens_to_vault(&ctx, token_a_offered_amount)?;
        instructions::make_offer::save_offer(ctx, id, token_b_wanted_amount)?;

        Ok(())
    }

    // we are going to have 2 parts with take offer:
    // 1. transfer tokens from vault to takers account
    // 2. transfer tokens from taker to makers account
    // so, what variables do we need for TakeOffer?
    // 
    pub fn take_offer(ctx: Context<TakeOffer>
    ) -> Result<()> {
        instructions::take_offer::send_wanted_tokens_to_maker(&ctx)?;
        instructions::take_offer::withdraw_and_close_vault(ctx)?;

        Ok(())
    }
}




