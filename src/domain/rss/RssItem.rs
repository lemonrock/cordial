// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RssItem
{
	pub(crate) canonicalLinkUrl: Rc<Url>,
	pub(crate) htmlDocumentItem: Rc<HtmlDocumentItem>,
}

impl RssItem
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		let canonicalLinkUrl = &self.canonicalLinkUrl;
		
		eventWriter.writeWithinLocalElement("item", &namespace, &RssChannel::rssVersionAttributes(), |eventWriter|
		{
			self.htmlDocumentItem.titleDescriptionContentEncodedAndPublicationDate(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, |title, description, contentEncoded, publicationDate|
			{
				eventWriter.writeCDataElement(&namespace, &emptyAttributes, "title".xml_local_name(), title)?;
				
				eventWriter.writeCDataElement(&namespace, &emptyAttributes, "description".xml_local_name(), description)?;
				
				if let Some(ref contentEncoded) = contentEncoded
				{
					eventWriter.writeCDataElement(&namespace, &emptyAttributes, RssChannel::ContentNamespacePrefix.prefixes_xml_name("encoded"), contentEncoded)?;
				}
				
				if let Some(publicationDate) = publicationDate
				{
					eventWriter.writeUnprefixedTextElementRfc2822(&namespace, &emptyAttributes, "pubDate", publicationDate)?;
				}
				
				Ok(())
			})?;
			
			eventWriter.writeUnprefixedTextElementUrl(&namespace, &emptyAttributes, "link", canonicalLinkUrl)?;
			
			eventWriter.writeUnprefixedTextElementUrl(&namespace, &[ "isPermaLink".xml_str_attribute("true") ], "guid", canonicalLinkUrl)?;
			
			
			self.htmlDocumentItem.writeXml(eventWriter, namespace, emptyAttributes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)
		})
	}
}
