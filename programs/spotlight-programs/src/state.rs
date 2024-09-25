use anchor_lang::{prelude::*, system_program};

#[account]
pub struct EscrowVault {
    pub sol_vault_bump: u8,
    pub total_sol_amount: u64,
    pub signer_authority: Pubkey,
}

impl EscrowVault {
    pub const VAULT_SEED: &'static str = "EscrowVault";

    pub const SOL_VAULT_SEED: &'static str = "EscrowSolVault";

    pub const ACCOUNT_SIZE: usize = 8 + 1 + 8 + 32;

    pub fn new(bump: u8, signer_authority: Pubkey) -> Self {
        Self {
            sol_vault_bump: bump,
            total_sol_amount: 0,
            signer_authority,
        }
    }

    pub fn transfer_to_vault<'info>(
        &mut self,
        from: &AccountInfo<'info>,
        to: &AccountInfo<'info>,
        sol_amount: u64,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                },
            ),
            sol_amount,
        )?;

        self.total_sol_amount += sol_amount;
        Ok(())
    }

    pub fn transfer_from_vault<'info>(
        &mut self,
        from: &AccountInfo<'info>,
        to: &AccountInfo<'info>,
        sol_amount: u64,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        system_program::transfer(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                },
                &[&[
                    EscrowVault::SOL_VAULT_SEED.as_bytes(),
                    &[self.sol_vault_bump],
                ]],
            ),
            sol_amount,
        )?;
        self.total_sol_amount -= sol_amount;
        Ok(())
    }
}
