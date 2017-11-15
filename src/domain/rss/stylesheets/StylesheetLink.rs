// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// Should have MIME types of text/xsl or text/css, eg <?xml-stylesheet type="text/xsl" media="screen" href="/~d/styles/rss2full.xsl"?><?xml-stylesheet type="text/css" media="screen" href="http://feeds.feedburner.com/~d/styles/itemcontent.css"?>
// Only seem to be used by Chrome
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub struct StylesheetLink
{
	url: ResourceReference,
	// #[serde(default = "StylesheetLink::media_default")] media: Option<MediaQuery>,
	#[serde(default = "StylesheetLink::media_default")] media: Option<String>,
	#[serde(default = "StylesheetLink::external_url_mime_type_hint_default")] external_url_mime_type_hint: MimeNewType,
}

impl StylesheetLink
{
	#[inline(always)]
	#[inline(always)]
	pub(crate) fn render<'a, 'b: 'a>(&'a self, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, resources: &'a Resources, newResponses: &'b Responses) -> Result<String, CordialError>
	{
		if let Some((url, response)) = self.url.urlAndResponse(primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code, resources, newResponses)?
		{
			let result = if let Some(response) = response
			{
				self.formatXmlString(&response.contentMimeTypeWithoutParameters(), url)
			}
			else
			{
				self.formatXmlString(&self.external_url_mime_type_hint, url)
			};
			Ok(result)
		}
		else
		{
			Err(CordialError::Configuration(format!("Could not find internal URL for {:?}", &self.url)))
		}
	}
	
	#[inline(always)]
	fn formatXmlString(&self, mimeType: &Mime, url: &Url) -> String
	{
		match self.media
		{
			None => format!("type=\"{}\" href=\"{}\"", mimeType, url),
			Some(ref mediaQuery) => format!("type=\"{}\" media=\"{}\" href=\"{}\"", mimeType, mediaQuery, url),
		}
	}
	
	#[inline(always)]
	fn media_default() -> Option<String>
	{
		Some("all".to_string())
//		Some
//		(
//			MediaQuery
//			{
//				qualifier: None,
//				media_type: MediaQueryType::All,
//				expressions: vec![],
//			}
//		)
	}
	
	#[inline(always)]
	fn external_url_mime_type_hint_default() -> MimeNewType
	{
		MimeNewType("text/css; charset=utf-8".parse().unwrap())
	}
}
