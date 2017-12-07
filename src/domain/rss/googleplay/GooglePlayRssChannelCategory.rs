// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// From https://support.google.com/googleplay/podcasts/answer/6260341?hl=en&ref_topic=6249881
// As of 6th December 2017, category lists are identical to iTunes top-level category.
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum GooglePlayCategory
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

impl Default for GooglePlayCategory
{
	#[inline(always)]
	fn default() -> Self
	{
		GooglePlayCategory::Technology
	}
}

impl GooglePlayCategory
{
	#[inline(always)]
	pub(crate) fn toCategoryStr(&self) -> &'static str
	{
		use self::GooglePlayCategory::*;
		
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
