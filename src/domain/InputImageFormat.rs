// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum InputImageFormat
{
	PNG,
	JPEG,
	GIF,
	BMP,
	ICO,
	TIFF,
	WebP,
	PPM,
	HDR,
	TGA
}

impl InputImageFormat
{
	#[inline(always)]
	pub(crate) fn fileExtensions(&self) -> Vec<&'static str>
	{
		use self::InputImageFormat::*;
		match *self
		{
			PNG => vec![".png"],
			JPEG => vec![".jpeg", ".jpg"],
			GIF => vec![".gif"],
			BMP => vec![".bmp"],
			ICO => vec![".ico"],
			TIFF => vec![".tiff", ".tif"],
			WebP => vec![".webp"],
			PPM => vec![".ppm"],
			HDR => vec![".hdr"],
			TGA => vec![".tga", ".icb", ".vda", ".vst"],
		}
	}
}
