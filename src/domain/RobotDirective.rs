// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Deserialize)]
pub(crate) enum RobotDirective
{
	Allow(String),
	Disallow(String),
}

impl RobotDirective
{
	#[inline(always)]
	pub(crate) fn withBaseUrl(&self, relative_root_url: &str) -> String
	{
		match *self
		{
			Allow(ref asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded) => Self::formatUrl("Allow", asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded),
			Disallow(ref asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded) => Self::formatUrl("Disallow", asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded),
		}
	}
	
	#[inline(always)]
	fn formatUrl(name: &str, asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded: &String, relative_root_url: &str) -> String
	{
		if asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded.to_str() == "*"
		{
			"*".to_owned()
		}
		else
		{
			if relative_root_url.ends_with('/')
			{
				format!("{}: {}{}", name, relative_root_url, asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded)
			}
			else
			{
				format!("{}: {}/{}", name, relative_root_url, asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded)
			}
		}
	}
}
