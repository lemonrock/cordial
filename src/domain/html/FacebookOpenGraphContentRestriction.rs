// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum FacebookOpenGraphContentRestriction
{
	alcohol,
}

impl Default for FacebookOpenGraphContentRestriction
{
	#[inline(always)]
	fn default() -> Self
	{
		FacebookOpenGraphContentRestriction::alcohol
	}
}

impl FacebookOpenGraphContentRestriction
{
	#[inline(always)]
	fn to_str(&self) -> &'static str
	{
		use self::FacebookOpenGraphContentRestriction::*;
		
		match *self
		{
			alcohol => "alcohol",
		}
	}
	
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>)
	{
		endHeadNodes.push(meta_with_property_and_content("og:restrictions:content", self.to_str()))
	}
}
