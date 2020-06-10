use super::*;

use std::str;

use serde::Deserialize;

use actix_web::dev::Service;
use actix_web::{http, test, App, Error};

use crate::server::client;
use crate::server::pokeapi;
use crate::server::pokeapi::{Pokemon, PokemonSpecies};
use crate::server::shakespearer::{ShakespearRequest, ShakespearTranslationContent};

const POKEMON: &str = "charizard";

#[derive(Deserialize)]
struct PokemonResult {
    name: String,
}

fn load_config() -> Config {
    Config::new("config/config.toml")
}

#[actix_rt::test]
async fn get_pokemon() {
    let config = load_config();
    let pokemon = client::get::<_, Pokemon>(format!(
        "{}{}{}",
        config.pokeapi.base_url,
        pokeapi::POKEMON_PATH,
        POKEMON
    ))
    .await;

    assert_eq!(pokemon.is_some(), true);
}

#[actix_rt::test]
async fn get_pokemon_species() {
    let config = load_config();
    let pokemon_species =
        client::get::<_, PokemonSpecies>(format!("{}pokemon-species/6/", config.pokeapi.base_url))
            .await;

    assert_eq!(pokemon_species.is_some(), true);
}

#[actix_rt::test]
async fn get_translation() {
    let config = load_config();
    let translation = client::post::<_, ShakespearTranslationContent, _>(
        format!("{}pokemon-species/6/", config.shakespearer.base_url),
        ShakespearRequest {
            text: "This is a test!".into(),
        },
    )
    .await;

    assert_eq!(translation.is_some(), true); // Could fail due to translator API limits
}

#[actix_rt::test]
async fn get_pokemon_tuple() {
    let config = load_config();

    let pokemon = pokeapi::get_pokemon_tuple(&config.pokeapi, POKEMON).await;

    assert_eq!(pokemon.is_some(), true);
}

#[actix_rt::test]
async fn test_get_pokemon_result() -> Result<(), Error> {
    let app = App::new().data(load_config()).service(server::get_pokemon);
    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .uri(format!("/pokemon/{}", POKEMON).as_str())
        .to_request();
    let resp = app.call(req).await.unwrap();

    assert_eq!(resp.status(), http::StatusCode::OK); // Could fail due to translator API limits

    let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    match serde_json::from_str(str::from_utf8(&response_body).unwrap_or("")) {
        Ok::<PokemonResult, _>(result) => assert_eq!(result.name, POKEMON.to_string()),
        _ => panic!("JSON error"),
    }

    Ok(())
}
