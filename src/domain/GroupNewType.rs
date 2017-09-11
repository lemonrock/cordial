// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
struct GroupNewType(Group);

impl<'de> Deserialize<'de> for GroupNewType
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		deserializer.deserialize_any(StringOrNumberVisitor(PhantomData))
	}
}

impl FromStringOrNumber for GroupNewType
{
	#[inline(always)]
	fn from_str<'a>(value: &'a str) -> Self
	{
		GroupNewType(Group::from(value))
	}
	
	#[inline(always)]
	fn from_u32(value: u32) -> Self
	{
		GroupNewType(Group::from(value))
	}
}
