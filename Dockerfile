FROM rust:1.76

RUN useradd -ms /bin/bash game
USER game
WORKDIR /home/game/
COPY . .

RUN cargo build --release
ENTRYPOINT ["./target/release/cli_game"]
