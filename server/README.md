# doshtrade server
A graphql backend that stores and serves created find-replaces. Built with Rust and sqlite. Automatically integrates client html code as part of the binary.

## Requirements

- Rust 1.48+
- Postgres

## Debug

```bash
cargo build -j $(nproc)
IP_PORT=0.0.0.0:80 ./target/debug/doshtrade_server
```

## Setup

Start postgres
```bash
docker run --rm -p 5432:5432 -e POSTGRES_PASSWORD=mysecretpassword postgres:alpine postgres -c log_statement=all
```

## Deployment

#### Build and test locally
```bash
cargo build --release -j $(nproc)
IP_PORT=0.0.0.0:80 ./target/release/doshtrade_server
```

#### Build client, build server, deploy
```bash
# build client
(cd ../client && yarn build)

#build server
docker run --rm -v "$PWD":/doshtrade/server -v "$PWD/../client":/doshtrade/client -w /doshtrade/server rust:slim sh -c "apt-get update && apt-get install -y pkg-config libssl-dev && cargo build --release"

# copy files
scp -C target/release/doshtrade_server ubuntu@direct.doshtrade.com:~/doshtrade_server.swp

# restart service in new tmux
ssh ubuntu@direct.dostrade.com "cd /root && \
if [[ -f doshtrade_server.swp ]]; then mv doshtrade_server.swp doshtrade_server; fi && \
tmux kill-server; \
tmux new-session -d sh -i -c 'IP_PORT=0.0.0.0:80 /root/doshtrade_server'"
```