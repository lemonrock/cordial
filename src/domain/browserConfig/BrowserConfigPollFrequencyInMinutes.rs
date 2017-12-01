// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum BrowserConfigPollFrequencyInMinutes
{
	#[serde(rename = "30")] _30,
	#[serde(rename = "60")] _60,
	#[serde(rename = "360")] _360,
	#[serde(rename = "720")] _720,
	#[serde(rename = "1440")] _1440,
}

impl Default for BrowserConfigPollFrequencyInMinutes
{
	#[inline(always)]
	fn default() -> Self
	{
		BrowserConfigPollFrequencyInMinutes::_30
	}
}

impl BrowserConfigPollFrequencyInMinutes
{
	#[inline(always)]
	fn to_str(&self) -> &'static str
	{
		use self::BrowserConfigPollFrequencyInMinutes::*;
		
		match *self
		{
			_30 => "30",
			_60 => "60",
			_360 => "360",
			_720 => "720",
			_1440 => "1440",
		}
	}
}
