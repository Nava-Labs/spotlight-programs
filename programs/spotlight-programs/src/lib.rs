use anchor_lang::prelude::*;
use state::EscrowVault;

declare_id!("DpEzxKthmkTxjBCq7qF414w1NR6cByxKHLimV7eU4jBt");

pub mod state;

#[program]
pub mod spotlight_programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.escrow_vault.sol_vault_bump = ctx.bumps.escrow_sol_vault;
        ctx.accounts.escrow_vault.signer_authority = ctx.accounts.signer_authority.key();

        Ok(())
    }

    pub fn request(ctx: Context<Request>, sol_amount: u64) -> Result<()> {
        let escrow_vault = &mut ctx.accounts.escrow_vault;
        let from = &mut ctx.accounts.user;
        let escrow_sol_vault = &mut ctx.accounts.escrow_sol_vault;
        let system_program = &ctx.accounts.system_program;

        escrow_vault.transfer_to_vault(
            &from.to_account_info(),
            &escrow_sol_vault.to_account_info(),
            sol_amount,
            system_program,
        )?;

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, sol_amount: u64) -> Result<()> {
        let escrow_vault = &mut ctx.accounts.escrow_vault;
        let escrow_sol_vault = &mut ctx.accounts.escrow_sol_vault;
        let to = &mut ctx.accounts.user;
        let system_program = &ctx.accounts.system_program;

        let signer_authority = ctx.accounts.signer_authority.key();
        if escrow_vault.signer_authority != signer_authority {
            return Err(ErrorCode::InvalidAuthority.into());
        }

        escrow_vault.transfer_from_vault(
            &escrow_sol_vault.to_account_info(),
            to,
            sol_amount,
            system_program,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, space = EscrowVault::ACCOUNT_SIZE, seeds = [EscrowVault::VAULT_SEED.as_bytes()], bump, payer = user)]
    pub escrow_vault: Account<'info, EscrowVault>,

    /// CHECK:
    #[account(mut, seeds = [EscrowVault::SOL_VAULT_SEED.as_bytes()], bump)]
    pub escrow_sol_vault: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub signer_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Request<'info> {
    #[account(mut, seeds = [EscrowVault::VAULT_SEED.as_bytes()], bump)]
    pub escrow_vault: Account<'info, EscrowVault>,

    /// CHECK:
    #[account(mut, seeds = [EscrowVault::SOL_VAULT_SEED.as_bytes()], bump)]
    pub escrow_sol_vault: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut, seeds = [EscrowVault::VAULT_SEED.as_bytes()], bump )]
    pub escrow_vault: Account<'info, EscrowVault>,

    /// CHECK:
    #[account(mut, seeds = [EscrowVault::SOL_VAULT_SEED.as_bytes()], bump )]
    pub escrow_sol_vault: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub signer_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid authority")]
    InvalidAuthority,
}
