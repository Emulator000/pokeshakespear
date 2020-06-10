use std::collections::VecDeque;

use serde::Deserialize;

use crate::config::PokeApiConfig;
use crate::server::client;

pub const POKEMON_PATH: &str = "pokemon/";

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    species: Species,
}

#[derive(Deserialize, Debug)]
pub struct Species {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct PokemonSpecies {
    flavor_text_entries: VecDeque<PokemonSpeciesEntry>,
}

#[derive(Deserialize, Debug)]
pub struct PokemonSpeciesEntry {
    pub flavor_text: String,
    pub language: PokemonSpeciesEntryLanguage,
}

#[derive(Deserialize, Debug)]
pub struct PokemonSpeciesEntryLanguage {
    name: String,
}

pub async fn get_pokemon_tuple<S: AsRef<str>>(
    config: &PokeApiConfig,
    name: S,
) -> Option<(Pokemon, PokemonSpeciesEntry)> {
    match client::get::<_, Pokemon>(format!(
        "{}{}{}",
        config.base_url,
        POKEMON_PATH,
        name.as_ref()
    ))
    .await
    {
        Some(pokemon) => match client::get::<_, PokemonSpecies>(pokemon.species.url.as_str()).await
        {
            Some(mut pokemon_species) => match pokemon_species.flavor_text_entries.pop_front() {
                Some(pokemon_species_entry) => Some((pokemon, pokemon_species_entry)),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}
