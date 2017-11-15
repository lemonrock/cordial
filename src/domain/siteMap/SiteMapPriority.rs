// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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

impl SiteMapPriority
{
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
