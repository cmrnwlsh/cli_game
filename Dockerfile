FROM rust:1.76

RUN useradd -ms /bin/bash game
USER game
WORKDIR /home/game/
COPY . .

ENTRYPOINT ["./target/release/cli_game"]
