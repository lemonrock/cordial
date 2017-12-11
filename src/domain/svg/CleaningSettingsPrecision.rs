// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct CleaningSettingsPrecision(u8);

impl Default for CleaningSettingsPrecision
{
	#[inline(always)]
	fn default() -> Self
	{
		CleaningSettingsPrecision(1)
	}
}

impl CleaningSettingsPrecision
{
	#[inline(always)]
	fn to_u8(&self) -> u8
	{
		self.0
	}
}

impl<'de> Deserialize<'de> for CleaningSettingsPrecision
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct CleaningSettingsPrecisionVisitor;
		
		impl<'de> Visitor<'de> for CleaningSettingsPrecisionVisitor
		{
			type Value = CleaningSettingsPrecision;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a priority with one decimal place between 0.0 and 1.0 inclusive")
			}
			
			#[inline(always)]
			fn visit_i64<E: DeserializeError>(self, v: i64) -> Result<Self::Value, E>
			{
				if v < 1 || v > 12
				{
					Err(E::custom("value was not between 1 to 12 inclusive"))
				}
				else
				{
					Ok(CleaningSettingsPrecision(v as u8))
				}
			}
			
			#[inline(always)]
			fn visit_u64<E: DeserializeError>(self, v: u64) -> Result<Self::Value, E>
			{
				if v < 1 || v > 12
				{
					Err(E::custom("value was not between 1 to 12 inclusive"))
				}
				else
				{
					Ok(CleaningSettingsPrecision(v as u8))
				}
			}
		}
		
		deserializer.deserialize_u64(CleaningSettingsPrecisionVisitor)
	}
}

impl Serialize for CleaningSettingsPrecision
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_u8(self.0)
	}
}
