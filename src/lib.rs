//! WASM ABI Tools

#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![warn(missing_docs)]
#![cfg_attr(feature="strict", deny(unused))]


extern crate byteorder;
#[macro_use] extern crate uint;


#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate hex_literal;

#[cfg(feature = "hex")]
extern crate rustc_hex;

#[cfg(not(any(feature = "std", test)))]
extern crate alloc;

#[macro_use] extern crate fixed_hash;

#[cfg(feature = "serialize")]
#[macro_use] extern crate serde;

pub mod eth;

/// Custom types which AbiType supports
pub mod types;


mod lib {

	mod core {
		#[cfg(any(feature="std", test))]
		pub use std::*;
		#[cfg(not(any(feature="std", test)))]
		pub use core::*;
	}

	pub use self::core::{cmp, iter, mem, ops, slice, str};
	pub use self::core::{i8, i16, i32, i64, isize};
	pub use self::core::{u8, u16, u32, u64, usize};

	pub use self::core::cell::{Cell, RefCell};
	pub use self::core::clone::{self, Clone};
	pub use self::core::convert::{self, From, Into};
	pub use self::core::default::{self, Default};
	pub use self::core::fmt::{self, Debug, Display};
	pub use self::core::marker::{self, PhantomData};
	pub use self::core::option::{self, Option};
	pub use self::core::result::{self, Result};

	#[cfg(any(feature="std", test))]
	pub use std::borrow::{Cow, ToOwned};
	#[cfg(not(any(feature="std", test)))]
	pub use alloc::borrow::{Cow, ToOwned};

	#[cfg(any(feature="std", test))]
	pub use std::string::String;
	#[cfg(not(any(feature="std", test)))]
	pub use alloc::string::{String, ToString};

	#[cfg(any(feature="std", test))]
	pub use std::vec::Vec;
	#[cfg(not(any(feature="std", test)))]
	pub use alloc::vec::Vec;

	#[cfg(any(feature="std", test))]
	pub use std::boxed::Box;
	#[cfg(not(any(feature="std", test)))]
	pub use alloc::boxed::Box;

}
