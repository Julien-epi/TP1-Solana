use anchor_lang::prelude::*;

#[account]
pub struct Item {
    pub nom: String,
    pub puissance: u8,
    pub prix: u64,
    pub proprietaire: Pubkey,
    pub en_vente: bool,
}