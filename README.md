# Hackernews API crawler
- Forked from: https://github.com/zhangjinpeng1987/hackernews-crawler
- I reimplement the project to learn Rust.

## Todo:
- [ ] Generic store
  - [x] Postgres
  - [x] Sqlite
  - [ ] File
- [ ] Asynchronous API
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