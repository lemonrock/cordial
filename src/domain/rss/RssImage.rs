// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RssImage
{
	pub(crate) url: ResourceReference,
	pub(crate) imageAbstract: Rc<ImageAbstract>,
	pub(crate) credit: FullName,
	pub(crate) iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language,
}

impl RssImage
{
	pub(crate) const MediaNamespacePrefix: &'static str = "media";
	
	pub(crate) const MediaNamespaceUrl: &'static str = "http://search.yahoo.com/mrss/";
	
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		let urlData = self.url.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
		urlData.validateIsSuitableForRssImage()?;
		
		let mimeType = urlData.mimeType().as_ref();
		let (width, height, fileSize) = urlData.image()?;
		
		// <enclosure>
		{
			let lengthAttribute = "length".xml_u64_attribute(fileSize);
			let enclosureAttributes =
			[
				"url".xml_url_from_UrlData_attribute(&urlData),
				lengthAttribute.borrow(),
				"type".xml_str_attribute(mimeType),
			];
			eventWriter.writeEmptyElement(namespace, &enclosureAttributes, "enclosure".xml_local_name())?;
		}
		
		// <media:content>; used by MailChimp, for instance
		{
			let heightAttribute = "height".xml_u32_attribute(height);
			let widthAttribute = "width".xml_u32_attribute(width);
			let fileSizeAttribute = "fileSize".xml_u64_attribute(fileSize);
			let contentAttributes =
			[
				"url".xml_url_from_UrlData_attribute(&urlData),
				"medium".xml_str_attribute("image"),
				heightAttribute.borrow(),
				widthAttribute.borrow(),
				fileSizeAttribute.borrow(),
				"type".xml_str_attribute(mimeType),
				"lang".xml_language_attribute(self.iso639Dash1Alpha2Language),
			];
			eventWriter.writeWithinElement(Self::MediaNamespacePrefix.prefixes_xml_name("content"), &namespace, &contentAttributes, |eventWriter|
			{
				eventWriter.writeTextElement(namespace, &emptyAttributes, Self::MediaNamespacePrefix.prefixes_xml_name("description"), &self.imageAbstract.alt)?;
				
				eventWriter.writeTextElement(namespace, &emptyAttributes, Self::MediaNamespacePrefix.prefixes_xml_name("credit"), &self.credit)?;
				
				let thumbnailAttributes =
				[
					widthAttribute.borrow(),
					heightAttribute.borrow(),
					"url".xml_url_from_UrlData_attribute(&urlData),
				];
				eventWriter.writeEmptyElement(namespace, &thumbnailAttributes, Self::MediaNamespacePrefix.prefixes_xml_name("thumbnail"))
			})
		}
	}
}
