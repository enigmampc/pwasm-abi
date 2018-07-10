#![allow(dead_code)]

use pwasm_abi::eth::EndpointInterface;
use pwasm_abi_derive::eth_abi;

#[eth_abi(TupleReturnEndpoint, TupleReturnClient)]
pub trait TupleReturnContract {
	fn ret2(&mut self) -> (u64, u64);
	fn ret6(&mut self) -> (u64, u64, u64, u64, u64, u64);
}

#[test]
fn bytes16() {
	pub struct Instance;

	impl TupleReturnContract for Instance {
		fn ret2(&mut self) -> (u64, u64) {
			(2, 2)
		}
		fn ret6(&mut self) -> (u64, u64, u64, u64, u64, u64) {
			(6, 6, 6, 6, 6, 6)
		}
	}

	let mut endpoint = TupleReturnEndpoint::new(Instance);

	let res2 = endpoint.dispatch(&[0xa6, 0x37, 0xe6, 0x9c]);
	assert_eq!(&res2[..], &[
		0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2
	][..]);

	let res6 = endpoint.dispatch(&[0x8f, 0x34, 0x96, 0x73]);
	assert_eq!(&res6[..], &[
 		0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6
	][..]);
}