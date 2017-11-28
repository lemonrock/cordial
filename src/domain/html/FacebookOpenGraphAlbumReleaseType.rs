// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum FacebookOpenGraphAlbumReleaseType
{
	original_release,
	re_release,
	anthology,
}

impl Default for FacebookOpenGraphAlbumReleaseType
{
	#[inline(always)]
	fn default() -> Self
	{
		FacebookOpenGraphAlbumReleaseType::original_release
	}
}

impl FacebookOpenGraphAlbumReleaseType
{
	#[inline(always)]
	pub(crate) fn to_str(&self) -> &'static str
	{
		use self::FacebookOpenGraphAlbumReleaseType::*;
		
		match *self
		{
			original_release => "original_release",
			re_release => "re_release",
			anthology => "anthology",
		}
	}
	
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>)
	{
		endHeadNodes.push(meta_with_property_and_content("music:release_type", self.to_str()));
	}
}
