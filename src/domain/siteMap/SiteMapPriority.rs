// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum SiteMapPriority
{
	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
}

impl Default for SiteMapPriority
{
	#[inline(always)]
	fn default() -> Self
	{
		SiteMapPriority::Five
	}
}

impl SiteMapPriority
{
	#[inline(always)]
	pub(crate) fn as_str(&self) -> &'static str
	{
		use self::SiteMapPriority::*;
		match *self
		{
			Zero => "0.0",
			One => "0.1",
			Two => "0.2",
			Three => "0.3",
			Four => "0.4",
			Five => "0.5",
			Six => "0.6",
			Seven => "0.7",
			Eight => "0.8",
			Nine => "0.9",
			Ten => "1.0",
		}
	}
}
