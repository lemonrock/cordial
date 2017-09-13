// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Serialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum HtmlVariant
{
	Canonical,
	AMP,
	PJAX,
}

impl HtmlVariant
{
	#[inline(always)]
	fn pathWithTrailingSlash(self) -> Option<&'static str>
	{
		use self::HtmlVariant::*;
		match self
		{
			Canonical => None,
			AMP => Some("amp/"),
			PJAX => None,
		}
	}
	
	#[inline(always)]
	fn appendToUrl(self, baseUrl: Url) -> Url
	{
		if let Some(pathWithTrailingSlash) = self.pathWithTrailingSlash()
		{
			baseUrl.join(pathWithTrailingSlash).unwrap()
		}
		else
		{
			baseUrl
		}
	}
}
