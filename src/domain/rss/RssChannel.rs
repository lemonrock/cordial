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
	#[serde(default)] details: HashMap<String, RssChannelLanguageSpecific>,
	#[serde(default = "RssChannel::image_url_default")] image_url: String,
	#[serde(default)] managing_editor: EMailAddress, // Consider using a back-reference to an users list
	#[serde(default)] web_master: EMailAddress, // Consider using a back-reference to an users list
	#[serde(default)] categories: Vec<String>,
	#[serde(default = "RssChannel::feedly_default")] feedly: Option<RssFeedlyChannel>,
}

impl RssChannel
{
	const ImageUrlTag: UrlTag = UrlTag::primary_image;
	
	#[inline(always)]
	pub fn renderResource<'a, 'b: 'a, 'c>(&'c self, languageData: &LanguageData, handlebars: &mut Handlebars, configuration: &Configuration, newResponses: &'b mut Responses, oldResponses: &Arc<Responses>, rssItems: &HashMap<String, Vec<RssItem>>, primary_iso_639_1_alpha_2_language_code: &str, resources: &'a Resources, parentGoogleAnalyticsCode: Option<&str>) -> Result<(), CordialError>
	{
		let iso_639_1_alpha_2_language_code = languageData.iso_639_1_alpha_2_language_code;
		
		let detail = match self.details.get(iso_639_1_alpha_2_language_code)
		{
			None => return Err(CordialError::Configuration(format!("No RSS details for language '{}'", iso_639_1_alpha_2_language_code))),
			Some(detail) => detail,
		};
		
		let description = &detail.description;
		const FeedlyDescriptionLength: usize = 140;
		if description.len() > FeedlyDescriptionLength
		{
			return Err(CordialError::Configuration("RSS description exceeds Feedly's maximum of 140 characters".to_owned()))
		}
		
		let resource = resources.get(&self.image_url).ok_or_else(|| CordialError::Configuration(format!("Could not find RSS resource for image_url '{}'", &self.image_url)))?.try_borrow()?;
		let imageMetaData = resource.imageMetaData().ok_or_else(|| CordialError::Configuration(format!("Could not find image meta data for image_url '{}'", &self.image_url)))?;
		let (imageUrl, imageJsonValue) = resource.urlAndJsonValue(primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code), &Self::ImageUrlTag).ok_or_else(|| CordialError::Configuration(format!("Could not find RSS {:?} for image_url '{}'", Self::ImageUrlTag, &self.image_url)))?;
		let imageAbstract = imageMetaData.abstract_(iso_639_1_alpha_2_language_code)?;
		let imageWidth = imageJsonValue.u32("width")?;
		let imageHeight = imageJsonValue.u32("height")?;
		let image_alt = &imageAbstract.alt;
		let image_tooltip = &imageAbstract.title;
		
		let deploymentDateTime: DateTime<Utc> = DateTime::from(configuration.deploymentDate);
		let timeToLiveInMinutes =
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
		};
		let unversionedCanonicalUrl = languageData.url(&format!("{}.rss.xml", iso_639_1_alpha_2_language_code))?;
		let rssItems = rssItems.get(iso_639_1_alpha_2_language_code).unwrap();
		let emptyAttributes = [];
		let mut eventWriter = Self::createEventWriter();
		
		eventWriter.writeBasicXmlDocumentPreamble()?;
		
		for stylesheet in self.stylesheets.iter()
		{
			let data = stylesheet.render(primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code), resources, newResponses)?;
			eventWriter.writeProcessingInstruction("xml-stylesheet", Some(&data))?;
		}
		
		let namespace = Namespace
		(
			btreemap!
			{
				"dc".to_owned() => "http://purl.org/dc/elements/1.1/".to_owned(),
				"content".to_owned() => "http://purl.org/rss/1.0/modules/content/".to_owned(),
				"atom".to_owned() => "http://www.w3.org/2005/Atom".to_owned(),
				"media".to_owned() => "http://search.yahoo.com/mrss/".to_owned(),
				"webfeeds".to_owned() => "http://webfeeds.org/rss/1.0".to_owned(),
			}
		);
		
		let attributes =
		[
			Attribute::new(Name::local("version"), "2.0"),
		];
		eventWriter.writeWithinElement(Name::local("rss"), &namespace, &attributes, |eventWriter|
		{
			eventWriter.writeWithinElement(Name::local("channel"), &namespace, &emptyAttributes, |eventWriter|
			{
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "title", &detail.title)?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "link", languageData.baseUrl(false).unwrap().as_ref())?;
				
				if let Some(ref feedly) = self.feedly
				{
					feedly.writeXml(eventWriter, &namespace, &emptyAttributes, primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code, resources, parentGoogleAnalyticsCode)?;
				}
				
				let attributes =
				[
					Attribute::new(Name::local("rel"), "self"),
					Attribute::new(Name::local("type"), "application/rss+xml"),
					Attribute::new(Name::local("href"), unversionedCanonicalUrl.as_ref()),
				];
				eventWriter.writeEmptyElement(&namespace, &attributes, Name::prefixed("link", "atom"))?;
				
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "description", description)?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "language", languageData.iso_639_1_alpha_2_language_code)?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "copyright", &detail.copyright)?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "managingEditor", &self.managing_editor.to_string())?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "webMaster", &self.web_master.to_string())?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "pubDate", &deploymentDateTime.to_rfc2822())?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "lastBuildDate",  &deploymentDateTime.to_rfc2822())?;
				for category in self.categories.iter()
				{
					eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "category", category)?;
				}
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "generator", "cordial")?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "docs", "http://www.rssboard.org/rss-specification")?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "ttl", &format!("{}", timeToLiveInMinutes))?;
				eventWriter.writeWithinElement(Name::local("image"), &namespace, &emptyAttributes, |eventWriter|
				{
					eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "url", imageUrl.as_str())?;
					eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "title", image_alt)?;
					eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "description", image_tooltip)?;
					if imageWidth != 0
					{
						eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "width", &format!("{}", imageWidth))?;
					}
					if imageHeight != 0
					{
						eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "height", &format!("{}", imageHeight))?;
					}
					Ok(())
				})?;
				
				for rssItem in rssItems.iter()
				{
					rssItem.writeXml(eventWriter, &namespace, &emptyAttributes)?;
				}
				
				Ok(())
			})
		})?;
		
		let headers = generateHeaders(handlebars, &self.headers, Some(languageData), HtmlVariant::Canonical, configuration, true, self.max_age_in_seconds, true, &unversionedCanonicalUrl)?;
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
	fn image_url_default() -> String
	{
		"/organization-logo.png".to_owned()
	}
	
	#[inline(always)]
	fn feedly_default() -> Option<RssFeedlyChannel>
	{
		Some(RssFeedlyChannel::default())
	}
}
