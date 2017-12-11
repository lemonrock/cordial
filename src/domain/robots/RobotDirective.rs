// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum RobotDirective
{
	Allow(String),
	Disallow(String),
}

impl RobotDirective
{
	#[inline(always)]
	pub(crate) fn withBaseUrl(&self, relativeUrlPathForRobotDirective: &str) -> String
	{
		use self::RobotDirective::*;
		
		match *self
		{
			Allow(ref asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded) => Self::formatUrl("Allow", asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded, relativeUrlPathForRobotDirective),
			Disallow(ref asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded) => Self::formatUrl("Disallow", asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded, relativeUrlPathForRobotDirective),
		}
	}
	
	#[inline(always)]
	fn formatUrl(name: &str, asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded: &String, relativeUrlPathForRobotDirective: &str) -> String
	{
		if asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded.as_str() == "*"
		{
			"*".to_owned()
		}
		else
		{
			if relativeUrlPathForRobotDirective.ends_with('/')
			{
				format!("{}: {}{}", name, relativeUrlPathForRobotDirective, asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded)
			}
			else
			{
				format!("{}: {}/{}", name, relativeUrlPathForRobotDirective, asteriskOrAbsolutePathWithWildcardsMightBePercentEncoded)
			}
		}
	}
}
