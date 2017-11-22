// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct UrlData
{
	urlOrDataUri: Rc<Url>,
	urlDataDetails: Rc<UrlDataDetails>,
	dataUriOrRawResponse: Option<Rc<RegularAndPjaxStaticResponse>>,
}

impl UrlData
{
	#[inline(always)]
	fn url(&self) -> &Rc<Url>
	{
		&self.urlOrDataUri
	}
	
	#[inline(always)]
	fn dimensions(&self) -> Result<(u32, u32), CordialError>
	{
		self.urlDataDetails.dimensions()
	}
	
	#[inline(always)]
	fn image(&self) -> Result<(u32, u32, &Mime, u64), CordialError>
	{
		self.urlDataDetails.image()
	}
}
