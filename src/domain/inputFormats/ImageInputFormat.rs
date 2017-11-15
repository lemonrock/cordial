// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum ImageInputFormat
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
	Targa
}

impl Default for ImageInputFormat
{
	#[inline(always)]
	fn default() -> Self
	{
		ImageInputFormat::PNG
	}
}

impl InputFormat for ImageInputFormat
{
	#[inline(always)]
	fn fileExtensions(&self) -> &'static [&'static str]
	{
		use self::ImageInputFormat::*;
		
		match *self
		{
			PNG => &[".png"],
			JPEG => &[".jpeg", ".jpg", ".jpe"],
			GIF => &[".gif"],
			BMP => &[".bmp"],
			ICO => &[".ico", ".cur"],
			TIFF => &[".tiff", ".tif"],
			WebP => &[".webp"],
			PPM => &[".ppm"],
			HDR => &[".hdr"],
			Targa => &[".tga", ".icb", ".vda", ".vst"],
		}
	}
	
	#[inline(always)]
	fn allFileExtensions() -> &'static [&'static str]
	{
		&[
			".png",
			".jpeg", ".jpg", ".jpe",
			".gif",
			".bmp",
			".ico", ".cur",
			".tiff", ".tif",
			".webp",
			".ppm",
			".hdr",
			".tga", ".icb", ".vda", ".vst",
		]
	}
}

impl ImageInputFormat
{
	#[inline(always)]
	pub(crate) fn load(option: Option<Self>, path: &Path) -> Option<Result<::image::DynamicImage, CordialError>>
	{
		if let Some(imageInputFormat) = option
		{
			Some(path.fileContentsAsImage(imageInputFormat))
		}
		else if let Some(osStrExtension) = path.extension()
		{
			use self::ImageInputFormat::*;
			
			if let Some(utf8FileExtension) = osStrExtension.to_str()
			{
				let imageInputFormat = match utf8FileExtension
				{
					"png" => PNG,
					"jpe" => JPEG,
					"jpeg" => JPEG,
					"jpg" => JPEG,
					"gif" => GIF,
					"bmp" => BMP,
					"ico" => ICO,
					"cur" => ICO,
					"tiff" => TIFF,
					"tif" => TIFF,
					"webp" => WebP,
					"ppm" => PPM,
					"hdr" => HDR,
					"tga" => Targa,
					"icb" => Targa,
					"vda" => Targa,
					"vst" => Targa,
					_ => return None,
				};
				Some(path.fileContentsAsImage(imageInputFormat))
			}
			else
			{
				None
			}
		}
		else
		{
			None
		}
	}
}
