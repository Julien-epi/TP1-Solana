use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
use crate::models::item::*;
use crate::models::joueur::*;
use crate::errors::ErrorCode;

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
    
    // Vérifier que l'utilisateur est bien le propriétaire de l'item
    if item.proprietaire != ctx.accounts.authority.key() {
        return Err(ErrorCode::NonProprietaire.into());
    }
    
    item.en_vente = true;
    item.prix = prix;
    
    Ok(())
}

pub fn retirer_vente(ctx: Context<GererVente>) -> Result<()> {
    let item = &mut ctx.accounts.item;
    
    // Vérifier que l'utilisateur est bien le propriétaire de l'item
    if item.proprietaire != ctx.accounts.authority.key() {
        return Err(ErrorCode::NonProprietaire.into());
    }
    
    item.en_vente = false;
    
    Ok(())
}

pub fn acheter_item(ctx: Context<AcheterItem>) -> Result<()> {
    let item = &mut ctx.accounts.item;
    
    // Vérifier que l'item est bien en vente
    if !item.en_vente {
        return Err(ErrorCode::ItemNonEnVente.into());
    }
    
    // Récupérer le prix de l'item
    let montant = item.prix;
    
    // Créer l'instruction de transfert
    let transfer_instruction = system_instruction::transfer(
        &ctx.accounts.acheteur.key(),
        &ctx.accounts.vendeur.key(),
        montant,
    );
    
    // Exécuter l'instruction de transfert
    invoke(
        &transfer_instruction,
        &[
            ctx.accounts.acheteur.to_account_info(),
            ctx.accounts.vendeur.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;
    
    // Mettre à jour le propriétaire de l'item
    item.proprietaire = ctx.accounts.acheteur.key();
    item.en_vente = false;
    
    Ok(())
}

pub fn utiliser_item(ctx: Context<UtiliserItem>) -> Result<()> {
    let item = &ctx.accounts.item;
    let joueur = &mut ctx.accounts.joueur;
    
    // Vérifier que le joueur est vivant
    if !joueur.vivant {
        return Err(ErrorCode::JoueurMort.into());
    }
    
    // Vérifier que l'utilisateur est bien le propriétaire de l'item
    if item.proprietaire != ctx.accounts.authority.key() {
        return Err(ErrorCode::NonProprietaire.into());
    }
    
    // Ajouter l'expérience au joueur en fonction de la puissance de l'item
    joueur.experience += item.puissance as u32;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(nom: String, puissance: u8, cout: u64)]
pub struct CreerItem<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 4 + 50 + 1 + 8 + 32 + 1  // 8 (discriminator) + String + u8 + u64 + Pubkey + bool
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
    
    /// CHECK: Le vendeur est vérifié par le programme
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