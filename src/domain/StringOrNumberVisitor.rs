// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


struct StringOrNumberVisitor<T>(PhantomData<fn() -> T>);

impl<'de, T: FromStringOrNumber> Visitor<'de> for StringOrNumberVisitor<T>
{
	type Value = T;
	
	fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
	{
		formatter.write_str("string or number")
	}
	
	fn visit_str<E: DeserializeError>(self, value: &str) -> Result<T, E>
	{
		Ok(FromStringOrNumber::from_str(value))
	}
	
	fn visit_borrowed_str<E: DeserializeError>(self, value: &'de str) -> Result<T, E>
	{
		Ok(FromStringOrNumber::from_str(value))
	}
	
	fn visit_string<E: DeserializeError>(self, value: String) -> Result<T, E>
	{
		Ok(FromStringOrNumber::from_str(&value))
	}
	
	fn visit_i8<E: DeserializeError>(self, value: i8) -> Result<T, E>
	{
		if value < 0
		{
			return Err(E::custom(format!("value is negative: {}", value)));
		}
		self.visit_u32(value as u32)
	}
	
	fn visit_i16<E: DeserializeError>(self, value: i16) -> Result<T, E>
	{
		if value < 0
		{
			return Err(E::custom(format!("value is negative: {}", value)));
		}
		self.visit_u32(value as u32)
	}
	
	fn visit_i32<E: DeserializeError>(self, value: i32) -> Result<T, E>
	{
		if value < 0
		{
			return Err(E::custom(format!("value is negative: {}", value)));
		}
		self.visit_u32(value as u32)
	}
	
	fn visit_i64<E: DeserializeError>(self, value: i64) -> Result<T, E>
	{
		if value < 0
		{
			return Err(E::custom(format!("value is negative: {}", value)));
		}
		if value > u32::max_value() as i64
		{
			return Err(E::custom(format!("value is too big: {}", value)));
		}
		self.visit_u32(value as u32)
	}
	
	fn visit_u8<E: DeserializeError>(self, value: u8) -> Result<T, E>
	{
		self.visit_u32(value as u32)
	}
	
	fn visit_u16<E: DeserializeError>(self, value: u16) -> Result<T, E>
	{
		self.visit_u32(value as u32)
	}
	
	fn visit_u32<E: DeserializeError>(self, value: u32) -> Result<T, E>
	{
		Ok(FromStringOrNumber::from_u32(value))
	}
	
	fn visit_u64<E: DeserializeError>(self, value: u64) -> Result<T, E>
	{
		if value > u32::max_value() as u64
		{
			return Err(E::custom(format!("value is too big: {}", value)));
		}
		self.visit_u32(value as u32)
	}
}
