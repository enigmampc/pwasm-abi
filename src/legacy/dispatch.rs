use byteorder::{BigEndian, ByteOrder};
use tiny_keccak::Keccak;

use super::{Signature, ValueType};
use super::util::Error;

pub struct HashSignature {
    hash: u32,
    signature: Signature,
}

pub struct NamedSignature {
	name: &'static str,
	signature: Signature,
}

#[derive(Default)]
pub struct Table {
	// vec instead of hashmap since dispatch table is usually small (todo: maybe add variant with hash tables)
	inner: Vec<HashSignature>,
}

impl From<NamedSignature> for HashSignature {
	fn from(named: NamedSignature) -> HashSignature {
		let name = named.name;
		let signature = named.signature;
		let mut signature_str = String::from(name);
		signature_str.push('(');
		for (i, p) in signature.params().iter().enumerate() { 
			p.to_member(&mut signature_str);
			if i != signature.params().len()-1 { signature_str.push(','); }
		}
		signature_str.push(')');

		let mut keccak = Keccak::new_keccak256();
		let mut res = [0u8; 32];
		keccak.update(signature_str.as_bytes());
		keccak.finalize(&mut res);

		HashSignature {
			hash: BigEndian::read_u32(&res[0..4]),
			signature: signature
		}
	}
}

impl Table {
	pub fn new(inner: Vec<HashSignature>) -> Self {
		Table { inner: inner }
	}

	pub fn push<S>(&mut self, signature: S) where S: Into<HashSignature> {
		self.inner.push(signature.into())
	}

	pub fn dispatch<D>(&self, payload: Vec<u8>, mut d: D) 
		-> Result<Vec<u8>, Error> 
		where D: FnMut(u32, Vec<ValueType>) -> Option<ValueType>
	{
		let mut payload = payload;
		if payload.len() < 4 { return Err(Error); }
		let method: Vec<u8> = payload.drain(0..4).collect();
		let method_id = BigEndian::read_u32(&method);

		let hash_signature = self.inner.iter().find(|x| x.hash == method_id).ok_or(Error)?;

		let args = hash_signature.signature.decode_invoke(&payload);
		let result = d(method_id, args);

		Ok(hash_signature.signature.encode_result(result)?)
	}
}

#[test]
fn match_signature() {

	use super::ParamType;

	let named = NamedSignature {
		name: "baz",
		signature: Signature::new_void(vec![ParamType::U32, ParamType::Bool]),
	};

	let hashed: HashSignature = named.into();

	assert_eq!(hashed.hash, 0xcdcd77c0);
}

#[test]
fn match_signature_2() {

	use super::ParamType;

	let named = NamedSignature {
		name: "sam",
		signature: Signature::new_void(vec![ParamType::Bytes, ParamType::Bool, ParamType::Array(Box::new(ParamType::U256))]),
	};

	let hashed: HashSignature = named.into();

	assert_eq!(hashed.hash, 0xa5643bf2);
}