// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// From https://help.apple.com/itc/podcasts_connect/?lang=en#/itc9267a2f12
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum ITunesRssChannelTopLevelCategory
{
	Arts,
	
	Business,
	
	Comedy,
	
	#[serde(rename = "Games & Hobbies")] Games_and_Hobbies,
	
	#[serde(rename = "Government & Organizations")] Government_and_Organizations,
	
	Education,
	
	Health,
	
	#[serde(rename = "Kids & Family")] Kids_and_Family,
	
	Music,
	
	#[serde(rename = "News & Politics")] News_and_Politics,
	
	#[serde(rename = "Religion & Spirituality")] Religion_and_Spirituality,
	
	#[serde(rename = "Science & Medicine")] Science_and_Medicine,
	
	#[serde(rename = "Society & Culture")] Society_and_Culture,
	
	#[serde(rename = "Sports & Recreation")] Sports_and_Recreation,
	
	Technology,
	
	#[serde(rename = "TV & Film")] TV_and_Film,
}

impl Default for ITunesRssChannelTopLevelCategory
{
	#[inline(always)]
	fn default() -> Self
	{
		ITunesRssChannelTopLevelCategory::Technology
	}
}

impl ITunesRssChannelTopLevelCategory
{
	#[inline(always)]
	pub(crate) fn toCategoryStr(&self) -> &'static str
	{
		use self::ITunesRssChannelTopLevelCategory::*;
		
		match *self
		{
			Arts => "Arts",
			Business => "Business",
			Comedy => "Comedy",
			Education => "Education",
			Games_and_Hobbies => "Games & Hobbies",
			Government_and_Organizations => "Government & Organizations",
			Health => "Health",
			Kids_and_Family => "Kids & Family",
			Music => "Music",
			News_and_Politics => "News & Politics",
			Religion_and_Spirituality => "Religion & Spirituality",
			Science_and_Medicine => "Science & Medicine",
			Society_and_Culture => "Society & Culture",
			Sports_and_Recreation => "Sports & Recreation",
			Technology => "Technology",
			TV_and_Film => "TV & Film",
		}
	}
}
