use std::str;

use serde::{Deserialize, Serialize};

use crate::server::client;

use crate::config::ShakespearerConfig;
use crate::server::pokeapi::PokemonSpeciesEntry;

#[derive(Serialize)]
pub struct ShakespearRequest {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct ShakespearTranslation {
    pub contents: ShakespearTranslationContent,
}

#[derive(Deserialize, Debug)]
pub struct ShakespearTranslationContent {
    pub translated: String,
}

pub async fn translate(
    config: &ShakespearerConfig,
    pokemon_species: &PokemonSpeciesEntry,
) -> Option<ShakespearTranslation> {
    client::post(
        config.base_url.as_str(),
        ShakespearRequest {
            text: pokemon_species.flavor_text.clone(),
        },
    )
    .await
}
