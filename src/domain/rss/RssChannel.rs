// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct RssChannel
{
	#[serde(default)] headers: HashMap<String, String>,
	#[serde(default = "RssChannel::max_age_in_seconds_default")] max_age_in_seconds: u32,
	#[serde(default)] compression: Compression,
	#[serde(default)] stylesheets: Vec<StylesheetLink>,
	#[serde(default)] details: HashMap<Iso639Dash1Alpha2Language, RssChannelLanguageSpecific>,
	#[serde(default = "RssChannel::image_url_default")] image_url: ResourceReference,
	#[serde(default)] managing_editor: EMailAddress, // Consider using a back-reference to an users list
	#[serde(default)] web_master: EMailAddress, // Consider using a back-reference to an users list
	#[serde(default)] categories: Vec<RssCategoryName>,
	#[serde(default = "RssChannel::feedly_default")] feedly: Option<FeedlyRssChannel>,
	#[serde(default)] itunes: Option<ITunesRssChannel>,
}

impl RssChannel
{
	#[inline(always)]
	fn timeToLiveInMinutes(&self) -> u32
	{
		{
			let minutesRoundedDown = self.max_age_in_seconds / 60;
			if minutesRoundedDown * 60 != self.max_age_in_seconds
			{
				minutesRoundedDown + 1
			}
			else
			{
				minutesRoundedDown
			}
		}
	}
	
	pub(crate) const AtomNamespacePrefix: &'static str = "atom";
	
	pub(crate) const AtomNamespaceUrl: &'static str = "http://www.w3.org/2005/Atom";
	
