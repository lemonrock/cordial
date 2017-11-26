// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RssItem
{
	pub(crate) rssItemLanguageVariant: RssItemLanguageVariant,
	pub(crate) lastModificationDate: Option<DateTime<Utc>>,
	pub(crate) author: Rc<EMailAddress>,
	pub(crate) categories: Rc<BTreeSet<String>>,
}

impl RssItem
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		let rssItemLanguageVariant = &self.rssItemLanguageVariant;
		
		let versionAttributes =
		[
			XmlAttribute::new(Name::local("version"), "2.0"),
		];
		eventWriter.writeWithinElement(Name::local("item"), &namespace, &versionAttributes, |eventWriter|
		{
			eventWriter.writeCDataElement(&namespace, &emptyAttributes, Name::local("title"), &rssItemLanguageVariant.webPageDescription)?;
			
			eventWriter.writeCDataElement(&namespace, &emptyAttributes, Name::local("description"), unsafe { from_utf8_unchecked(&rssItemLanguageVariant.webPageUsefulContentHtml) })?;
			
			eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "link", rssItemLanguageVariant.languageSpecificUrl.as_ref())?;
			
			let guidAttributes =
			[
				XmlAttribute::new(Name::local("isPermaLink"), "true"),
			];
			eventWriter.writeUnprefixedTextElement(&namespace, &guidAttributes, "guid", rssItemLanguageVariant.languageSpecificUrl.as_ref())?;
			
			for category in self.categories.iter()
			{
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "category", category)?;
			}
			
			if let Some(lastModificationDate) = self.lastModificationDate
			{
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "pubData", &lastModificationDate.to_rfc2822())?;
			}
			
			eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "author", &self.author.to_string())?;
			
			eventWriter.writePrefixedTextElement(&namespace, &emptyAttributes, "dc", "creator", &self.author.full_name)?;
			
			if let Some(ref primaryImage) = rssItemLanguageVariant.primaryImage
			{
				primaryImage.writeXml(eventWriter, namespace, emptyAttributes)?;
			}
			Ok(())
		})
	}
}
