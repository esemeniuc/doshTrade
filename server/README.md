# doshtrade server
A graphql backend that stores stock and option info. Built with Rust and postgres. Automatically integrates client html code as part of the binary.

## Requirements

- Rust 1.50+
- Postgres

## Local Development
### Build client
```
(cd ../client && yarn build)
```

### Start Postgres
```bash
docker run --rm -p 5432:5432 -e POSTGRES_PASSWORD=mysecretpassword postgres:alpine postgres -c log_statement=all
```

### Build Server
```bash
cargo build
RUST_LOG=INFO target/debug/doshtrade_server
```
Server runs on http://localhost:8080

## Production

### Build client, build server, deploy
```bash
# build client
(cd ../client && yarn build)

#build server
docker run --rm -v "$PWD":/doshtrade/server -v "$PWD/../client":/doshtrade/client -w /doshtrade/server rust:slim sh -c "apt-get update && apt-get install -y pkg-config libssl-dev && cargo build --release"

# copy files
scp -C target/release/doshtrade_server root@direct.doshtrade.com:~/doshtrade_server.swp

# restart service in new tmux
ssh root@direct.dostrade.com "cd ~ && \
if [[ -f doshtrade_server.swp ]]; then mv doshtrade_server.swp doshtrade_server; fi && \
tmux kill-server; \
tmux new-session -d sh -i -c 'sudo ~/doshtrade_server --port 80'"
```
