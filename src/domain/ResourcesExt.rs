// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait ResourcesExt
{
	#[inline(always)]
	fn urlData<'resources>(&'resources self, resourceReference: &ResourceReference, primaryIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<Option<Rc<UrlData>>, CordialError>;
	
	#[inline(always)]
	fn urlDataWithContentMimeTypeWithoutParameters<'resources>(&'resources self, resourceReference: &ResourceReference, primaryIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, newResponses: &Responses) -> Result<Option<(Rc<UrlData>, Mime)>, CordialError>;
}

impl ResourcesExt for Resources
{
	#[inline(always)]
	fn urlData<'resources>(&'resources self, resourceReference: &ResourceReference, primaryIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<Option<Rc<UrlData>>, CordialError>
	{
		match resourceReference.get(self)
		{
			None => Ok(None),
			Some(resourceRefCell) =>
			{
				let refResource = resourceRefCell.try_borrow()?;
				
				Ok(refResource.urlData(primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &resourceReference.tag).cloned())
			},
		}
	}
	
	#[inline(always)]
	fn urlDataWithContentMimeTypeWithoutParameters<'resources>(&'resources self, resourceReference: &ResourceReference, primaryIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, newResponses: &Responses) -> Result<Option<(Rc<UrlData>, Mime)>, CordialError>
	{
		let urlData = match self.urlData(resourceReference, primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?
		{
			None => return Ok(None),
			Some(urlData) => urlData,
		};
		
		let contentMimeTypeWithoutParameters = if let Some(ref response) = urlData.dataUriOrRawResponse
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
