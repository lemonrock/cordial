// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct NonZeroNumber(pub(crate) u16);

impl Default for NonZeroNumber
{
	#[inline(always)]
	fn default() -> Self
	{
		NonZeroNumber(1)
	}
}

impl<'de> Deserialize<'de> for NonZeroNumber
{
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		let value = u16::deserialize(deserializer)?;
		if value == 0
		{
			return Err(D::Error::custom("value is zero"))
		}
		Ok(NonZeroNumber(value))
	}
}
