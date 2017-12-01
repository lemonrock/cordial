// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum BrowserConfigPollNotificationCycle
{
	#[serde(rename = "0")] _0,
	#[serde(rename = "1")] _1,
	#[serde(rename = "2")] _2,
	#[serde(rename = "3")] _3,
	#[serde(rename = "4")] _4,
	#[serde(rename = "5")] _5,
	#[serde(rename = "6")] _6,
	#[serde(rename = "7")] _7,
}

impl Default for BrowserConfigPollNotificationCycle
{
	#[inline(always)]
	fn default() -> Self
	{
		BrowserConfigPollNotificationCycle::_0
	}
}

impl BrowserConfigPollNotificationCycle
{
	#[inline(always)]
	fn to_str(&self) -> &'static str
	{
		use self::BrowserConfigPollNotificationCycle::*;
		
		match *self
		{
			_0 => "0",
			_1 => "1",
			_2 => "2",
			_3 => "3",
			_4 => "4",
			_5 => "5",
			_6 => "6",
			_7 => "7",
		}
	}
}
