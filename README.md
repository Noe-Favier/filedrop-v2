# FileDrop V2 ?

## Story

this web app is a reengineering of [FileDrop V1](https://github.com/Noe-Favier/file_drop) which is a rust app created without any dependency.

Here, FileDrop V2 was realised with **A LOT** of lib. Furthermore, it was realised insanely fastser (and working better)
(*4Hours for V2 // +10Hours V1*) to achieve the same thing.

## Demo

FiledropV2 is running at [noais.fr](https://noais.fr) if u wanna see

# How to setup :

## .env

settings which can be set in [.env](.env) :

- **files_path** : (*default* : `/var/file_drop_files`) > path to the folder where files will be stored
- **allow_create** : (*default* : `true`) > is FileDrop allowed to create that dir if it doesn't exists ?

## More conf

This project was realised with [rocket.rs](https://rocket.rs/). U will find the conf file of the web server in [Rocket.toml](Rocket.toml).

for example, u will be able to :

- change the port
- enable tls (=https) (if you can)
- change the secret key *(see below)*

### Changing secret_key

**IMPORTANT !**
on linux, run `openssl rand -base64 32` and replace the field 'secret_key' with the value returned by that command in [Rocket.toml](Rocket.toml).
More info about that [here](https://rocket.rs/v0.5-rc/guide/configuration/#secret-key).

# How to run

FileDrop is written in rust, you will be able to run it like a normal rust app.

if you need help :

- [install rust](https://www.rust-lang.org/tools/install)
- [cargo run](https://doc.rust-lang.org/cargo/commands/cargo-run.html#examples)

## Docker

> see [the docker hub repo](https://hub.docker.com/r/noecl/filedrop)

`docker run -p 80:8000 --name filedrop -it -d noecl/filedrop`
