
use anchor_lang::prelude::*;
//use anchor_spl::token::spl_token::instruction::transfer_checked;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, TransferChecked, transfer_checked};
use anchor_spl::token_interface::Mint;

// way to have program work across old token programs and new token programs

// & reference ensures that the accont is READ ONLY
pub fn transfer_tokens <'info> (
    from: &InterfaceAccount<'info, TokenAccount>,       // &, READ ONLY
    to: &InterfaceAccount<'info, TokenAccount>,         // &, READ ONLY
    amount: &u64,                                       // &, READ ONLY
    mint: &InterfaceAccount<'info, Mint>,               // &, READ ONLY
    authority: &Signer<'info>,                          // &, READ ONLY
    token_program: &Interface<'info, TokenInterface>    // &, READ ONLY
) -> Result<()> {      

    // transfer checked is anchor's out of the box solution to transferring tokens
    let transfer_accounts_options = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info()
    };

    let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts_options);

    transfer_checked(cpi_context, *amount, mint.decimals);

    Ok(())
}