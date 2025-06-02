#![cfg_attr(target_os = "solana", feature(asm_experimental_arch))]

#[cfg(test)]
mod tests;

#[cfg(all(target_os = "solana", feature = "native_rust"))]
mod native_rust;
#[cfg(all(target_os = "solana", feature = "native_rust"))]
use native_rust::*;

#[cfg(feature = "optimized")]
mod optimized;
#[cfg(feature = "optimized")]
use optimized::*;

#[cfg(all(target_os = "solana", feature = "native"))]
mod native;
#[cfg(all(target_os = "solana", feature = "native"))]
use native::*;
