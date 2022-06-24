# EFM32WG (Wonder Gecko) support for Rust

[![PACs](https://github.com/efm32-rs/efm32wg-pacs/actions/workflows/pacs.yml/badge.svg)](https://github.com/efm32-rs/efm32wg-pacs/actions/workflows/pacs.yml)

This repository contains Peripheral Access Crates (PACs) for Silabs' EFM32 series of Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust](https://github.com/rust-embedded/svd2rust).

Refer to the [CHANGELOG](CHANGELOG.md) to see what changed in the last releases.

## Crates

Every EFM32WG chip has its own PAC, listed below:

| Crate            | Docs                                                                                   | crates.io                                                                                                   | target                  |
|------------------|----------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|-------------------------|
| `efm32wg230-pac` | [![docs.rs](https://docs.rs/efm32wg230-pac/badge.svg)](https://docs.rs/efm32wg230-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg230-pac.svg)](https://crates.io/crates/efm32wg230-pac) | `thumbv7em-none-eabihf` |
| `efm32wg232-pac` | [![docs.rs](https://docs.rs/efm32wg232-pac/badge.svg)](https://docs.rs/efm32wg232-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg232-pac.svg)](https://crates.io/crates/efm32wg232-pac) | `thumbv7em-none-eabihf` |
| `efm32wg280-pac` | [![docs.rs](https://docs.rs/efm32wg280-pac/badge.svg)](https://docs.rs/efm32wg280-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg280-pac.svg)](https://crates.io/crates/efm32wg280-pac) | `thumbv7em-none-eabihf` |
| `efm32wg290-pac` | [![docs.rs](https://docs.rs/efm32wg290-pac/badge.svg)](https://docs.rs/efm32wg290-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg290-pac.svg)](https://crates.io/crates/efm32wg290-pac) | `thumbv7em-none-eabihf` |
| `efm32wg295-pac` | [![docs.rs](https://docs.rs/efm32wg295-pac/badge.svg)](https://docs.rs/efm32wg295-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg295-pac.svg)](https://crates.io/crates/efm32wg295-pac) | `thumbv7em-none-eabihf` |
| `efm32wg330-pac` | [![docs.rs](https://docs.rs/efm32wg330-pac/badge.svg)](https://docs.rs/efm32wg330-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg330-pac.svg)](https://crates.io/crates/efm32wg330-pac) | `thumbv7em-none-eabihf` |
| `efm32wg332-pac` | [![docs.rs](https://docs.rs/efm32wg332-pac/badge.svg)](https://docs.rs/efm32wg332-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg332-pac.svg)](https://crates.io/crates/efm32wg332-pac) | `thumbv7em-none-eabihf` |
| `efm32wg360-pac` | [![docs.rs](https://docs.rs/efm32wg360-pac/badge.svg)](https://docs.rs/efm32wg360-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg360-pac.svg)](https://crates.io/crates/efm32wg360-pac) | `thumbv7em-none-eabihf` |
| `efm32wg380-pac` | [![docs.rs](https://docs.rs/efm32wg380-pac/badge.svg)](https://docs.rs/efm32wg380-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg380-pac.svg)](https://crates.io/crates/efm32wg380-pac) | `thumbv7em-none-eabihf` |
| `efm32wg390-pac` | [![docs.rs](https://docs.rs/efm32wg390-pac/badge.svg)](https://docs.rs/efm32wg390-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg390-pac.svg)](https://crates.io/crates/efm32wg390-pac) | `thumbv7em-none-eabihf` |
| `efm32wg395-pac` | [![docs.rs](https://docs.rs/efm32wg395-pac/badge.svg)](https://docs.rs/efm32wg395-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg395-pac.svg)](https://crates.io/crates/efm32wg395-pac) | `thumbv7em-none-eabihf` |
| `efm32wg840-pac` | [![docs.rs](https://docs.rs/efm32wg840-pac/badge.svg)](https://docs.rs/efm32wg840-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg840-pac.svg)](https://crates.io/crates/efm32wg840-pac) | `thumbv7em-none-eabihf` |
| `efm32wg842-pac` | [![docs.rs](https://docs.rs/efm32wg842-pac/badge.svg)](https://docs.rs/efm32wg842-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg842-pac.svg)](https://crates.io/crates/efm32wg842-pac) | `thumbv7em-none-eabihf` |
| `efm32wg880-pac` | [![docs.rs](https://docs.rs/efm32wg880-pac/badge.svg)](https://docs.rs/efm32wg880-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg880-pac.svg)](https://crates.io/crates/efm32wg880-pac) | `thumbv7em-none-eabihf` |
| `efm32wg890-pac` | [![docs.rs](https://docs.rs/efm32wg890-pac/badge.svg)](https://docs.rs/efm32wg890-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg890-pac.svg)](https://crates.io/crates/efm32wg890-pac) | `thumbv7em-none-eabihf` |
| `efm32wg895-pac` | [![docs.rs](https://docs.rs/efm32wg895-pac/badge.svg)](https://docs.rs/efm32wg895-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg895-pac.svg)](https://crates.io/crates/efm32wg895-pac) | `thumbv7em-none-eabihf` |
| `efm32wg900-pac` | [![docs.rs](https://docs.rs/efm32wg900-pac/badge.svg)](https://docs.rs/efm32wg900-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg900-pac.svg)](https://crates.io/crates/efm32wg900-pac) | `thumbv7em-none-eabihf` |
| `efm32wg940-pac` | [![docs.rs](https://docs.rs/efm32wg940-pac/badge.svg)](https://docs.rs/efm32wg940-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg940-pac.svg)](https://crates.io/crates/efm32wg940-pac) | `thumbv7em-none-eabihf` |
| `efm32wg942-pac` | [![docs.rs](https://docs.rs/efm32wg942-pac/badge.svg)](https://docs.rs/efm32wg942-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg942-pac.svg)](https://crates.io/crates/efm32wg942-pac) | `thumbv7em-none-eabihf` |
| `efm32wg980-pac` | [![docs.rs](https://docs.rs/efm32wg980-pac/badge.svg)](https://docs.rs/efm32wg980-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg980-pac.svg)](https://crates.io/crates/efm32wg980-pac) | `thumbv7em-none-eabihf` |
| `efm32wg990-pac` | [![docs.rs](https://docs.rs/efm32wg990-pac/badge.svg)](https://docs.rs/efm32wg990-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg990-pac.svg)](https://crates.io/crates/efm32wg990-pac) | `thumbv7em-none-eabihf` |
| `efm32wg995-pac` | [![docs.rs](https://docs.rs/efm32wg995-pac/badge.svg)](https://docs.rs/efm32wg995-pac) | [![crates.io](https://img.shields.io/crates/d/efm32wg995-pac.svg)](https://crates.io/crates/efm32wg995-pac) | `thumbv7em-none-eabihf` |

Process finished with exit code 0


## Device Reference Manuals from Silabs

**WIP**

## License

The included SVD files are sourced from https://www.silabs.com/documents/public/cmsis-packs and
are licensed under the Zlib (see [LICENSE-3RD-PARTY](LICENSE-3RD-PARTY-Zlib)).

The remainder of the code is under:

- 3-Clause BSD license ([LICENSE-3BSD](LICENSE-3BSD) or https://opensource.org/licenses/BSD-3-Clause)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the BSD-3-Clause license without any additional terms or conditions.