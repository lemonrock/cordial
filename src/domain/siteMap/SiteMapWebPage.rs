// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct SiteMapWebPage
{
	pub(crate) lastModified: Option<DateTime<Utc>>,
	pub(crate) changeFrequency: SiteMapChangeFrequency,
	pub(crate) priority: SiteMapPriority,
	pub(crate) urlsByIso639Dash1Alpha2Language: BTreeMap<Iso639Dash1Alpha2Language, Url>,
	pub(crate) images: Vec<SiteMapWebPageImage>,
	pub(crate) audiosVideos: Vec<SiteMapWebPageAudioVideo>,
}

impl SiteMapWebPage
{
	pub(crate) const XhtmlNamespacePrefix: &'static str = "xhtml";
	
	pub(crate) const XhtmlNamespaceUrl: &'static str = "http://www.w3.org/1999/xhtml";
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<bool, CordialError>
	{
		let locationUrl = match self.urlsByIso639Dash1Alpha2Language.get(&iso639Dash1Alpha2Language)
		{
			None => return Ok(false),
			Some(locationUrl) => locationUrl,
		};
		
		eventWriter.writeWithinLocalElement("url", namespace, emptyAttributes, |eventWriter|
		{
			eventWriter.writeUnprefixedTextElement(namespace, emptyAttributes, "loc", locationUrl.as_ref())?;
			if let Some(lastModified) = self.lastModified
			{
				eventWriter.writeUnprefixedTextElementRfc3339(namespace, emptyAttributes, "lastmod", lastModified)?;
			}
			eventWriter.writeUnprefixedTextElement(namespace, emptyAttributes, "changefreq", self.changeFrequency.as_str())?;
			eventWriter.writeUnprefixedTextElement(namespace, emptyAttributes, "priority", self.priority.as_str())?;
			
			for (iso639Dash1Alpha2Language, url) in self.urlsByIso639Dash1Alpha2Language.iter()
			{
				Self::writeXhtmlTranslationElement(eventWriter, namespace, *iso639Dash1Alpha2Language, url)?;
			}
			
			for image in self.images.iter()
			{
				image.writeXml(eventWriter, namespace, emptyAttributes, resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
			}
			
			for audioVideo in self.audiosVideos.iter()
			{
				audioVideo.writeXml(eventWriter, namespace, emptyAttributes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			}
			
			Ok(())
		})?;
		
		Ok(true)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn writeXhtmlTranslationElement<'a, W: Write>(eventWriter: &mut EventWriter<W>, namespace: &Namespace, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, url: &Url) -> Result<(), CordialError>
	{
		eventWriter.writeEmptyElement
		(
			namespace,
			&[
				"rel".xml_str_attribute("alternate"),
				"hreflang".xml_language_attribute(iso639Dash1Alpha2Language),
				"href".xml_str_attribute(url.as_ref()),
			],
			Self::XhtmlNamespacePrefix.prefixes_xml_name("link")
		)
	}
}
