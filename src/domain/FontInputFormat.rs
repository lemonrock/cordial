// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum FontInputFormat
{
	TTF,
	// OTF, WOFF, WOFF2, SVG (font), SVG (icons), EOT
}

impl Default for FontInputFormat
{
	#[inline(always)]
	fn default() -> Self
	{
		FontInputFormat::TTF
	}
}

impl FontInputFormat
{
	#[inline(always)]
	pub(crate) fn fileExtensions(&self) -> Vec<&'static str>
	{
		use self::FontInputFormat::*;
		match *self
		{
			TTF => vec![".ttf"],
		}
	}
}
