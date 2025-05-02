use anchor_lang::prelude::*;

#[account]
pub struct Joueur {
    pub user_address: Pubkey,
    pub pseudonyme: String,
    pub points_de_vie: u8,
    pub experience: u32,
    pub vivant: bool,
}