// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::hyper::StatusCode;


#[inline(always)]
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<StatusCode, D::Error>
{
	struct StatusCodeVisitor;
	
	impl<'de> Visitor<'de> for StatusCodeVisitor
	{
		type Value = StatusCode;
		
		fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
		{
			formatter.write_str("a Status Code between 100 and 699")
		}
		
		fn visit_u8<E: DeserializeError>(self, value: u8) -> Result<Self::Value, E>
		{
			self.visit_u16(value as u16)
		}
		
		fn visit_u16<E: DeserializeError>(self, value: u16) -> Result<Self::Value, E>
		{
			match StatusCode::try_from(value)
			{
				Err(_) => Err(E::custom("status code is invalid")),
				Ok(statusCode) => Ok(statusCode),
			}
		}
		
		fn visit_u32<E: DeserializeError>(self, value: u32) -> Result<Self::Value, E>
		{
			if value > u16::max_value() as u32
			{
				return Err(E::custom("status code exceed u16 range"));
			}
			self.visit_u16(value as u16)
		}
		
		fn visit_u64<E: DeserializeError>(self, value: u64) -> Result<Self::Value, E>
		{
			if value > u16::max_value() as u64
			{
				return Err(E::custom("status code exceed u16 range"));
			}
			self.visit_u16(value as u16)
		}
		
		fn visit_i8<E: DeserializeError>(self, value: i8) -> Result<Self::Value, E>
		{
			if value < 0
			{
				return Err(E::custom("status code is negative"));
			}
			self.visit_u16(value as u16)
		}
		
		fn visit_i16<E: DeserializeError>(self, value: i16) -> Result<Self::Value, E>
		{
			if value < 0
			{
				return Err(E::custom("status code is negative"));
			}
			self.visit_u16(value as u16)
		}
		
		fn visit_i32<E: DeserializeError>(self, value: i32) -> Result<Self::Value, E>
		{
			if value < 0
			{
				return Err(E::custom("status code is negative"));
			}
			if value > u16::max_value() as i32
			{
				return Err(E::custom("status code exceed u16 range"));
			}
			self.visit_u16(value as u16)
		}
		
		fn visit_i64<E: DeserializeError>(self, value: i64) -> Result<Self::Value, E>
		{
			if value < 0
			{
				return Err(E::custom("status code is negative"));
			}
			if value > u16::max_value() as i64
			{
				return Err(E::custom("status code exceed u16 range"));
			}
			self.visit_u16(value as u16)
		}
	}
	
	deserializer.deserialize_u16(StatusCodeVisitor)
}

#[inline(always)]
pub(crate) fn serialize<S: Serializer>(value: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
{
	serializer.serialize_u16(value.as_u16())
}
