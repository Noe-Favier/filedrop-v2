#BUILDER 
FROM rust:alpine3.17
    RUN mkdir /builder
    WORKDIR /builder

    COPY . .

    RUN apk upgrade -a 

    #Mandatory for build
    RUN apk add musl-dev  

    RUN cargo build --release


#RUNNER
FROM alpine:latest
    VOLUME ["/var/file_drop_files"]
    EXPOSE 8000

    RUN mkdir /filedrop
    WORKDIR /filedrop

    RUN apk upgrade -a
    RUN apk add --no-cache openssl

    COPY --from=0 /builder/ ./

    RUN mv /filedrop/target/release/filedrop-v2 ./filedrop-v2
    
    RUN rm -rf /filedrop/target
    RUN rm -rf /filedrop/src
    RUN rm -f /Cargo.lock
    RUN rm -f /Cargo.toml

    CMD ["/filedrop/run-filedrop-v2.sh"]