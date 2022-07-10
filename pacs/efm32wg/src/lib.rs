//! Peripheral access API for EFM32WG microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.24.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.24.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32wg-pacs)
//!
//! This crate supports all EFM32WG devices; for the complete list please see:
//! [efm32wg](https://github.com/efm32-rs/efm32wg-pacs/pacs/efm32wg)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32wg230")]
pub mod efm32wg230;

#[cfg(feature = "efm32wg232")]
pub mod efm32wg232;

#[cfg(feature = "efm32wg280")]
pub mod efm32wg280;

#[cfg(feature = "efm32wg290")]
pub mod efm32wg290;

#[cfg(feature = "efm32wg295")]
pub mod efm32wg295;

#[cfg(feature = "efm32wg330")]
pub mod efm32wg330;

#[cfg(feature = "efm32wg332")]
pub mod efm32wg332;

#[cfg(feature = "efm32wg360")]
pub mod efm32wg360;

#[cfg(feature = "efm32wg380")]
pub mod efm32wg380;

#[cfg(feature = "efm32wg390")]
pub mod efm32wg390;

#[cfg(feature = "efm32wg395")]
pub mod efm32wg395;

#[cfg(feature = "efm32wg840")]
pub mod efm32wg840;

#[cfg(feature = "efm32wg842")]
pub mod efm32wg842;

#[cfg(feature = "efm32wg880")]
pub mod efm32wg880;

#[cfg(feature = "efm32wg890")]
pub mod efm32wg890;

#[cfg(feature = "efm32wg895")]
pub mod efm32wg895;

#[cfg(feature = "efm32wg900")]
pub mod efm32wg900;

#[cfg(feature = "efm32wg940")]
pub mod efm32wg940;

#[cfg(feature = "efm32wg942")]
pub mod efm32wg942;

#[cfg(feature = "efm32wg980")]
pub mod efm32wg980;

#[cfg(feature = "efm32wg990")]
pub mod efm32wg990;

#[cfg(feature = "efm32wg995")]
pub mod efm32wg995;
