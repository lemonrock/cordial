// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// Should have MIME types of text/xsl or text/css, eg <?xml-stylesheet type="text/xsl" media="screen" href="/~d/styles/rss2full.xsl"?><?xml-stylesheet type="text/css" media="screen" href="http://feeds.feedburner.com/~d/styles/itemcontent.css"?>
// Only seem to be used by Chrome
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct StylesheetLink
{
	#[serde(default = "StylesheetLink::url_default")] url: ResourceReference,
	#[serde(default = "StylesheetLink::media_default")] media: Option<String>,
}

impl StylesheetLink
{
	#[inline(always)]
	#[inline(always)]
	pub(crate) fn render<'a>(&'a self, resources: &'a Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<String, CordialError>
	{
		let urlData = self.url.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;

		let mimeTypeWithoutParameters = urlData.mimeTypeWithoutParameters();
		let url = urlData.url_str();

		let xml = match self.media
		{
			None => format!("type=\"{}\" href=\"{}\"", mimeTypeWithoutParameters, url),
			Some(ref mediaQuery) => format!("type=\"{}\" media=\"{}\" href=\"{}\"", mimeTypeWithoutParameters, mediaQuery, url),
		};

		Ok(xml)
	}

	#[inline(always)]
	fn url_default() -> ResourceReference
	{
		ResourceReference::new("/rss.css", ResourceTag::default)
	}

	#[inline(always)]
	fn media_default() -> Option<String>
	{
		Some("all".to_string())
	}
}
