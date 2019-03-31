# remote-hal

A JSON RPC over TCP remote [embedded-hal](https://github.com/rust-embedded/embedded-hal) implementation, so you can develop embedded-hal based drivers on one machine and test them on a different one that has a [linux-embedded-hal](https://github.com/rust-embedded/linux-embedded-hal) implementation.

## Status

[![GitHub tag](https://img.shields.io/github/tag/ryankurte/rust-remote-hal.svg)](https://github.com/ryankurte/rust-remote-hal)
[![Build Status](https://travis-ci.com/ryankurte/rust-remote-hal.svg?branch=master)](https://travis-ci.com/ryankurte/rust-remote-hal)
[![Crates.io](https://img.shields.io/crates/v/remote-hal.svg)](https://crates.io/crates/remote-hal)
[![Docs.rs](https://docs.rs/remote-hal/badge.svg)](https://docs.rs/remote-hal)

[Open Issues](https://github.com/ryankurte/rust-remote-hal/issues)


## Usage

See [src/bin/client.rs](src/bin/client.rs) for a simple remote client, or [src/server/mod.rs](src/server/mod.rs) for server commands.

- `cargo install remote-hal` to install
- `rhd` to run the remote-hal-daemon (or `rhd --help` to list options)
- `rhc` to run the remote-hal-cli (or `rhc --help` to list options)

Note that this provides no mechanisms for secure communication, and thus should only be run on trusted networks.
