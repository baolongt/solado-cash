# Issue List

## 

- [Error: IDL doesnt exist. | Rust | Solana | Anchor](https://stackoverflow.com/questions/78137225/error-idl-doesnt-exist-rust-solana-anchor)

I was on Anchor 0.30.0 which introduced new changes with the [IDL build feature](https://www.anchor-lang.com/release-notes/0.30.0#idl-build-feature)

```bash
avm use 0.30.0
```

change file Cargo.toml

```toml
[features]
idl-build = ["anchor-lang/idl-build"]
```

- Unable to read keypair file - [fix](https://www.soldev.app/course/local-setup)

```bash
solana-keygen new --no-bip39-passphrase
```
