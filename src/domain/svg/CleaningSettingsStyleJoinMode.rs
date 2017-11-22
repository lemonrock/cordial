// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CleaningSettingsPrecisionStyleJoinMode
{
	none,
	all,
	some,
}

impl Default for CleaningSettingsPrecisionStyleJoinMode
{
	#[inline(always)]
	fn default() -> Self
	{
		CleaningSettingsPrecisionStyleJoinMode::some
	}
}

impl CleaningSettingsPrecisionStyleJoinMode
{
	#[inline(always)]
	fn toStyleJoinMode(&self) -> StyleJoinMode
	{
		use self::CleaningSettingsPrecisionStyleJoinMode::*;
		use self::StyleJoinMode::*;
		
		match *self
		{
			none => None,
			all => All,
			some => Some,
		}
	}
}