	#[inline(always)]
	pub(crate) fn rssVersionAttributes<'a>() -> [XmlAttribute<'a>; 1]
	{
		[ "version".xml_str_attribute("2.0") ]
	}
	
	#[inline(always)]
	pub(crate) fn renderRssChannel<'a, 'b: 'a, 'c>(&'c self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, languageData: &LanguageData, handlebars: &HandlebarsWrapper, configuration: &Configuration, newResponses: &'b mut Responses, oldResponses: &Arc<Responses>, resources: &'a Resources, parentGoogleAnalyticsCode: Option<&str>, rssChannelName: &Rc<RssChannelName>, rssItems: &Vec<RssItem>) -> Result<(), CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let detail = match self.details.get(&iso639Dash1Alpha2Language)
		{
			None => return Err(CordialError::Configuration(format!("No RSS details for language '{}'", iso639Dash1Alpha2Language))),
			Some(detail) => detail,
		};
		
		// trim() because Apple iTunes podcasts should not have leading or trailing whitespace: https://help.apple.com/itc/podcasts_connect/?lang=en#/itcb54353390
		let title = detail.title.trim();
		let description = detail.description.trim();
		let copyright = detail.copyright.trim();
		
		if self.itunes.is_some()
		{
			const ITunesTitleLength: usize = 255;
			if title.chars().count() > ITunesTitleLength
			{
				return Err(CordialError::Configuration(format!("RSS title exceeds iTunes's maximum of {} characters", ITunesTitleLength)))
			}
			
			const ITunesDescriptionLength: usize = 4000;
			if description.chars().count() > ITunesDescriptionLength
			{
				return Err(CordialError::Configuration(format!("RSS description exceeds iTunes's maximum of {} characters", ITunesDescriptionLength)))
			}
			
			const ITunesCopyrightLength: usize = 255;
			if copyright.chars().count() > ITunesCopyrightLength
			{
				return Err(CordialError::Configuration(format!("RSS description exceeds iTunes's maximum of {} characters", ITunesCopyrightLength)))
			}
		}
		
		if self.feedly.is_some()
		{
			const FeedlyDescriptionLength: usize = 140;
			if description.chars().count() > FeedlyDescriptionLength
			{
				return Err(CordialError::Configuration("RSS description exceeds Feedly's maximum of 140 characters".to_owned()))
			}
		}
		
		let deploymentDateTime: DateTime<Utc> = DateTime::from(configuration.deploymentDate);
		let unversionedCanonicalUrl = ResourceUrl::rssUrl(rssChannelName, iso639Dash1Alpha2Language).url(languageData)?;
		let emptyAttributes = [];
		let mut eventWriter = Self::createEventWriter();
		
		eventWriter.writeBasicXmlDocumentPreamble()?;
		
		for stylesheetLink in self.stylesheets.iter()
		{
			let data = stylesheetLink.render(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			eventWriter.writeProcessingInstruction("xml-stylesheet", Some(&data))?;
		}
		
		let rssNamespace = Namespace
		(
			btreemap!
			{
				RssItem::DcNamespacePrefix.to_owned() => RssItem::DcNamespaceUrl.to_owned(), // also needed by Feedly
				"content".to_owned() => "http://purl.org/rss/1.0/modules/content/".to_owned(), // seems to be needed by Feedly
				Self::AtomNamespacePrefix.to_owned() => Self::AtomNamespaceUrl.to_owned(),
				RssImage::MediaNamespacePrefix.to_owned() => RssImage::MediaNamespaceUrl.to_owned(),
				FeedlyRssChannel::WebfeedsNamespacePrefix.to_owned() => FeedlyRssChannel::WebfeedsNamespaceUrl.to_owned(),
				ITunesRssChannel::ITunesNamespacePrefix.to_owned() => ITunesRssChannel::ITunesNamespaceUrl.to_owned(),
			}
		);
		
		eventWriter.writeWithinLocalElement("rss", &rssNamespace, &Self::rssVersionAttributes(), |eventWriter|
		{
			eventWriter.writeWithinLocalElement("channel", &rssNamespace, &emptyAttributes, |eventWriter|
			{
				eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "title", title)?;
				
				eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "link", languageData.baseUrl(false).unwrap().as_ref())?;
				
				if let Some(ref feedly) = self.feedly
				{
					feedly.writeXml(eventWriter, &rssNamespace, &emptyAttributes, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, resources, parentGoogleAnalyticsCode)?;
				}
				
				let linkAttributes =
				[
					"rel".xml_str_attribute("self"),
					"type".xml_str_attribute("application/rss+xml"),
					"href".xml_url_attribute(&unversionedCanonicalUrl),
				];
				eventWriter.writeEmptyElement(&rssNamespace, &linkAttributes, Self::AtomNamespacePrefix.prefixes_xml_name("link"))?;
				
				eventWriter.writeCDataElement(&rssNamespace, &emptyAttributes, "description".xml_local_name(), description)?;
				
				eventWriter.writeUnprefixedTextElementLanguageCode(&rssNamespace, &emptyAttributes, "language", iso639Dash1Alpha2Language)?;
				
				eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "copyright", copyright)?;
				
				eventWriter.writeUnprefixedTextElementEMailAddress(&rssNamespace, &emptyAttributes, "managingEditor", &self.managing_editor)?;
				
				eventWriter.writeUnprefixedTextElementEMailAddress(&rssNamespace, &emptyAttributes, "webMaster", &self.web_master)?;
				
				eventWriter.writeUnprefixedTextElementRfc2822(&rssNamespace, &emptyAttributes, "pubDate", deploymentDateTime)?;
				
				eventWriter.writeUnprefixedTextElementRfc2822(&rssNamespace, &emptyAttributes, "lastBuildDate",  deploymentDateTime)?;
				
				for category in self.categories.iter()
				{
					eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "category", category)?;
				}
				
				eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "generator", "cordial")?;
				
				eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "docs", "http://www.rssboard.org/rss-specification")?;
				
				eventWriter.writeUnprefixedTextElementU32(&rssNamespace, &emptyAttributes, "ttl", self.timeToLiveInMinutes())?;
				
				eventWriter.writeWithinLocalElement("image", &rssNamespace, &emptyAttributes, |eventWriter|
				{
					let (urlData, resource) = self.image_url.urlDataAndResourceMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
					
					eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "url", urlData.url_str())?;
					
					let imageAbstract = resource.imageMetaData()?.imageAbstract(iso639Dash1Alpha2Language)?;
					
					eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "title", imageAbstract.alt.as_str())?;
					
					eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "description", imageAbstract.title.as_str())?;
					
					let (imageWidth, imageHeight) = urlData.dimensions()?;
					
					if imageWidth != 0 && imageHeight != 0
					{
						eventWriter.writeUnprefixedTextElementU32(&rssNamespace, &emptyAttributes, "width", imageWidth)?;
						
						eventWriter.writeUnprefixedTextElementU32(&rssNamespace, &emptyAttributes, "height", imageHeight)?;
					}
					
					Ok(())
				})?;
				
				for rssItem in rssItems.iter()
				{
					rssItem.writeXml(eventWriter, &rssNamespace, &emptyAttributes, resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
				}
				
				Ok(())
			})
		})?;
		
		const CanBeCompressed: bool = true;
		const CanBeDownloaded: bool = true;
		let headers = HeaderGenerator
		{
			handlebars,
			headerTemplates: &self.headers,
			ifLanguageAwareLanguageData: Some(languageData),
			configuration,
		}.generateHeadersForAsset(CanBeCompressed, self.max_age_in_seconds, CanBeDownloaded, &unversionedCanonicalUrl)?;
		
		let mut bodyUncompressed = eventWriter.into_inner();
		bodyUncompressed.shrink_to_fit();
		let bodyCompressed = self.compression.compress(&bodyUncompressed)?;
		
		let xmlMimeType = "text/xml; charset=utf-8".parse().unwrap();
		let staticResponse = StaticResponse::new(StatusCode::Ok, ContentType(xmlMimeType), headers, bodyUncompressed, Some(bodyCompressed));
		
		newResponses.addResponse(unversionedCanonicalUrl, RegularAndPjaxStaticResponse::regular(staticResponse), oldResponses.clone());
		
		Ok(())
	}
	
	#[inline(always)]
	fn createEventWriter<'a>() -> EventWriter<Vec<u8>>
	{
		let configuration = EmitterConfig
		{
			line_separator: Cow::Borrowed(""),
			indent_string: Cow::Borrowed(""),
			perform_indent: false,
			perform_escaping: true,
			write_document_declaration: true,
			normalize_empty_elements: true,
			cdata_to_characters: true,
			keep_element_names_stack: true,
			autopad_comments: false,
		};
		configuration.create_writer(Vec::with_capacity(32 * 1024))
	}
	
	#[inline(always)]
	fn max_age_in_seconds_default() -> u32
	{
		// BBC feeds use 15 minutes in September 2017
		15 * 60
	}
	
	#[inline(always)]
	fn image_url_default() -> ResourceReference
	{
		ResourceReference::new("/organization-logo.png", ResourceTag::default)
	}
	
	#[inline(always)]
	fn feedly_default() -> Option<FeedlyRssChannel>
	{
		Some(FeedlyRssChannel::default())
	}
}
