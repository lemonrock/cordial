// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum FacebookOpenGraphArticleContentType
{
	metered,
	locked,
	free,
}

impl Default for FacebookOpenGraphArticleContentType
{
	#[inline(always)]
	fn default() -> Self
	{
		FacebookOpenGraphArticleContentType::metered
	}
}

impl FacebookOpenGraphArticleContentType
{
	#[inline(always)]
	pub(crate) fn to_str(&self) -> &'static str
	{
		use self::FacebookOpenGraphArticleContentType::*;
		
		match *self
		{
			metered => "metered",
			locked => "locked",
			free => "free",
		}
	}
	
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>)
	{
		endHeadNodes.push(meta_with_property_and_content("article:content_tier", self.to_str()));
	}
}
