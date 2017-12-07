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
	#[serde(default)] link: ResourceUrl,
	#[serde(default = "RssChannel::artwork_default")] artwork: ResourceUrl,

	#[serde(default)] categories: HashSet<RssCategoryName>,
	#[serde(default = "RssChannel::feedly_default")] feedly: Option<FeedlyRssChannel>,
	#[serde(default)] podcast: Option<PodcastRssChannel>,
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
	
	pub(crate) const ContentNamespacePrefix: &'static str = "content";
	
	pub(crate) const ContentNamespaceUrl: &'static str = "http://purl.org/rss/1.0/modules/content/";
	
	pub(crate) const DcNamespacePrefix: &'static str = "dc";
	
	pub(crate) const DcNamespaceUrl: &'static str = "http://purl.org/dc/elements/1.1/";
	
	pub(crate) const DcTermsNamespacePrefix: &'static str = "dcterms";
	
	pub(crate) const DcTermsNamespaceUrl: &'static str = "http://purl.org/dc/terms/";
	
	pub(crate) const GooglePlayNamespacePrefix: &'static str = "googleplay";
	
	pub(crate) const GooglePlayNamespaceUrl: &'static str = "http://www.google.com/schemas/play-podcasts/1.0";
	
	pub(crate) const ITunesNamespacePrefix: &'static str = "itunes";
	
	pub(crate) const ITunesNamespaceUrl: &'static str = "http://www.itunes.com/dtds/podcast-1.0.dtd";
	
	pub(crate) const MediaNamespacePrefix: &'static str = "media";
	
	pub(crate) const MediaNamespaceUrl: &'static str = "http://search.yahoo.com/mrss/";
	
	#[inline(always)]
	pub(crate) fn rssVersionAttributes<'a>() -> [XmlAttribute<'a>; 1]
	{
		[ "version".xml_str_attribute("2.0") ]
	}
	
	#[inline(always)]
	pub(crate) fn renderRssChannel<'a, 'b: 'a, 'c>(&'c self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, languageData: &LanguageData, handlebars: &HandlebarsWrapper, configuration: &Configuration, newResponses: &'b mut Responses, oldResponses: &Arc<Responses>, resources: &'a Resources, parentGoogleAnalyticsCode: Option<&str>, rssChannelName: &Rc<RssChannelName>, rssItems: &Vec<RssItem>) -> Result<(), CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		let isUsingFeedly = self.feedly.is_some();
		let isForPodcasting = self.podcast.is_some();
		
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
		
		let rssNamespace = namespace!
		{
			Self::DcNamespacePrefix => Self::DcNamespaceUrl,
			Self::DcTermsNamespacePrefix => Self::DcTermsNamespaceUrl,
			Self::ContentNamespacePrefix => Self::ContentNamespaceUrl,
			Self::AtomNamespacePrefix => Self::AtomNamespaceUrl,
			Self::MediaNamespacePrefix => Self::MediaNamespaceUrl,
			FeedlyRssChannel::WebfeedsNamespacePrefix => FeedlyRssChannel::WebfeedsNamespaceUrl,
			Self::ITunesNamespacePrefix => Self::ITunesNamespaceUrl,
			Self::GooglePlayNamespacePrefix => Self::GooglePlayNamespaceUrl,
		};
		
		eventWriter.writeWithinLocalElement("rss", &rssNamespace, &Self::rssVersionAttributes(), |eventWriter|
		{
			eventWriter.writeWithinLocalElement("channel", &rssNamespace, &emptyAttributes, |eventWriter|
			{
				let rssChannelLanguageSpecific = match self.details.get(&iso639Dash1Alpha2Language)
				{
					None => return Err(CordialError::Configuration(format!("No RSS details for language '{}'", iso639Dash1Alpha2Language))),
					Some(rssChannelLanguageSpecific) => rssChannelLanguageSpecific,
				};
				rssChannelLanguageSpecific.writeXml(eventWriter, &rssNamespace, &emptyAttributes, isUsingFeedly, isForPodcasting)?;
				
				let url = ResourceReference
				{
					resource: self.link.clone(),
					tag: ResourceTag::default,
				}.urlMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
				eventWriter.writeUnprefixedTextElementUrl(&rssNamespace, &emptyAttributes, "link", &url)?;
				
				eventWriter.writeUnprefixedTextElementLanguageCode(&rssNamespace, &emptyAttributes, "language", iso639Dash1Alpha2Language)?;
				
				if let Some(ref feedly) = self.feedly
				{
					feedly.writeXml(eventWriter, &rssNamespace, &emptyAttributes, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, resources, parentGoogleAnalyticsCode)?;
				}
				
				if let Some(ref podcast) = self.podcast
				{
					podcast.writeXml(eventWriter, &rssNamespace, &emptyAttributes, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, resources, &self.artwork)?;
				}
				
				// atom:link
				let linkAttributes =
				[
					"rel".xml_str_attribute("self"),
					"type".xml_str_attribute("application/rss+xml"),
					"href".xml_url_attribute(&unversionedCanonicalUrl),
				];
				eventWriter.writeEmptyElement(&rssNamespace, &linkAttributes, Self::AtomNamespacePrefix.prefixes_xml_name("link"))?;
				
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
					// RSS supports images with a maximum width of 144px and maximum height of 400px
					let (urlData, resource) = ResourceReference
					{
						resource: self.artwork.clone(),
						tag: ResourceTag::width_image(144),
					}.urlDataAndResourceMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
					urlData.validateIsSuitableForRssImage()?;
					
					eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "url", urlData.url_str())?;
					
					let imageAbstract = resource.imageMetaData()?.imageAbstract(iso639Dash1Alpha2Language)?;
					
					eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "title", imageAbstract.alt.as_str())?;
					
					eventWriter.writeUnprefixedTextElement(&rssNamespace, &emptyAttributes, "description", imageAbstract.title.as_str())?;
					
					let (imageWidth, imageHeight) = urlData.dimensions()?;
					
					// RSS supports images with a maximum width of 144px and maximum height of 400px
					if imageHeight > 400
					{
						return Err(CordialError::Configuration("RSS channel artwork can not be higher than 400px".to_owned()));
					}
					
					eventWriter.writeUnprefixedTextElementU32(&rssNamespace, &emptyAttributes, "width", imageWidth)?;
					
					eventWriter.writeUnprefixedTextElementU32(&rssNamespace, &emptyAttributes, "height", imageHeight)?;
					
					Ok(())
				})?;
				
				for rssItem in rssItems.iter()
				{
					rssItem.writeXml(eventWriter, &rssNamespace, &emptyAttributes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
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
	fn artwork_default() -> ResourceUrl
	{
		ResourceUrl::string("/organization-logo.png")
	}
	
	#[inline(always)]
	fn feedly_default() -> Option<FeedlyRssChannel>
	{
		Some(FeedlyRssChannel::default())
	}
}
