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
}
