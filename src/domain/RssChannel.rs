// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct RssChannel
{
	// Should have MIME types of text/xsl or text/css, eg <?xml-stylesheet type="text/xsl" media="screen" href="/~d/styles/rss2full.xsl"?><?xml-stylesheet type="text/css" media="screen" href="http://feeds.feedburner.com/~d/styles/itemcontent.css"?>
	// Only seem to be used by Chrome
	#[serde(default)] stylesheets: Vec<ResourceReference>,
	#[serde(default = "RssChannel::external_stylesheet_mime_type_default")] external_stylesheet_mime_type: String,
	title: String, // should default to home page or feed title; need access to web pages collection
	description: String, // should default to
	
	// Maximum value for width is 144, default value is 88.
	// Maximum value for height is 400, default value is 31.
	// Probably should use organization logo
	// BBC uses 120x60, NYT 250x40, Google Publisher logos are 60x600px maximum
	#[serde(default = "RssChannel::image_url_default")] image_url: ResourceReference,
	image_alt: String, // this is the img element's alt attribute; in practice it should match RssChannel.title
	image_tooltip: String, // this is the a element's title attribute, ie tooltip
	
	copyright: String,
	managing_editor: EMailAddress, // Consider using a back-reference to an users list
	web_master: EMailAddress, // Consider using a back-reference to an users list
	#[serde(default)] categories: Vec<String>,
	#[serde(default = "RssChannel::feedly_default")] feedly: Option<RssFeedlyChannel>,
	
	#[serde(default)] headers: HashMap<String, String>,
	#[serde(default = "RssChannel::max_age_in_seconds_default")] max_age_in_seconds: u32,
	#[serde(default)] compression: Compression,
}

impl RssChannel
{
	#[inline(always)]
	pub fn renderResource<'a, 'b: 'a, 'c>(&'c self, languageData: (&str, &Language), handlebars: &mut Handlebars, configuration: &Configuration, newResources: &'b mut Resources, oldResources: &Arc<Resources>, rssItems: &HashMap<String, Vec<RssItem>>, primary_iso_639_1_alpha_2_language_code: &str, resources: &'a BTreeMap<String, Resource>) -> Result<(), CordialError>
	{
		const FeedlyDescriptionLength: usize = 140;
		if self.description.len() > FeedlyDescriptionLength
		{
			return Err(CordialError::Configuration("RSS feed description exceeds Feedly's maximum of 140 characters".to_owned()))
		}
		
		let iso_639_1_alpha_2_language_code = languageData.0;
		let rssChannelBaseUrlWithTrailingSlash = languageData.1.baseUrl(iso_639_1_alpha_2_language_code)?;
		
		let (imageUrl, imageWidthAndHeight) = match self.image_url.urlAndJsonValue(primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code), resources)
		{
			None => return Err(CordialError::Configuration(format!("Invalid rss.image_url {:?}", &self.image_url))),
			Some((url, jsonValue)) =>
			{
				let width = jsonValue["width"].unwrap().as_u64().unwrap() as u32;
				let height = jsonValue["height"].unwrap().as_u64().unwrap() as u32;
				Some(url, Some(width, height))
			}
		};
		
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
		let unversionedCanonicalUrl = rssChannelBaseUrlWithTrailingSlash.join(&format!("{}.rss.xml", iso_639_1_alpha_2_language_code)).unwrap();
		let rssItems = rssItems.get(iso_639_1_alpha_2_language_code).unwrap();
		let namespace = Namespace(BTreeMap::new());
		let emptyAttributes = [];
		let mut eventWriter = Self::createEventWriter();
		
		eventWriter.writeBasicXmlDocumentPreamble()?;
		
		for stylesheet in self.stylesheets
		{
			if let Some((url, response)) = stylesheet.urlAndResponse(primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code), resources, newResources)
			{
				let mimeType = if response.is_some()
				{
					response.contentMimeTypeWithoutParameters();
				}
				else
				{
					match self.external_stylesheet_mime_type.parse()
					{
						Err(error) => return Err(CordialError::Configuration(format!("Could not parse rss.external_stylesheet_mime_type because '{:?}", error))),
						Ok(mimeType) => mimeType,
					}
				};
				eventWriter.writeProcessingInstruction("xml-stylesheet", Some(&format!("type=\"{}\" media=\"screen\" href=\"{}\"", mimeType, url)))?;
			}
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
		
		let versionAttributes =
		[
			Attribute::new(Name::local("version"), "2.0"),
		];
		eventWriter.writeWithinElement(Name::local("rss"), &namespace, &versionAttributes, |eventWriter|
		{
			eventWriter.writeWithinElement(Name::local("channel"), &namespace, &emptyAttributes, |eventWriter|
			{
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "title", &self.title)?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "link", languageData.1.baseUrl(languageData.0).unwrap().as_ref())?;
				
				if let Some(feedly) = self.feedly
				{
					feedly.writeXml(eventWriter, &namespace, &emptyAttributes, primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code, resources, newResources)?;
				}
				
				let atomLinkAttributes =
				[
					Attribute::new(Name::local("rel"), "self"),
					Attribute::new(Name::local("type"), "application/rss+xml"),
					Attribute::new(Name::local("href"), unversionedCanonicalUrl.as_ref()),
				];
				eventWriter.writeEmptyElement(&namespace, &atomLinkAttributes, Name::prefixed("link", "atom"))?;
				
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "description", &self.description)?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "language", languageData.0)?;
				eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "copyright", &self.copyright)?;
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
					eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "title", &self.image_alt)?;
					eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "description", &self.image_tooltip)?;
					if let Some((width, height)) = imageWidthAndHeight
					{
						eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "width", &format!("{}", width))?;
						eventWriter.writeUnprefixedTextElement(&namespace, &emptyAttributes, "height", &format!("{}", height))?;
					}
					Ok(())
				})?;
				
				// rating, textInput, skipHours and skipDays are not generated
				
				for rssItem in rssItems.iter()
				{
					rssItem.writeXml(iso_639_1_alpha_2_language_code, eventWriter, &namespace, &emptyAttributes)?;
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
		
		newResources.addResource(unversionedCanonicalUrl, RegularAndPjaxStaticResponse::regular(staticResponse), oldResources.clone());
		
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
	fn image_url_default() -> ResourceReference
	{
		ResourceReference::internal("/organization-logo.png".to_owned(), Some(UrlTag::primary_image))
	}
	
	#[inline(always)]
	fn feedly_default() -> RssFeedlyChannel
	{
		Feedly::default()
	}
	
	#[inline(always)]
	fn external_stylesheet_mime_type_default() -> String
	{
		"text/css".to_owned()
	}
	
	#[inline(always)]
	fn max_age_in_seconds_default() -> u32
	{
		// BBC feeds use 15 minutes in September 2017
		15 * 60
	}
}
