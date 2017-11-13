// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum EngiffenDisposal
{
	no_action,
	keep,
	restore_to_background_color,
	restore_to_previous_frame,
}

impl Default for EngiffenDisposal
{
	#[inline(always)]
	fn default() -> Self
	{
		EngiffenDisposal::keep
	}
}

impl EngiffenDisposal
{
	#[inline(always)]
	fn toDisposalMethod(&self) -> ::gif::DisposalMethod
	{
		use self::EngiffenDisposal::*;
		use ::gif::DisposalMethod::*;
		
		match *self
		{
			no_action => Any,
			keep => Keep,
			restore_to_background_color => Background,
			restore_to_previous_frame => Previous,
		}
	}
}
