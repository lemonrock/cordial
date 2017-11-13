// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.

#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub struct StylesheetLink
{
	url: ResourceReference,
	#[serde(default = "StylesheetLink::media_default")] media: Option<MediaType>,
	#[serde(default = "StylesheetLink::external_url_mime_type_including_charset_if_desired_default")] external_url_mime_type_including_charset_if_desired: MimeNewType,
}

impl StylesheetLink
{
	#[inline(always)]
	#[inline(always)]
	pub(crate) fn render<'a, 'b: 'a>(&'a self, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, resources: &'a BTreeMap<String, Resource>, newResources: &'b Resources) -> Result<(&'a Url, Option<&'a MediaType>, Mime, Option<::hyper::mime::Name<'a>>), CordialError>
	{
		if let Some((url, response)) = self.url.urlAndResponse(primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code, resources, newResources)
		{
			if let Some(response) = response
			{
				Ok((url, self.media.as_ref(), response.contentMimeTypeWithoutParameters(), None))
			}
			else
			{
				let characterSet = self.external_url_mime_type_including_charset_if_desired.characterSet();
				Ok((url, self.media.as_ref(), self.external_url_mime_type_including_charset_if_desired.withoutParameters(), characterSet))
			}
		}
		else
		{
			Err(CordialError::Configuration(format!("Could not find internal URL for {:?}", &self.url)))
		}
	}
	
	#[inline(always)]
	fn media_default() -> Option<MediaType>
	{
		Some(MediaType::default())
	}
	
	#[inline(always)]
	fn external_url_mime_type_including_charset_if_desired_default() -> MimeNewType
	{
		MimeNewType("text/css; charset=utf-8".parse().unwrap())
	}
}
