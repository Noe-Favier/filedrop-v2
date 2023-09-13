#BUILDER 
FROM rust:alpine3.17
    RUN mkdir /builder
    WORKDIR /builder

    #Mandatory for build
    RUN apk upgrade -a 
    RUN apk add musl-dev

    #clone project
    COPY . .

    #build filedrop-v2
    RUN cargo build --release


#RUNNER
FROM alpine:3.18.3
    VOLUME ["/var/file_drop_files"]
    EXPOSE 8000

    RUN mkdir /filedrop
    WORKDIR /filedrop

    #needed to create secret in Rocket.toml with run-filedrop-v2.sh
    RUN apk -U add --no-cache openssl

    #copy all files from the builder
    COPY --from=0 /builder/templates ./templates
    COPY --from=0 /builder/assets ./assets
    COPY --from=0 /builder/Rocket.toml ./
    COPY --from=0 /builder/run-filedrop-v2.sh ./
    COPY --from=0 /builder/.env ./

    COPY --from=0 /builder/target/release/filedrop-v2 ./filedrop-v2

    RUN chmod u+x ./run-filedrop-v2.sh

    ENTRYPOINT ["/bin/sh", "/filedrop/run-filedrop-v2.sh"]