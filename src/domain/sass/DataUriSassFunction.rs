// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct DataUriSassFunction
{
	resources: Rc<Resources>,
	fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language,
	iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>,
	newResponses: Rc<Responses>,
}

impl SassFunction for DataUriSassFunction
{
	#[inline(always)]
	fn signature(&self) -> SassFunctionSignature
	{
		SassFunctionSignature::from_static_str("embed-data-uri($resourceUrl, $resourceTag)")
	}
	
	fn callback(&mut self, arguments: ListSassValue, _compiler: SassCompiler) -> Result<SassValue, SassValueError>
	{
		if arguments.length() != 2
		{
			return SassValueError::function_failed_from_static_str("Must have two arguments, $resourceUrl and $resourceTag");
		}
		
		let zeroth = unsafe { arguments.get_value_unchecked(0) };
		let resourceUrl = zeroth.as_string()?.value();
		let resource = ResourceUrl::string(resourceUrl.to_str()?);
		
		let first = unsafe { arguments.get_value_unchecked(1) };
		let resourceTag = first.as_string()?.value();
		let tag: ResourceTag = match ::serde_json::from_str(resourceTag.to_str()?)
		{
			Err(error) => return SassValueError::function_failed_from_string(format!("Could not deserialize $resourceTag JSON because '{}'", error)),
			Ok(resourceTag) => resourceTag,
		};
		
		let urlData = match (ResourceReference { resource, tag }).urlDataMandatory(&self.resources, self.fallbackIso639Dash1Alpha2Language, self.iso639Dash1Alpha2Language)
		{
			Err(error) => return SassValueError::function_failed_from_string(format!("Could not find resource UrlData because '{}'", error)),
			Ok(urlData) => urlData,
		};
		
		let dataUri = match self.newResponses.find(urlData.url().as_ref())
		{
			None => return SassValueError::function_failed_from_static_str("Response not yet created (processing priority is set incorrectly)"),
			Some(staticResponseVersions) => match staticResponseVersions.currentStaticResponse()
			{
				None => return SassValueError::function_failed_from_static_str("Response is discontinued"),
				Some(regularAndPjaxStaticResponse) => regularAndPjaxStaticResponse.toDataUri(),
			}
		};
		
		Ok(SassValue::new_unquoted_string(&CString::new(dataUri.into_bytes())?))
	}
}
