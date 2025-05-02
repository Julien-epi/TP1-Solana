use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
use crate::models::joueur::*;
use crate::errors::ErrorCode;

pub fn initialize_joueur(ctx: Context<InitializeJoueur>, pseudonyme: String) -> Result<()> {
    let joueur = &mut ctx.accounts.joueur;
    joueur.user_address = ctx.accounts.authority.key();
    joueur.pseudonyme = pseudonyme;
    joueur.points_de_vie = 100;
    joueur.experience = 0;
    joueur.vivant = true;
    Ok(())
}

pub fn buy_experience(ctx: Context<BuyExperience>) -> Result<()> {
    let joueur = &mut ctx.accounts.joueur;
    joueur.experience += 10;
    
    // Transfert de 0.1 SOL du joueur vers le vault
    // 0.1 SOL = 100_000_000 lamports (1 SOL = 1_000_000_000 lamports)
    let amount = 100_000_000;
    
    // Création de l'instruction de transfert
    let transfer_instruction = system_instruction::transfer(
        &ctx.accounts.authority.key(),
        &ctx.accounts.vault.key(),
        amount,
    );
    
    // Exécution de l'instruction de transfert
    invoke(
        &transfer_instruction,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;
    
    Ok(())
}

pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
    // Initialisation du vault (pas besoin de stocker des données supplémentaires)
    Ok(())
}

pub fn attaquer(ctx: Context<Attaquer>) -> Result<()> {
    // Vérifier que l'attaquant est vivant
    if !ctx.accounts.attaquant.vivant {
        return Err(ErrorCode::JoueurMort.into());
    }

    // Vérifier que la cible est vivante
    if !ctx.accounts.cible.vivant {
        return Err(ErrorCode::JoueurMort.into());
    }

    // Calculer les dégâts en fonction de l'expérience de l'attaquant
    // Plus l'attaquant a d'expérience, plus les dégâts sont importants
    let degats = 5 + (ctx.accounts.attaquant.experience / 10) as u8;
    
    // Limiter les dégâts aux points de vie restants de la cible
    let degats_appliques = std::cmp::min(degats, ctx.accounts.cible.points_de_vie);
    
    // Appliquer les dégâts
    ctx.accounts.cible.points_de_vie = ctx.accounts.cible.points_de_vie.saturating_sub(degats_appliques);
    
    // Vérifier si la cible est morte
    if ctx.accounts.cible.points_de_vie == 0 {
        ctx.accounts.cible.vivant = false;
        
        // L'attaquant gagne de l'expérience en tuant un autre joueur
        ctx.accounts.attaquant.experience += 50;
    } else {
        // L'attaquant gagne un peu d'expérience même si la cible survit
        ctx.accounts.attaquant.experience += 5;
    }
    
    Ok(())
}

pub fn soigner(ctx: Context<Soigner>) -> Result<()> {
    // Vérifier que le joueur est vivant
    if !ctx.accounts.joueur.vivant {
        return Err(ErrorCode::JoueurMort.into());
    }

    // Soigner le joueur - coûte de l'expérience
    let cout_xp = 20;
    
    // Vérifier si le joueur a assez d'expérience
    if ctx.accounts.joueur.experience < cout_xp {
        return Err(ErrorCode::ExperienceInsuffisante.into());
    }

    // Déduire le coût en expérience
    ctx.accounts.joueur.experience -= cout_xp;
    
    // Soigner le joueur (maximum 100 points de vie)
    let soin = 20;
    ctx.accounts.joueur.points_de_vie = std::cmp::min(100, ctx.accounts.joueur.points_de_vie + soin);
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(pseudonyme: String)]
pub struct InitializeJoueur<'info> {
    #[account(
        init,
        seeds = [b"joueur", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 4 + 100 + 1 + 4 + 1  // 8 (discriminator) + Pubkey + String + u8 + u32 + bool
    )]
    pub joueur: Account<'info, Joueur>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyExperience<'info> {
    #[account(mut, seeds = [b"joueur", authority.key().as_ref()], bump)]
    pub joueur: Account<'info, Joueur>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Ce compte est utilisé uniquement pour recevoir des SOL
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    /// CHECK: Ce compte est utilisé uniquement pour recevoir des SOL
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub vault: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
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