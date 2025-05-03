use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use std::str::FromStr;

declare_id!("4RgzWS9Gixt44wwULLUVw47Dixxh1ywbGkZ4D1yPVUgn");

const ADMIN_PUBKEY: &str = "FwN1nDBaVhEzjnYRN537zx4hx3FgXRiGfRUKTf1ywCib";

#[program]
pub mod tp1 {
    use super::*;

    pub fn initialize_joueur(ctx: Context<InitializeJoueur>, pseudo: String) -> Result<()> {
        let joueur = &mut ctx.accounts.joueur;
        joueur.pdv = 100;
        joueur.pseudonyme = pseudo;
        joueur.xp = 0;
        joueur.vivant = true;
        joueur.user_address = *ctx.accounts.signer.key;
        joueur.level = Level::Beginner;

        msg!("Joueur initialized!");
        msg!("pdv {}", joueur.pdv);
        msg!("pseudonyme {}", joueur.pseudonyme);
        msg!("xp {}", joueur.xp);
        msg!("vivant {}", joueur.vivant);
        msg!("user_address {}", joueur.user_address);

        Ok(())
    }

    pub fn buy_experience(ctx: Context<BuyExperience>) -> Result<()> {
        let joueur = &mut ctx.accounts.joueur;
        joueur.xp += 10;

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.signer.to_account_info(), 
                to: ctx.accounts.vault.to_account_info(),   
            },
        );

        let amount = (LAMPORTS_PER_SOL as f64 * 0.1) as u64;
        transfer(cpi_context, amount)?; 

        joueur.level = match joueur.xp {
            experience if experience < 5 => Level::Beginner,
            experience if experience >= 5 && experience < 35 => Level::Explorer,
            experience if experience >= 35 && experience < 100 => Level::Champion,
            _ => Level::Legend,
        };

        Ok(())
    }

    pub fn withdraw_vault(ctx: Context<WithdrawVault>) -> Result<()> {
        let admin_pubkey = Pubkey::from_str(ADMIN_PUBKEY).unwrap();
        if ctx.accounts.admin.key() != admin_pubkey {
            return Err(error!(ErrorCode::NonAdmin));
        }

        let amount = **ctx.accounts.vault.to_account_info().lamports.borrow();

        let seeds = &[b"vault".as_ref(), &[ctx.bumps.vault]];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.admin.to_account_info(),
            },
            signer_seeds,
        );

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn battle(ctx: Context<Battle>) -> Result<()> {
        let attacker = &mut ctx.accounts.attacker;
        let defender = &mut ctx.accounts.defender;

        if !attacker.vivant {
            return Err(error!(ErrorCode::JoueurMort));
        }
        
        if !defender.vivant {
            return Err(error!(ErrorCode::JoueurMort));
        }

        if attacker.xp >= defender.xp {

            if defender.pdv <= 50 {
                defender.pdv = 0;
                defender.vivant = false;
            } else {
                defender.pdv -= 50;
            }

            attacker.xp += 5;
        } else {
            if attacker.pdv <= 50 {
                attacker.pdv = 0;
                attacker.vivant = false;
            } else {
                attacker.pdv -= 50;
            }

            defender.xp += 5;
        }

        attacker.level = match attacker.xp {
            experience if experience < 5 => Level::Beginner,
            experience if experience >= 5 && experience < 35 => Level::Explorer,
            experience if experience >= 35 && experience < 100 => Level::Champion,
            _ => Level::Legend,
        };

        defender.level = match defender.xp {
            experience if experience < 5 => Level::Beginner,
            experience if experience >= 5 && experience < 35 => Level::Explorer,
            experience if experience >= 35 && experience < 100 => Level::Champion,
            _ => Level::Legend,
        };

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeJoueur<'info> {
    #[account(
        init, 
        payer = signer, 
        space = 8 + Joueur::INIT_SPACE,
        seeds = [b"joueur", signer.key().as_ref()],
        bump
    )]
    pub joueur: Account<'info, Joueur>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyExperience<'info> {
    #[account(
        mut,
        seeds = [b"joueur", signer.key().as_ref()],
        bump
    )]
    pub joueur: Account<'info, Joueur>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawVault<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Battle<'info> {
    #[account(
        mut,
        seeds = [b"joueur", attacker_owner.key().as_ref()],
        bump
    )]
    pub attacker: Account<'info, Joueur>,
    #[account(mut)]
    pub attacker_owner: Signer<'info>,
    
    #[account(
        mut,
        constraint = attacker.key() != defender.key() @ ProgramError::InvalidArgument
    )]
    pub defender: Account<'info, Joueur>,
    
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Joueur {
    user_address: Pubkey,
    #[max_len(50)]
    pseudonyme: String,
    pdv: u64,
    xp: u64,
    vivant: bool,
    level: Level,
}

#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Debug, Clone, Copy)]
pub enum Level {
    Beginner,
    Explorer,
    Champion,
    Legend,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Le joueur est mort")]
    JoueurMort,
    #[msg("Expérience insuffisante")]
    ExperienceInsuffisante,
    #[msg("Vous n'êtes pas autorisé à effectuer cette action")]
    NonAdmin,
}