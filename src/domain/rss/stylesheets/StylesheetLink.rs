// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// Should have MIME types of text/xsl or text/css, eg <?xml-stylesheet type="text/xsl" media="screen" href="/~d/styles/rss2full.xsl"?><?xml-stylesheet type="text/css" media="screen" href="http://feeds.feedburner.com/~d/styles/itemcontent.css"?>
// Only seem to be used by Chrome
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub struct StylesheetLink
{
	#[serde(default = "StylesheetLink::url_default")] url: UrlWithTag,
	#[serde(default = "StylesheetLink::media_default")] media: Option<String>,
}

impl StylesheetLink
{
	#[inline(always)]
	#[inline(always)]
	pub(crate) fn render<'a, 'b: 'a>(&'a self, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, resources: &'a Resources, newResponses: &'b Responses) -> Result<String, CordialError>
	{
		if let Some((urlData, contentMimeTypeWithoutParameters)) = resources.urlDataWithContentMimeTypeWithoutParameters(&self.url, primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code, newResponses)?
		{
			Ok(self.formatXmlString(&urlData.urlOrDataUri, &contentMimeTypeWithoutParameters))
		}
		else
		{
			Err(CordialError::Configuration(format!("Could not find a url-response pair for {:?}", &self.url)))
		}
	}
	
	#[inline(always)]
	fn formatXmlString(&self, url: &Url, mimeType: &Mime) -> String
	{
		match self.media
		{
			None => format!("type=\"{}\" href=\"{}\"", mimeType, url),
			Some(ref mediaQuery) => format!("type=\"{}\" media=\"{}\" href=\"{}\"", mimeType, mediaQuery, url),
		}
	}
	
	#[inline(always)]
	fn url_default() -> UrlWithTag
	{
		UrlWithTag::new("/rss.css", UrlTag::default)
	}
	
	#[inline(always)]
	fn media_default() -> Option<String>
	{
		Some("all".to_string())
	}
}
