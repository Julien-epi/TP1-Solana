use anchor_lang::prelude::*;

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