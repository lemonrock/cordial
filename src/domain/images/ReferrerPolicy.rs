// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum ReferrerPolicy
{
	no_referrer,
	no_referrer_when_downgrade,
	origin,
	origin_when_cross_origin,
	unsafe_url,
}

impl Default for ReferrerPolicy
{
	#[inline(always)]
	fn default() -> Self
	{
		ReferrerPolicy::no_referrer_when_downgrade
	}
}

impl ReferrerPolicy
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn addToImgAttributes(&self, imgAttributes: &mut Vec<Attribute>)
	{
		#[inline(always)]
		fn add(imgAttributes: &mut Vec<Attribute>, value: &str)
		{
			imgAttributes.push("referrerpolicy".str_attribute(value))
		}
		
		use self::ReferrerPolicy::*;
		
		match *self
		{
			no_referrer => add(imgAttributes, "no-referrer"),
			origin => add(imgAttributes, "origin"),
			origin_when_cross_origin => add(imgAttributes, "origin-when-cross-origin"),
			unsafe_url => add(imgAttributes, "unsafe-url"),
			_ => (),
		}
	}
}
