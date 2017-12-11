// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum SiteMapPriority
{
	zero,
	one,
	two,
	three,
	four,
	five,
	six,
	seven,
	eight,
	nine,
	ten,
}

impl Default for SiteMapPriority
{
	#[inline(always)]
	fn default() -> Self
	{
		SiteMapPriority::five
	}
}

impl<'de> Deserialize<'de> for SiteMapPriority
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		use self::SiteMapPriority::*;
		
		struct SiteMapPriorityVisitor;
		
		impl<'de> Visitor<'de> for SiteMapPriorityVisitor
		{
			type Value = SiteMapPriority;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("a priority with one decimal place between 0.0 and 1.0 inclusive")
			}
			
			#[inline(always)]
			fn visit_i64<E: DeserializeError>(self, v: i64) -> Result<Self::Value, E>
			{
				match v
				{
					0 => Ok(zero),
					1 => Ok(ten),
					_ => Err(E::custom("value was not zero or 1"))
				}
			}
			
			#[inline(always)]
			fn visit_u64<E: DeserializeError>(self, v: u64) -> Result<Self::Value, E>
			{
				match v
				{
					0 => Ok(zero),
					1 => Ok(ten),
					_ => Err(E::custom("value was not zero or 1"))
				}
			}
			
			#[inline(always)]
			fn visit_f64<E: DeserializeError>(self, v: f64) -> Result<Self::Value, E>
			{
				if v.is_nan()
				{
					return Err(E::custom("site map priority is NaN"))
				}
				
				if v.is_infinite()
				{
					return Err(E::custom("site map priority is infinite"))
				}
				
				if v.is_sign_negative()
				{
					if v == -0.0f64
					{
						return Ok(zero);
					}
					else
					{
						return Err(E::custom("site map priority is negative"))
					}
				}
				
				match (v * 10.0f64).trunc() as u64
				{
					0 => Ok(zero),
					1 => Ok(one),
					2 => Ok(two),
					3 => Ok(three),
					4 => Ok(four),
					5 => Ok(five),
					6 => Ok(six),
					7 => Ok(seven),
					8 => Ok(eight),
					9 => Ok(nine),
					10 => Ok(ten),
					_ => Err(E::custom("site map priority exceeds 1.0")),
				}
			}
		}
		
		deserializer.deserialize_f64(SiteMapPriorityVisitor)
	}
}

impl Serialize for SiteMapPriority
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_f64(self.as_f64())
	}
}

impl SiteMapPriority
{
	#[inline(always)]
	pub(crate) fn as_f64(&self) -> f64
	{
		use self::SiteMapPriority::*;
		
		match *self
		{
			zero => 0.0f64,
			one => 0.1f64,
			two => 0.2f64,
			three => 0.3f64,
			four => 0.4f64,
			five => 0.5f64,
			six => 0.6f64,
			seven => 0.7f64,
			eight => 0.8f64,
			nine => 0.9f64,
			ten => 1.0f64,
		}
	}
	
	#[inline(always)]
	pub(crate) fn as_str(&self) -> &'static str
	{
		use self::SiteMapPriority::*;
		
		match *self
		{
			zero => "0.0",
			one => "0.1",
			two => "0.2",
			three => "0.3",
			four => "0.4",
			five => "0.5",
			six => "0.6",
			seven => "0.7",
			eight => "0.8",
			nine => "0.9",
			ten => "1.0",
		}
	}
}
