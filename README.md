# Hackernews API crawler
- Forked from: https://github.com/zhangjinpeng1987/hackernews-crawler
- I reimplement the project to learn Rust.

## Todo:
- [x] Generic store
  - [x] Postgres
  - [x] Sqlite
  - [-] File
- [x] Asynchronous API
- [ ] Real-time event

# How to?
## Build:
```shell
cargo build
```

## Test:
- Test specified module
```shell
cargo test ext::sqlite 
```
- or test all
```shell
cargo test
```

## Run:
- Store to Sqlite:
```shell
crawler --store sqlite --store-uri=<db/file/path>
```
- Store to Postgres:
```shell
crawler --store postgres --store-uri=<postgresql://uri>
```