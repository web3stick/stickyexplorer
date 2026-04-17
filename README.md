# stickyexplorer
fast near blockchain explorer using fastnear explorer api and dioxus ui

---

### Dev and Build

```sh
# dx
dx check
dx fmt
# serve
dx serve
dx serve --platform web
# dx serve --platform desktop
# bundle
dx bundle --platform web
dx bundle --release
# dx bundle --platform desktop

# CARGO COMMANDS
# cargo run
cargo check
cargo test
cargo clean
cargo check --target wasm32-unknown-unknown
cargo fmt
cargo update

# iced ui
cargo build --no-default-features --features iced_desktop --bin stickyexplorer_iced_main
cargo run  --no-default-features --features iced_desktop --bin stickyexplorer_iced_main


# netlify
# stickyweb-stickyexplorer
netlify deploy
netlify deploy --prod
```



---

copyright 2026 by sleet.near