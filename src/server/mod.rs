pub mod client;
pub mod pokeapi;
pub mod shakespearer;

use serde::Serialize;

use actix_web::{get, web, HttpResponse, Responder};

use crate::config::Config;

const USER_AGENT_KEY: &'static str = "User-Agent";
const UA: &'static str = "Actix-web";

#[derive(Serialize)]
struct PokemonBody {
    name: String,
    description: String,
}

#[derive(Serialize)]
struct PokemonError {
    error: String,
}

#[get("/pokemon/{pokemon_name}")]
pub async fn get_pokemon(
    config: web::Data<Config>,
    pokemon_name: web::Path<String>,
) -> impl Responder {
    super::logger::log(format!("Pokémon name: {}", pokemon_name));

    let name = format!("{}", pokemon_name);
    match pokeapi::get_pokemon_tuple(&config.pokeapi, name.as_str()).await {
        Some(pokemon) => {
            super::logger::log(format!("Pokémon found: {:?}", pokemon));

            match shakespearer::translate(&config.shakespearer, &pokemon.1).await {
                Some(translation) => {
                    super::logger::log(format!("Pokémon translated: {:?}", translation));

                    HttpResponse::Ok().json(PokemonBody {
                        name,
                        description: translation.contents.translated,
                    })
                }
                _ => {
                    super::logger::log(format!("Couldn't translate Pokémon \"{}\".", pokemon_name));

                    HttpResponse::NotFound().json(PokemonError {
                        error: "Error translating this Pokémon.".into(),
                    })
                }
            }
        }
        _ => {
            super::logger::log(format!("Pokémon \"{}\" not found.", pokemon_name));

            HttpResponse::NotFound().json(PokemonError {
                error: "Pokémon not found.".into(),
            })
        }
    }
}
