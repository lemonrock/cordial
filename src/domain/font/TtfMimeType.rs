// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum TtfMimeType
{
	application_font_sfnt,
	font_sfnt,
	font_ttf,
}

impl Default for TtfMimeType
{
	#[inline(always)]
	fn default() -> Self
	{
		TtfMimeType::application_font_sfnt
	}
}

impl TtfMimeType
{
	#[inline(always)]
	pub(crate) fn contentType(&self) -> ContentType
	{
		use self::TtfMimeType::*;
		
		match *self
		{
			application_font_sfnt => content_type_application_font_sfnt(),
			font_sfnt => content_type_font_sfnt(),
			font_ttf => content_type_font_ttf(),
		}
	}
}
