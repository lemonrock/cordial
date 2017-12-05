// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RssItem
{
	pub(crate) rssItemLanguageSpecific: RssItemLanguageSpecific,
	pub(crate) lastModificationDate: Option<DateTime<Utc>>,
	pub(crate) author: Rc<EMailAddress>,
	pub(crate) categories: Rc<BTreeSet<String>>,
	pub(crate) source: Option<ResourceUrl>,
	// third-party source
	// rss <source url="">Title Text</source>
}

impl RssItem
{
	pub(crate) const DcNamespacePrefix: &'static str = "dc";
	
	pub(crate) const DcNamespaceUrl: &'static str = "http://purl.org/dc/elements/1.1/";
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<(), CordialError>
	{
		let rssItemLanguageVariant = &self.rssItemLanguageSpecific;
		
		eventWriter.writeWithinLocalElement("item", &namespace, &RssChannel::rssVersionAttributes(), |eventWriter|
		{
			eventWriter.writeCDataElement(&namespace, &emptyAttributes, "title".xml_local_name(), &rssItemLanguageVariant.webPageDescription)?;
			
			eventWriter.writeCDataElement(&namespace, &emptyAttributes, "description".xml_local_name(), unsafe { from_utf8_unchecked(&rssItemLanguageVariant.webPageUsefulContentHtml) })?;
			
			let languageSpecificUrl = &rssItemLanguageVariant.languageSpecificUrl;
			
			eventWriter.writeUnprefixedTextElementUrl(&namespace, &emptyAttributes, "link", languageSpecificUrl)?;
			
			if let Some(ref source) = self.source
			{
				let (url, title) = ResourceReference
				{
					resource: source.clone(),
					tag: ResourceTag::default,
				}.urlAndAnchorTitleAttribute(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language.unwrap_or(fallbackIso639Dash1Alpha2Language))?;
				eventWriter.writeUnprefixedTextElement(&namespace, &["url".xml_url_attribute(&url)], "source", &title)?;
			}
			eventWriter.writeUnprefixedTextElementUrl(&namespace, &[ "isPermaLink".xml_str_attribute("true") ], "guid", languageSpecificUrl)?;
			
			for category in self.categories.iter()
			{
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "category", category)?;
			}
			
			if let Some(lastModificationDate) = self.lastModificationDate
			{
				eventWriter.writeUnprefixedTextElementRfc2822(&namespace, &emptyAttributes, "pubData", lastModificationDate)?;
			}
			
			eventWriter.writeUnprefixedTextElementString(&namespace, &emptyAttributes, "author", self.author.to_string())?;
			
			eventWriter.writePrefixedTextElement(&namespace, &emptyAttributes, Self::DcNamespacePrefix, "creator", &self.author.full_name)?;
			
			if let Some(ref primaryImage) = rssItemLanguageVariant.primaryImage
			{
				primaryImage.writeXml(eventWriter, namespace, emptyAttributes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			}
			
			Ok(())
		})
	}
}
