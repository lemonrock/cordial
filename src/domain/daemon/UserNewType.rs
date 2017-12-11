// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
struct UserNewType(User);

impl Deref for UserNewType
{
	type Target = User;
	
	fn deref(&self) -> &User
	{
		&self.0
	}
}

impl DerefMut for UserNewType
{
	fn deref_mut(&mut self) -> &mut User
	{
		&mut self.0
	}
}

impl<'de> Deserialize<'de> for UserNewType
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		deserializer.deserialize_any(StringOrNumberVisitor(PhantomData))
	}
}

impl Serialize for UserNewType
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		match self.0
		{
			User::Name(ref string) => serializer.serialize_str(string.as_str()),
			User::Id(id) => serializer.serialize_u32(id),
		}
	}
}

impl FromStringOrNumber for UserNewType
{
	#[inline(always)]
	fn from_str<'a>(value: &'a str) -> Self
	{
		UserNewType(User::from(value))
	}
	
	#[inline(always)]
	fn from_u32(value: u32) -> Self
	{
		UserNewType(User::from(value))
	}
}
