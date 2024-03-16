use std::mem::size_of;

use bytes::{Buf, BufMut, BytesMut};

use crate::callback::{CallbackInput, CallbackOutput};

impl CallbackInput for String {
	fn deserialize(input: &mut BytesMut) -> Self {
		let len = input.get_u32();
		String::from_utf8_lossy(&input.split_to(len as usize)).into_owned()
	}
}

impl CallbackOutput for String {
	fn serialize(self) -> BytesMut {
		BytesMut::from_iter(self.into_bytes())
	}
}

impl CallbackInput for () {
	fn deserialize(_input: &mut BytesMut) -> Self {}
}

impl CallbackOutput for () {
	fn serialize(self) -> BytesMut {
		BytesMut::new()
	}
}

impl<T: CallbackInput> CallbackInput for Vec<T> {
	fn deserialize(input: &mut BytesMut) -> Self {
		let len = input.get_u32() as usize;
		let mut vec = Vec::with_capacity(len);
		for _ in 0..len {
			vec.push(T::deserialize(input));
		}
		vec
	}
}

impl<T: CallbackOutput> CallbackOutput for Vec<T> {
	#[allow(clippy::cast_possible_truncation)]
	fn serialize(self) -> BytesMut {
		let mut buf = BytesMut::with_capacity((self.len() * size_of::<T>()) + size_of::<u32>());
		buf.put_u32(self.len() as u32);
		for ele in self {
			buf.extend(ele.serialize());
		}
		buf
	}
}

macro_rules! impl_for_nums {
	($($ty:ty),*) => {
		$(
			impl CallbackInput for $ty {
				fn deserialize(input: &mut BytesMut) -> Self {
					::paste::paste! { input.[<get_$ty>]() }
				}
			}

			impl CallbackOutput for $ty {
				fn serialize(self) -> BytesMut {
					BytesMut::from_iter(self.to_be_bytes())
				}
			}
		)*
	};
}

impl_for_nums!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);
