// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RssItem
{
	pub(crate) rssItemLanguageVariant: RssItemLanguageVariant,
	pub(crate) lastModificationDate: Option<DateTime<Utc>>,
	pub(crate) author: EMailAddress,
	pub(crate) categories: BTreeSet<String>,
}

impl RssItem
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> XmlWriterResult
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
				let fileSize = format!("{}", primaryImage.fileSize);
				let width = format!("{}", primaryImage.width);
				let height = format!("{}", primaryImage.height);
				
				let enclosureAttributes =
				[
					XmlAttribute::new(Name::local("url"), primaryImage.url.as_ref()),
					XmlAttribute::new(Name::local("length"), &fileSize),
					XmlAttribute::new(Name::local("type"), primaryImage.mimeType.as_ref()),
				];
				eventWriter.writeEmptyElement(namespace, &enclosureAttributes, Name::local("enclosure"))?;
				
				let contentAttributes =
				[
					XmlAttribute::new(Name::local("url"), primaryImage.url.as_ref()),
					XmlAttribute::new(Name::local("medium"), "image"),
					XmlAttribute::new(Name::local("height"), &height),
					XmlAttribute::new(Name::local("width"), &width),
					XmlAttribute::new(Name::local("fileSize"), &fileSize),
					XmlAttribute::new(Name::local("type"), primaryImage.mimeType.as_ref()),
					XmlAttribute::new(Name::local("lang"), primaryImage.iso639Dash1Alpha2Language.to_iso_639_1_alpha_2_language_code()),
				];
				eventWriter.writeWithinElement(Name::prefixed("content", "media"), &namespace, &contentAttributes, |eventWriter|
				{
					eventWriter.writeTextElement(namespace, &emptyAttributes, Name::prefixed("description", "media"), &primaryImage.alt)?;
					
					eventWriter.writeTextElement(namespace, &emptyAttributes, Name::prefixed("credit", "media"), &primaryImage.credit)?;
					
					let thumbnailAttributes =
					[
						XmlAttribute::new(Name::local("width"), &width),
						XmlAttribute::new(Name::local("height"), &height),
						XmlAttribute::new(Name::local("url"), primaryImage.url.as_ref()),
					];
					eventWriter.writeEmptyElement(namespace, &thumbnailAttributes, Name::prefixed("thumbnail", "media"))
				})?;
			}
			Ok(())
		})
	}
}
