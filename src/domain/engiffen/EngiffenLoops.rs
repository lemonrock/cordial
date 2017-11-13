// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum EngiffenLoops
{
	infinite,
	finite
	{
		#[serde(default = "EngiffenLoops::repetitions_default")] repetitions: u16,
	}
}

impl Default for EngiffenLoops
{
	#[inline(always)]
	fn default() -> Self
	{
		EngiffenLoops::infinite
	}
}

impl EngiffenLoops
{
	#[inline(always)]
	fn toRepeat(&self) -> ::gif::Repeat
	{
		use self::EngiffenLoops::*;
		use ::gif::Repeat::*;
		
		match *self
		{
			infinite => Infinite,
			finite { repetitions } => Finite(repetitions)
		}
	}
	
	#[inline(always)]
	fn repetitions_default() -> u16
	{
		0
	}
}
