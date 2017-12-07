// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum HtmlDocumentItemVariant
{
	Article
	{
		#[serde(default, skip_deserializing)] details: RefCell<HashMap<Iso639Dash1Alpha2Language, ArticleLanguageSpecificRssItemVariant>>,
		#[serde(default)] image: Option<ResourceUrl>,
	},
	
	Podcast(Podcast),
}

impl Default for HtmlDocumentItemVariant
{
	#[inline(always)]
	fn default() -> Self
	{
		HtmlDocumentItemVariant::Article
		{
			details: Default::default(),
			image: None,
		}
	}
}

impl HtmlDocumentItemVariant
{
	#[inline(always)]
	pub(crate) fn withRssHtml(&self, description: Rc<String>, rssHtml: Vec<u8>, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		use self::HtmlDocumentItemVariant::*;
		
		match *self
		{
			Article { ref details, .. } =>
			{
				details.borrow_mut().insert(iso639Dash1Alpha2Language, ArticleLanguageSpecificRssItemVariant
				{
					rssTitle: description,
					rssDescription: rssHtml,
				});
				Ok(())
			}
			
			Podcast(ref podcast) => podcast.withRssHtml(description, rssHtml, iso639Dash1Alpha2Language),
		}
	}
	
	#[inline(always)]
	pub(crate) fn titleDescriptionAndContentEncoded<R, User: FnMut(&str, &str, Option<&str>) -> Result<R, CordialError>>(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, user: User) -> Result<R, CordialError>
	{
		use self::HtmlDocumentItemVariant::*;
		
		match *self
		{
			Article { ref details, .. } =>
			{
				let details = details.borrow();
				if let Some(languageSpecificRssItemVariant) = details.get(&iso639Dash1Alpha2Language)
				{
					languageSpecificRssItemVariant.titleDescriptionAndContentEncoded(user)
				}
				else if let Some(languageSpecificRssItemVariant) = details.get(&fallbackIso639Dash1Alpha2Language)
				{
					languageSpecificRssItemVariant.titleDescriptionAndContentEncoded(user)
				}
				else
				{
					Err(CordialError::Configuration("No ArticleLanguageSpecificRssItemVariant for language or its fallback".to_owned()))
				}
			}
			
			Podcast(ref podcast) => podcast.titleDescriptionAndContentEncoded(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, user),
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		use self::HtmlDocumentItemVariant::*;
		
		match *self
		{
			Article { ref image, .. } => if let &Some(ref image) = image
			{
				let (largestImageUrlData, largestImageResource) = ResourceReference
				{
					resource: image.clone(),
					tag: ResourceTag::largest_image,
				}.urlDataAndResourceMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
				largestImageUrlData.validateIsSuitableForRssImage()?;
				
				let mimeType = largestImageUrlData.mimeType().as_ref();
				let (width, height, fileSize) = largestImageUrlData.image()?;
				
				// <enclosure>
				{
					let lengthAttribute = "length".xml_u64_attribute(fileSize);
					let enclosureAttributes =
					[
						"url".xml_url_from_UrlData_attribute(&largestImageUrlData),
						lengthAttribute.borrow(),
						"type".xml_str_attribute(mimeType),
					];
					eventWriter.writeEmptyElement(namespace, &enclosureAttributes, "enclosure".xml_local_name())?;
				}
				
				// <media:content>; used by MailChimp, for instance
				{
					let widthAttribute = "width".xml_u32_attribute(width);
					let heightAttribute = "height".xml_u32_attribute(height);
					let fileSizeAttribute = "fileSize".xml_u64_attribute(fileSize);
					let contentAttributes =
					[
						"url".xml_url_from_UrlData_attribute(&largestImageUrlData),
						"medium".xml_str_attribute("image"),
						heightAttribute.borrow(),
						widthAttribute.borrow(),
						fileSizeAttribute.borrow(),
						"type".xml_str_attribute(mimeType),
					];
					eventWriter.writeWithinElement(RssChannel::MediaNamespacePrefix.prefixes_xml_name("content"), &namespace, &contentAttributes, |eventWriter|
					{
						let largestImageMetaData = largestImageResource.imageMetaData()?;
						
						if let Some(title) = largestImageMetaData.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?
						{
							eventWriter.writeTextElement(namespace, &["type".xml_str_attribute("plain")], RssChannel::MediaNamespacePrefix.prefixes_xml_name("title"), &title)?;
						}
						
						eventWriter.writeTextElement(namespace, &[ "type".xml_str_attribute("plain") ], RssChannel::MediaNamespacePrefix.prefixes_xml_name("description"), largestImageMetaData.alt(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?)?;
						
						eventWriter.writeTextElement(namespace, &emptyAttributes, RssChannel::MediaNamespacePrefix.prefixes_xml_name("credit"), &largestImageMetaData.credit)?;
						
						let (licenseUrl, licenseTitle) = largestImageMetaData.licenseUrlAndAnchorTitleAttribute(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
						let licenseUrl = licenseUrl.as_ref().as_str();
						eventWriter.writeTextElement(namespace, &[ "url".xml_str_attribute(licenseUrl) ], RssChannel::MediaNamespacePrefix.prefixes_xml_name("copyright"), &licenseTitle)?;
						eventWriter.writeTextElement(namespace, &[ "type".xml_str_attribute("text/html"), "href".xml_str_attribute(licenseUrl) ], RssChannel::MediaNamespacePrefix.prefixes_xml_name("license"), &licenseTitle)?;
						
						// thumbnail image
						{
							let thumbnailImageUrlData = ResourceReference
							{
								resource: image.clone(),
								tag: ResourceTag::smallest_image,
							}.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
							thumbnailImageUrlData.validateIsSuitableForRssImage()?;
							
							let (thumbnailWidth, thumbnailHeight, _) = thumbnailImageUrlData.image()?;
							let thumbnailWidthAttribute = "width".xml_u32_attribute(thumbnailWidth);
							let thumbnailHeightAttribute = "height".xml_u32_attribute(thumbnailHeight);
							
							let thumbnailAttributes =
							[
								"url".xml_url_from_UrlData_attribute(&thumbnailImageUrlData),
								thumbnailWidthAttribute.borrow(),
								thumbnailHeightAttribute.borrow(),
							];
							eventWriter.writeEmptyElement(namespace, &thumbnailAttributes, RssChannel::MediaNamespacePrefix.prefixes_xml_name("thumbnail"))
						}
					})
				}
			}
			else
			{
				Ok(())
			},
			
			Podcast(ref podcast) => podcast.writeXml(eventWriter, namespace, emptyAttributes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
		}
	}
}
