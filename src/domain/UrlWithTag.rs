// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct UrlWithTag
{
	pub(crate) resource: String,
	pub(crate) tag: UrlTag,
}

impl UrlWithTag
{
	#[inline(always)]
	pub(crate) fn new<S: Into<String>>(resource: S, tag: UrlTag) -> Self
	{
		Self
		{
			resource: resource.into(),
			tag,
		}
	}
}
