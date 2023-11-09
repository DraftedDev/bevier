# Bevier - Bevy Game Template Generator

[![crates.io](https://img.shields.io/crates/v/bevier)](https://crates.io/crates/bevier)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

**Generate Bevy Projects from various templates and configs!**

## Usage

Enter `bevier new` and complete the prompts:

- `Project Name`: The name of your project (will not change the template crate name).
- `Project Template`: The template you want to use.
- `Project Config`: The cargo config to generate (`.cargo/config.toml`).
- `Bevy Features`: The bevy features to use
- `Do you want to continue?`: Whether you want to continue (`y` or `n`).

## Bevy Support

| Bevy Version   | Crate Version   |
| -------------- | --------------- |
| `0.12`         | `1.1.0`         |
| -------------- | --------------- |
| `0.11`         | `1.0.0`         |
| -------------- | --------------- |
