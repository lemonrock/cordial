// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum AudioCodec
{
	mp4a_40_2,
	vorbis,
}

impl AudioCodec
{
	#[inline(always)]
	pub(crate) fn to_str(&self) -> &'static str
	{
		match *self
		{
			mp4a_40_2 => "mp4a.40.2",
			vorbis => "vorbis",
		}
	}
}
