// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct UrlData
{
	url: Rc<Url>,
	mimeType: Mime,
	urlDataDetails: Rc<UrlDataDetails>,
}

impl UrlData
{
	#[inline(always)]
	pub(crate) fn url(&self) -> &Rc<Url>
	{
		&self.url
	}
	
	#[inline(always)]
	pub(crate) fn url_str(&self) -> &str
	{
		self.url().as_ref().as_str()
	}
	
	#[inline(always)]
	pub(crate) fn mimeType(&self) -> &Mime
	{
		&self.mimeType
	}
	
	#[inline(always)]
	pub(crate) fn mimeTypeWithoutParameters(&self) -> Mime
	{
		self.mimeType().withoutParameters()
	}
	
	#[inline(always)]
	pub(crate) fn dimensions(&self) -> Result<(u32, u32), CordialError>
	{
		self.urlDataDetails.dimensions()
	}
	
	#[inline(always)]
	pub(crate) fn image(&self) -> Result<(u32, u32, u64), CordialError>
	{
		self.urlDataDetails.image()
	}
	
	#[inline(always)]
	pub(crate) fn size(&self) -> u64
	{
		self.urlDataDetails.size()
	}
	
	#[inline(always)]
	pub(crate) fn isSuitableForFacebookOpenGraphImage(&self) -> bool
	{
		match self.mimeType.type_()
		{
			mime::IMAGE => match self.mimeType.subtype()
			{
				mime::GIF | mime::JPEG | mime::PNG => true,
				_ => false,
			}
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn isSuitableForTwitterCardsImage(&self) -> bool
	{
		match self.mimeType.type_()
		{
			mime::IMAGE => match self.mimeType.subtype()
			{
				mime::GIF => true,
				
				mime::JPEG => true,
				
				mime::PNG => true,
				
				_ => self.mimeType == "image/webp".parse::<Mime>().unwrap(),
			}
			
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsPng(&self) -> Result<(), CordialError>
	{
		match (self.mimeType.type_(), self.mimeType.subtype())
		{
			(mime::IMAGE, mime::PNG) => Ok(()),
			
			_ => Err(CordialError::Configuration("Resource should be a PNG".to_owned())),
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsSuitableForWebAppManifestIcon(&self) -> Result<(), CordialError>
	{
		match self.mimeType.as_ref()
		{
			"image/png" | "image/webp" | "image/svg+xml" => Ok(()),
			_ => Err(CordialError::Configuration("Resource should be a PNG, WebP or SVG".to_owned()))
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsSuitableForWebAppManifestScreenshot(&self) -> Result<(), CordialError>
	{
		self.validateIsSuitableForWebAppManifestIcon()
	}
	
	#[inline(always)]
	pub(crate) fn validateIsSvg(&self) -> Result<(), CordialError>
	{
		if self.mimeType == mimeType("image/svg+xml")
		{
			Ok(())
		}
		else
		{
			Err(CordialError::Configuration("Resource should be a SVG".to_owned()))
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsHtml(&self) -> Result<(), CordialError>
	{
		match (self.mimeType.type_(), self.mimeType.subtype())
		{
			(mime::TEXT, mime::HTML) => Ok(()),
			
			_ => Err(CordialError::Configuration("Resource should be HTML".to_owned())),
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateHasMimeType(&self, hasMimeType: &Mime) -> Result<(), CordialError>
	{
		if &self.mimeType == hasMimeType
		{
			Ok(())
		}
		else
		{
			Err(CordialError::Configuration(format!("Resource should have mime type '{:?}'", hasMimeType)))
		}
	}
}
