# beam-proto-rs

Rust codes generated from protocol buffer files in the [apache/beam](https://github.com/apache/beam) repository.

This repository also provides a build script to compile the protobuf into Rust codes.

## Versioning

This crate basically follows the beam's versions.

- `v2.41.0` is generated from `.proto`s from [apache/beam](https://github.com/apache/beam) tag:v2.41.0.
- Some versions may have build numbers like `v2.41.0+N`, which means the N-th build (N > 1) generated from beam's tag:v2.41.0. 2nd and later builds are typically for bugfixes.

## Development

### Build

Generating `.rs` from beam's `.proto` requires `build-proto` feature flag.

```bash
cargo build --features build-proto -vv
```

`-vv` is to show `eprintln!()` strings to your terminal.

### Updating Beam

`beam/` is a git submodule.

```console
cd beam
git fetch --tags
git checkout v<NEW VERSION>
cd ..
git commit -am 'build: bump up beam to v<NEW VERSION>'
```

### Deployment

1. Change version in `Cargo.toml`.
2. Update Beam.
3. Shrink submodule size to avoid 413 error from crates.io: `cd beam; git sparse-checkout set model; cd ..`
4. In the `main` branch: `cargo publish`.

## License

Licensed under [Apache License, Version 2.0](LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in beam-proto-rs by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

Copyright (c) 2022 Sho Nakatani \<lay.sakura@gmail.com\>.
