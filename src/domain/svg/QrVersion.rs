// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum QrVersion
{
	normal(i16),
	micro(i16),
}

impl Default for QrVersion
{
	#[inline(always)]
	fn default() -> Self
	{
		QrVersion::normal(5)
	}
}

impl QrVersion
{
	#[inline(always)]
	pub(crate) fn toVersion(&self) -> Version
	{
		use self::QrVersion::*;
		use self::Version::*;
		
		match *self
		{
			normal(value) => Normal(value),
			micro(value) => Micro(value),
		}
	}
}
