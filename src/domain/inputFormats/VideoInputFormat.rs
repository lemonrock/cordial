// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum VideoInputFormat
{
	MP4_with_WebM,
}

impl Default for VideoInputFormat
{
	#[inline(always)]
	fn default() -> Self
	{
		VideoInputFormat::MP4_with_WebM
	}
}

impl InputFormat for VideoInputFormat
{
	#[inline(always)]
	fn fileExtensions(&self) -> &'static [&'static str]
	{
		use self::VideoInputFormat::*;
		
		match *self
		{
			MP4_with_WebM => &[".mp4"],
		}
	}
	
	#[inline(always)]
	fn allFileExtensions() -> &'static [&'static str]
	{
		&[
			".mp4",
		]
	}
}

impl VideoInputFormat
{
}
