// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
struct GroupNewType(Group);

impl Deref for GroupNewType
{
	type Target = Group;
	
	fn deref(&self) -> &Group
	{
		&self.0
	}
}

impl DerefMut for GroupNewType
{
	fn deref_mut(&mut self) -> &mut Group
	{
		&mut self.0
	}
}

impl<'de> Deserialize<'de> for GroupNewType
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		deserializer.deserialize_any(StringOrNumberVisitor(PhantomData))
	}
}

impl Serialize for GroupNewType
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		match self.0
		{
			Group::Name(ref string) => serializer.serialize_str(string.as_str()),
			Group::Id(id) => serializer.serialize_u32(id),
		}
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
