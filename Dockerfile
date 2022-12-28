FROM rust:latest
RUN mkdir /filedrop
WORKDIR /filedrop
VOLUME ["/var/file_drop_files"]
EXPOSE 8000
COPY . .

RUN cargo build --release
RUN sed -i "s|you'll have to replace that value|`openssl rand -base64 32`|g" Rocket.toml

CMD ["/filedrop/target/release/filedrop-v2"]