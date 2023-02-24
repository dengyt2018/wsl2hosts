```bat
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --bin wsl2hosts --release --target=x86_64-pc-windows-msvc
cargo build --release

```

`wsl2hosts` # autorun program
`wsl2hosts install` # install as service, service name wsl2hosts.service.1
`wsl2hosts remove`  # remove service
```