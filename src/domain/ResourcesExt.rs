// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait ResourcesExt
{
	#[inline(always)]
	fn urlData(&self, urlWithTag: &UrlWithTag, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>) -> Result<Option<Rc<UrlData>>, CordialError>;
	
	#[inline(always)]
	fn urlDataWithContentMimeTypeWithoutParameters(&self, urlWithTag: &UrlWithTag, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, newResponses: &Responses) -> Result<Option<(Rc<UrlData>, Mime)>, CordialError>;
}

impl ResourcesExt for Resources
{
	#[inline(always)]
	fn urlData(&self, urlWithTag: &UrlWithTag, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>) -> Result<Option<Rc<UrlData>>, CordialError>
	{
		match self.get(&urlWithTag.resource)
		{
			None => Ok(None),
			Some(resourceRefCell) =>
			{
				let refResource = resourceRefCell.try_borrow()?;
				
				Ok(refResource.urlData(primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code, &urlWithTag.tag))
			},
		}
	}
	
	#[inline(always)]
	fn urlDataWithContentMimeTypeWithoutParameters(&self, urlWithTag: &UrlWithTag, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, newResponses: &Responses) -> Result<Option<(Rc<UrlData>, Mime)>, CordialError>
	{
		let urlData = match self.urlData(urlWithTag, primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code)?
		{
			None => return Ok(None),
			Some(urlData) => urlData,
		};
		
		let contentMimeTypeWithoutParameters = if let Some(ref response) = urlData.dataUriResponse
		{
			response.contentMimeTypeWithoutParameters()
		}
		else
		{
			match newResponses.getLatestResponse(&urlData.urlOrDataUri)
			{
				None => return Ok(None),
				Some(response) => response.contentMimeTypeWithoutParameters(),
			}
		};
		
		Ok(Some((urlData, contentMimeTypeWithoutParameters)))
	}
}
