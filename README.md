# Friendly fire will not be tolerated

This is an attempt of partecipation for the bevy jam 4th edition.

* **URL**: [Bevy Jam #4](https://itch.io/jam/bevy-jam-4)
* **Theme**: Lot of entities

## Prerequisites

* ` wasm-server-runner`
  ```bash
  cargo install wasm-server-runner
  ```

## Commands

- Dev
  ```bash
  cargo watch -x 'run --target wasm32-unknown-unknown'
  ```

* Build
  ```bash
  cargo build --release --target wasm32-unknown-unknown

  wasm-bindgen --no-typescript --target web \
      --out-dir ./www/wasm \
      --out-name "bevyjam4" \
      ./target/wasm32-unknown-unknown/release/bevy_jam_4.wasm

  cp -R assets/* wwww/public/*
  ```
