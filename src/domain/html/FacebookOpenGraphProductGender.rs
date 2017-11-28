// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum FacebookOpenGraphProductGender
{
	male,
	female,
	unisex,
}

impl Default for FacebookOpenGraphProductGender
{
	// Defaults to male simply because this is the more common gender in most domains.
	#[inline(always)]
	fn default() -> Self
	{
		FacebookOpenGraphProductGender::male
	}
}

impl FacebookOpenGraphProductGender
{
	#[inline(always)]
	pub(crate) fn to_str(&self) -> &'static str
	{
		use self::FacebookOpenGraphProductGender::*;
		
		match *self
		{
			male => "male",
			female => "female",
			unisex => "unisex",
		}
	}
}
