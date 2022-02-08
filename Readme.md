# Coding test notes

[![Rust](https://github.com/dymayday/super-duper-octo-echo/actions/workflows/rust.yml/badge.svg)](https://github.com/dymayday/super-duper-octo-echo/actions/workflows/rust.yml)

Sending a payload to a echo server and using a fail over system in case of client interuption.

## Run

Run it in two separate consoles.

```console
    RUST_LOG=debug cargo run --bin server
```

Use `Ctrl-c` to trigger an interuption event.

```console
    RUST_LOG=debug cargo run --bin client
```

## Thing to discuss

- A simple fail over system has been implemented using `tokio::select`, but we could use something like a heartbeat mechanism instead which would require that the salve client would be run in the background. And it would minimize latency and downtime.
