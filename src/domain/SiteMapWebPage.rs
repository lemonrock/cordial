// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct SiteMapWebPage
{
	lastModified: DateTime<Utc>,
	changeFrequency: SiteMapChangeFrequency,
	priority: SiteMapPriority,
	urlsByIsoLanguageCode: BTreeMap<String, Url>,
	images: Vec<SiteMapWebPageImage>,
}

impl SiteMapWebPage
{
	#[inline(always)]
	pub(crate) fn new(lastModified: SystemTime, changeFrequency: SiteMapChangeFrequency, priority: SiteMapPriority, urlsByIsoLanguageCode: BTreeMap<String, Url>, images: Vec<SiteMapWebPageImage>) -> Self
	{
		Self
		{
			lastModified: DateTime::from(lastModified),
			changeFrequency,
			priority,
			urlsByIsoLanguageCode,
			images
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, primary_iso_639_1_alpha_2_language_code: &str, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[Attribute<'a>]) -> ::xml::writer::Result<bool>
	{
		let locationUrl = self.urlsByIsoLanguageCode.get(primary_iso_639_1_alpha_2_language_code);
		if locationUrl.is_none()
		{
			return Ok(false);
		}
		
		eventWriter.writeWithinElement(Name::local("url"), namespace, emptyAttributes, ||
		{
			eventWriter.writeUnprefixedTextElement(namespace, emptyAttributes, "loc", locationUrl.unwrap().as_ref())?;
			eventWriter.writeUnprefixedTextElement(namespace, emptyAttributes, "lastmod", &self.lastModified.to_rfc3339())?;
			eventWriter.writeUnprefixedTextElement(namespace, emptyAttributes, "changefreq", self.changeFrequency.as_str())?;
			eventWriter.writeUnprefixedTextElement(namespace, emptyAttributes, "priority", self.priority.as_str())?;
			
			for (iso_639_1_alpha_2_language_code, url) in self.urlsByIsoLanguageCode.iter()
			{
				Self::writeXhtmlTranslationElement(eventWriter, namespace, emptyAttributes, iso_639_1_alpha_2_language_code, url);
			}
			
			for image in self.images.iter()
			{
				image.writeXml(eventWriter, namespace, emptyAttributes)?;
			}
			
			Ok(())
		});
		
		Ok(true)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn writeXhtmlTranslationElement<'a, W: Write>(eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[Attribute<'a>], iso_639_1_alpha_2_language_code: &str, url: &Url) -> XmlWriterResult
	{
		eventWriter.writeEmptyElement(namespace,
		&[
			Attribute::new(Name::local("rel"), "alternate"),
			Attribute::new(Name::local("hreflang"), iso_639_1_alpha_2_language_code),
			Attribute::new(Name::local("href"), url.as_ref()),
		], Name::prefixed("link", "xhtml"))
	}
}
