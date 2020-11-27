## local setup

_install rust_
``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

_update rust_
``` bash
rustup self update
rustup update
```

_install cargo-watch and clippy_
``` bash
rustup component add clippy
cargo install cargo-watch
```

_rebuild on code change_
``` bash
cargo watch -x check -x clippy -x test -x run
```
