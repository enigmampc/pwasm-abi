//! Provides primitive fixed size hash types.

// Constructed via `fixed_hash` crate macro.

pub use uint::U256;

#[cfg(feature = "std")]
pub use std::{string::String, vec::Vec};
#[cfg(not(feature = "std"))]
pub use alloc::{string::String, vec::Vec};

use lib::ops::Deref;
// pub use parity_hash::*;

construct_fixed_hash!{
	/// A 160 bits (20 bytes) hash type.
	///
	/// # Note
	///
	/// Can be interchanged with `Address`
    #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
	pub struct H160(20);
}

construct_fixed_hash!{
	/// A 256-bits (32 bytes) hash type.
	#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
    pub struct H256(32);
}

// Auto-impl `From` conversions between `H256` and `H160`.
impl_fixed_hash_conversions!(H256, H160);

/// Represents an address in ethereum context.
/// 
/// # Note
/// 
/// Addresses have 160 bytes length.
pub type Address = H160;

impl From<U256> for H256 {
	fn from(uint: U256) -> H256 {
		let mut hash = H256::zero();
		uint.to_big_endian(hash.as_bytes_mut());
		hash
	}
}

impl<'a> From<&'a U256> for H256 {
	fn from(uint: &'a U256) -> H256 {
		let mut hash: H256 = H256::zero();
		uint.to_big_endian(hash.as_bytes_mut());
		hash
	}
}

impl From<H256> for U256 {
	fn from(hash: H256) -> U256 {
		U256::from(hash.as_bytes())
	}
}

impl<'a> From<&'a H256> for U256 {
	fn from(hash: &'a H256) -> U256 {
		U256::from(hash.as_bytes())
	}
}

impl Deref for H256 {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for H160 {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &self.0
    }
}



impl<'a> From<&'a [u8]> for H160 {
    fn from(s: &'a [u8]) -> Self {
        Self::from_slice(s)
    }
}

impl<'a> From<&'a [u8]> for H256 {
    fn from(s: &'a [u8]) -> Self {
        Self::from_slice(s)
    }
}