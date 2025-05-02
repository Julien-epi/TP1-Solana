use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::solana_program::{program::invoke, system_instruction, native_token::LAMPORTS_PER_SOL};

declare_id!("4RgzWS9Gixt44wwULLUVw47Dixxh1ywbGkZ4D1yPVUgn");

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
            experience if experience <= 5 => Level::Beginner,
            experience if experience <= 35 => Level::Explorer,
            experience if experience <= 100 => Level::Champion,
            _ => Level::Legend,
        };

        Ok(())
    }

    pub fn withdraw_vault(ctx: Context<WithdrawVault>) -> Result<()> {
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

    pub fn attaquer(ctx: Context<Attaquer>) -> Result<()> {
        if !ctx.accounts.attaquant.vivant {
            return Err(ErrorCode::JoueurMort.into());
        }

        if !ctx.accounts.cible.vivant {
            return Err(ErrorCode::JoueurMort.into());
        }

        let degats = 5 + (ctx.accounts.attaquant.xp / 10) as u8;
        
        let degats_appliques = std::cmp::min(degats, ctx.accounts.cible.pdv as u8);
        
        ctx.accounts.cible.pdv = ctx.accounts.cible.pdv.saturating_sub(degats_appliques as u64);
        
        if ctx.accounts.cible.pdv == 0 {
            ctx.accounts.cible.vivant = false;
            
            ctx.accounts.attaquant.xp += 50;
        } else {
            ctx.accounts.attaquant.xp += 5;
        }
        
        Ok(())
    }

    pub fn soigner(ctx: Context<Soigner>) -> Result<()> {
        if !ctx.accounts.joueur.vivant {
            return Err(ErrorCode::JoueurMort.into());
        }

        let cout_xp = 20;
        
        if ctx.accounts.joueur.xp < cout_xp {
            return Err(ErrorCode::ExperienceInsuffisante.into());
        }

        ctx.accounts.joueur.xp -= cout_xp;
        
        let soin = 20;
        ctx.accounts.joueur.pdv = std::cmp::min(100, ctx.accounts.joueur.pdv + soin);
        
        Ok(())
    }

    pub fn creer_item(ctx: Context<CreerItem>, nom: String, puissance: u8, cout: u64) -> Result<()> {
        let item = &mut ctx.accounts.item;
        item.nom = nom;
        item.puissance = puissance;
        item.prix = cout;
        item.proprietaire = ctx.accounts.authority.key();
        item.en_vente = false;
        
        Ok(())
    }

    pub fn mettre_en_vente(ctx: Context<GererVente>, prix: u64) -> Result<()> {
        let item = &mut ctx.accounts.item;
        
        if item.proprietaire != ctx.accounts.authority.key() {
            return Err(ErrorCode::NonProprietaire.into());
        }
        
        item.en_vente = true;
        item.prix = prix;
        
        Ok(())
    }
    
    pub fn retirer_vente(ctx: Context<GererVente>) -> Result<()> {
        let item = &mut ctx.accounts.item;
        
        if item.proprietaire != ctx.accounts.authority.key() {
            return Err(ErrorCode::NonProprietaire.into());
        }
        
        item.en_vente = false;
        
        Ok(())
    }

    pub fn acheter_item(ctx: Context<AcheterItem>) -> Result<()> {
        let item = &mut ctx.accounts.item;
        
        if !item.en_vente {
            return Err(ErrorCode::ItemNonEnVente.into());
        }
        
        let montant = item.prix;
        
        let transfer_instruction = system_instruction::transfer(
            &ctx.accounts.acheteur.key(),
            &ctx.accounts.vendeur.key(),
            montant,
        );
        
        invoke(
            &transfer_instruction,
            &[
                ctx.accounts.acheteur.to_account_info(),
                ctx.accounts.vendeur.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        
        item.proprietaire = ctx.accounts.acheteur.key();
        item.en_vente = false;
        
        Ok(())
    }

    pub fn utiliser_item(ctx: Context<UtiliserItem>) -> Result<()> {
        let item = &ctx.accounts.item;
        let joueur = &mut ctx.accounts.joueur;
        
        if !joueur.vivant {
            return Err(ErrorCode::JoueurMort.into());
        }
        
        if item.proprietaire != ctx.accounts.authority.key() {
            return Err(ErrorCode::NonProprietaire.into());
        }
        
        joueur.xp += item.puissance as u64;
        
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
pub struct Attaquer<'info> {
    #[account(mut, seeds = [b"joueur", authority.key().as_ref()], bump)]
    pub attaquant: Account<'info, Joueur>,
    
    #[account(mut)]
    pub cible: Account<'info, Joueur>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Soigner<'info> {
    #[account(mut, seeds = [b"joueur", authority.key().as_ref()], bump)]
    pub joueur: Account<'info, Joueur>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nom: String, puissance: u8, cout: u64)]
pub struct CreerItem<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 4 + 50 + 1 + 8 + 32 + 1 
    )]
    pub item: Account<'info, Item>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GererVente<'info> {
    #[account(mut)]
    pub item: Account<'info, Item>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcheterItem<'info> {
    #[account(mut)]
    pub item: Account<'info, Item>,
    
    #[account(mut)]
    pub vendeur: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub acheteur: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UtiliserItem<'info> {
    #[account(mut)]
    pub item: Account<'info, Item>,
    
    #[account(mut, seeds = [b"joueur", authority.key().as_ref()], bump)]
    pub joueur: Account<'info, Joueur>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
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

#[account]
pub struct Item {
    pub nom: String,
    pub puissance: u8,
    pub prix: u64,
    pub proprietaire: Pubkey,
    pub en_vente: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Le joueur est mort")]
    JoueurMort,
    #[msg("Expérience insuffisante")]
    ExperienceInsuffisante,
    #[msg("Vous n'êtes pas le propriétaire")]
    NonProprietaire,
    #[msg("L'item n'est pas en vente")]
    ItemNonEnVente,
}