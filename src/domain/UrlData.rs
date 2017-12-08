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
	pub(crate) fn durationInSeconds(&self) -> Result<u64, CordialError>
	{
		self.urlDataDetails.durationInSeconds()
	}
	
	#[inline(always)]
	pub(crate) fn optionalVideoWidthHeight(&self) -> Option<(u16, u16)>
	{
		self.urlDataDetails.optionalVideoWidthHeight()
	}
	
	#[inline(always)]
	pub(crate) fn size(&self) -> u64
	{
		self.urlDataDetails.size()
	}
	
	#[inline(always)]
	pub(crate) fn isSuitableForFacebookOpenGraphImage(&self) -> bool
	{
		match (self.mimeType.type_(), self.mimeType.subtype())
		{
			(IMAGE, GIF) => true,
			
			(IMAGE, JPEG) => true,
			
			(IMAGE, PNG) => true,
			
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn isSuitableForTwitterCardsImage(&self) -> bool
	{
		match (self.mimeType.type_(), self.mimeType.subtype().as_str())
		{
			(IMAGE, "gif") => true,
			
			(IMAGE, "jpeg") => true,
			
			(IMAGE, "png") => true,
			
			(IMAGE, "webp") => true,
			
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn isSuitableForGoogleVideoSiteMapThumbnailImage(&self) -> bool
	{
		match (self.mimeType.type_(), self.mimeType.subtype().as_str())
		{
			(IMAGE, "jpeg") => true,
			
			(IMAGE, "png") => true,
			
			(IMAGE, "webp") => true,
			
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn isSuitableForGooglePlayArtwork(&self) -> bool
	{
		match (self.mimeType.type_(), self.mimeType.subtype())
		{
			(IMAGE, JPEG) => true,
			
			(IMAGE, PNG) => true,
			
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn isSuitableForITunesArtwork(&self) -> bool
	{
		match (self.mimeType.type_(), self.mimeType.subtype())
		{
			(IMAGE, JPEG) => true,
			
			(IMAGE, PNG) => true,
			
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsPng(&self) -> Result<(), CordialError>
	{
		match (self.mimeType.type_(), self.mimeType.subtype())
		{
			(IMAGE, PNG) => Ok(()),
			
			_ => Err(CordialError::Configuration("Resource should be a PNG".to_owned())),
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsSuitableForRssImage(&self) -> Result<(), CordialError>
	{
		if self.isSuitableForITunesArtwork()
		{
			Ok(())
		}
		else
		{
			Err(CordialError::Configuration("Resource should be a JPEG or PNG".to_owned()))
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsSuitableForWebAppManifestIcon(&self) -> Result<(), CordialError>
	{
		match (self.mimeType.type_(), self.mimeType.subtype().as_str(), self.mimeType.suffix())
		{
			(IMAGE, "png", None) => Ok(()),
			
			(IMAGE, "webp", None) => Ok(()),
			
			(IMAGE, "svg", Some(XML)) => Ok(()),
			
			_ => Err(CordialError::Configuration("Not suitable for a web app manifest icon".to_owned())),
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsSuitableForWebAppManifestScreenshot(&self) -> Result<(), CordialError>
	{
		match (self.mimeType.type_(), self.mimeType.subtype().as_str(), self.mimeType.suffix())
		{
			(IMAGE, "png", None) => Ok(()),
			
			(IMAGE, "webp", None) => Ok(()),
			
			(IMAGE, "svg", Some(XML)) => Ok(()),
			
			_ => Err(CordialError::Configuration("Not suitable for a web app manifest screenshot".to_owned())),
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsSvg(&self) -> Result<(), CordialError>
	{
		match (self.mimeType.type_(), self.mimeType.subtype().as_str(), self.mimeType.suffix())
		{
			(IMAGE, "svg", Some(XML)) => Ok(()),
			
			_ => Err(CordialError::Configuration("Resource should be a SVG".to_owned())),
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsHtml(&self) -> Result<(), CordialError>
	{
		match (self.mimeType.type_(), self.mimeType.subtype())
		{
			(TEXT, HTML) => Ok(()),
			
			_ => Err(CordialError::Configuration("Resource should be HTML".to_owned())),
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsXml(&self) -> Result<(), CordialError>
	{
		match (self.mimeType.type_(), self.mimeType.subtype())
		{
			(TEXT, XML) => Ok(()),
			
			_ => Err(CordialError::Configuration("Resource should be XML".to_owned())),
		}
	}
	
	#[inline(always)]
	pub(crate) fn validateIsExcludingParameters(&self, mimeType: &Mime) -> Result<(), CordialError>
	{
		let ours = &self.mimeType;
		if ours.type_() == mimeType.type_() && ours.subtype() == mimeType.subtype() && ours.suffix() == mimeType.suffix()
		{
			Ok(())
		}
		else
		{
			Err(CordialError::Configuration(format!("Resource should have mime type '{:?}'", mimeType)))
		}
	}
}
