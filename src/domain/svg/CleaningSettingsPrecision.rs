// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CleaningSettingsPrecision
{
	one,
	two,
	three,
	four,
	five,
	six,
	seven,
	eight,
	nine,
	ten,
	eleven,
	twelve,
}

impl Default for CleaningSettingsPrecision
{
	#[inline(always)]
	fn default() -> Self
	{
		CleaningSettingsPrecision::one
	}
}

impl CleaningSettingsPrecision
{
	#[inline(always)]
	fn to_u8(&self) -> u8
	{
		use self::CleaningSettingsPrecision::*;
		
		match *self
		{
			one => 1,
			two => 2,
			three => 3,
			four => 4,
			five => 5,
			six => 6,
			seven => 7,
			eight => 8,
			nine => 9,
			ten => 10,
			eleven => 11,
			twelve => 12,
		}
	}
}
