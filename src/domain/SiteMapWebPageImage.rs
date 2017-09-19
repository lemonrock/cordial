// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct SiteMapWebPageImage
{
	url: Url,
	caption: String,
	geographicLocation: Option<String>,
	title: String,
	licenseUrl: Url,
}

impl SiteMapWebPageImage
{
	#[inline(always)]
	fn writeXml<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[Attribute<'a>]) -> XmlWriterResult
	{
		eventWriter.writeWithinElement(Name::prefixed("image", "image"), namespace, emptyAttributes, |eventWriter|
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "image", "loc", self.url.as_ref())?;
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "image", "caption", &self.caption)?;
			if let Some(ref geographicLocation) = self.geographicLocation
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "image", "geo_location", geographicLocation)?;
			}
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "image", "title", &self.title)?;
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "image", "license_url", self.licenseUrl.as_ref())
		})
	}
}
