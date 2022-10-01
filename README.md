# FileDrop V2 ?
this web app is a reengineering of [FileDrop V1](https://github.com/Noe-Favier/file_drop) which is a rust app created without any dependency.

Here, FileDrop V2 was realised with **A LOT** of lib. Furthermore, it was realised insanely fastser 
(*4~5Hours for V2 // +10Hours V1*). 

# How to setup :

## .env
- **files_path** : (*default* : `/files`) > path to the folder where files will be stored
- **allow_create** : (*default* : `true`) > is FileDrop allowed to create that dir if it doesn't exists ?


## tls support : 
To enable tls support (https), go in the folder [`ssl`](ssl) and then run :

    openssl req -new -newkey rsa:4096 -x509 -sha256 -days 365 -nodes -out cert.crt -keyout key.key

## More conf !

This project was realised with [rocket.rs](https://rocket.rs/). U will find the conf file of the web server in [rocket.toml](rocket.toml). 

for example, u will be able to : 
- change the port 
- disable tls (removing the `[default.tls]` part)

