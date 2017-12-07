// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Default, Debug, Clone)]
pub(crate) struct HtmlDocumentItem
{
	#[serde(default)] pub(crate) categories: BTreeSet<String>,
	#[serde(default)] pub(crate) author: EMailAddress,
	#[serde(default)] pub(crate) source: Option<ExternalUrlOrResourceUrl>,
	#[serde(default)] pub(crate) htmlDocumentItemVariant: HtmlDocumentItemVariant,
}

impl HtmlDocumentItem
{
	#[inline(always)]
	pub(crate) fn withPodcastRssHtml(&self, containingHtmlDocumentLastModifiedDate: Option<DateTime<Utc>>, description: Rc<String>, rssHtml: Vec<u8>, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		self.htmlDocumentItemVariant.withPodcastRssHtml(containingHtmlDocumentLastModifiedDate, description, rssHtml, iso639Dash1Alpha2Language)
	}
	
	#[inline(always)]
	pub(crate) fn titleDescriptionContentEncodedAndPublicationDate<R, User: FnMut(&str, &str, Option<&str>, Option<DateTime<Utc>>) -> Result<R, CordialError>>(&self, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, user: User) -> Result<R, CordialError>
	{
		self.htmlDocumentItemVariant.titleDescriptionContentEncodedAndPublicationDate(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, user)
	}
	
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		if let Some(ref source) = self.source
		{
			source.useUrlAndTitle(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, |url, title|
			{
				eventWriter.writeUnprefixedTextElement(&namespace, &["url".xml_url_attribute(url)], "source", title)
			})?;
		}
		
		for category in self.categories.iter()
		{
			eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "category", category)?;
		}
		
		eventWriter.writeUnprefixedTextElementString(&namespace, &emptyAttributes, "author", self.author.to_string())?;
		
		eventWriter.writePrefixedTextElement(&namespace, &emptyAttributes, RssChannel::DcNamespacePrefix, "creator", &self.author.full_name)?;
		
		self.htmlDocumentItemVariant.writeXml(eventWriter, namespace, emptyAttributes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)
	}
}
