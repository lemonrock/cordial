// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum AudioVideoDisabledControl
{
	nodownload,
	nofullscreen,
	noremoteplayback,
}

impl Deref for AudioVideoDisabledControl
{
	type Target = str;
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		use self::AudioVideoDisabledControl::*;
		
		match *self
		{
			nodownload => "nodownload",
			nofullscreen => "nofullscreen",
			noremoteplayback => "noremoteplayback",
		}
	}
}
