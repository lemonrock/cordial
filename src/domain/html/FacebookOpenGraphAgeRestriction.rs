// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum FacebookOpenGraphAgeRestriction
{
	#[serde(rename = "13+")] ThirteenPlus,
	#[serde(rename = "16+")] SixteenPlus,
	#[serde(rename = "17+")] SeventeenPlus,
	#[serde(rename = "18+")] EighteenPlus,
	#[serde(rename = "19+")] NineteenPlus,
	#[serde(rename = "21+")] TwentyOnePlus,
}

impl Default for FacebookOpenGraphAgeRestriction
{
	#[inline(always)]
	fn default() -> Self
	{
		FacebookOpenGraphAgeRestriction::TwentyOnePlus
	}
}

impl FacebookOpenGraphAgeRestriction
{
	#[inline(always)]
	fn to_str(&self) -> &'static str
	{
		use self::FacebookOpenGraphAgeRestriction::*;
		
		match *self
		{
			ThirteenPlus => "13+",
			SixteenPlus => "16+",
			SeventeenPlus => "17+",
			EighteenPlus => "18+",
			NineteenPlus => "19+",
			TwentyOnePlus => "21+",
		}
	}
	
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>)
	{
		endHeadNodes.push(meta_with_property_and_content("og:restrictions:age", self.to_str()))
	}
}
