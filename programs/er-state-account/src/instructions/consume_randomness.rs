use anchor_lang::prelude::*;

use crate::state::UserAccount;
#[derive(Accounts)]
pub struct ConsumeRandomness<'info> {
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}
impl<'info> ConsumeRandomness<'info> {
    pub fn consume_randomness(&mut self, randomness: [u8; 32]) -> Result<()> {
        let rnd = ephemeral_vrf_sdk::rnd::random_u64(&randomness);
        msg!("Consuming random number: {:?}", rnd);
        let user_account = &mut self.user_account;
        user_account.data = rnd; // Update the user's last result
        msg!("User data updated: {:?}", user_account.data);
        Ok(())
    }
}
