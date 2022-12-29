#BUILDER 
FROM rust:alpine3.17
    RUN mkdir /builder
    WORKDIR /builder

    COPY . .

    RUN apk upgrade -a 

    #Mandatory for build
    RUN apk add musl-dev  

    #build filedrop-v2 (~600mb)
    RUN cargo build --release


#RUNNER
FROM alpine:latest
    VOLUME ["/var/file_drop_files"]
    EXPOSE 8000

    RUN mkdir /filedrop
    WORKDIR /filedrop

    #needed to create secret in Rocket.toml with run-filedrop-v2.sh
    RUN apk upgrade -a
    RUN apk add --no-cache openssl

    #copy all files from the builder
    COPY --from=0 /builder/ ./

    #move the exec from ./target to ./
    RUN mv /filedrop/target/release/filedrop-v2 ./filedrop-v2
    
    #remove useless stuff
    RUN rm -rf /filedrop/target
    RUN rm -rf /filedrop/src
    RUN rm -f /Cargo.lock
    RUN rm -f /Cargo.toml

    CMD ["/filedrop/run-filedrop-v2.sh"]