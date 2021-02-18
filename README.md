# doshTrade
A trade curation service 

## Demo

Client runs on http://localhost:3000

Server runs on http://localhost:8080

From git root, run:

```bash
(cd client && yarn build)
docker run -d --rm -p 5432:5432 -e POSTGRES_PASSWORD=mysecretpassword postgres:alpine postgres -c log_statement=all
(cd server && cargo build && RUST_LOG=INFO ./target/debug/doshtrade_server)
```
