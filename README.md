PokéShakespear by Dario Cancelliere
=====
A simple Pokémon description fetcher with a Shakespear translation using PokéApi V2.

Rrequirements
---------
You need Docker in order to run this application. Visit https://docs.docker.com/get-docker/ in order to get the right Docker environment.

Running the application
---------
You will have to build the Docker container first, then, you can run the application through Docker using these commands:
* `docker build --no-cache --tag pokeshakespear .`
* `docker run --rm -p 5000:5000 pokeshakespear`

Once the application is running, visit the default URI: http://127.0.0.1:5000/ in order to see the default server page.
Visit http://127.0.0.1:5000/pokemon/raichu for example in order to see a Pokémon with their description.

Configuration
---------
You can change the configuration of the server and other setting just changing values inside `"config/config.toml"`.

Building
---------
You can build the application again for the Docker enviroment with the command: `cargo +stable build --release --target x86_64-unknown-linux-musl --locked`

If you want just to run the application in the host machine, please install the Rust enviroment and run the command: `cargo +stable run --release`

_Made with_ ❤️ _in Rust_