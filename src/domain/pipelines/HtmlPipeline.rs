// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct HtmlPipeline
{
	#[serde(default = "HtmlPipeline::max_age_in_seconds_none_default")] max_age_in_seconds: u32,
	#[serde(default)] input_format: HtmlInputFormat,
	#[serde(default)] is_leaf: bool,
	#[serde(default = "HtmlPipeline::redirect_nearly_identical_url_default")] redirect_nearly_identical_url: bool,
	#[serde(default = "HtmlPipeline::site_map_default")] site_map: bool,
	#[serde(default)] site_map_change_frequency: SiteMapChangeFrequency,
	#[serde(default)] site_map_priority: SiteMapPriority,
	#[serde(default)] site_map_images: Vec<ResourceUrl>,
	#[serde(default)] site_map_audios: Vec<ResourceUrl>,
	#[serde(default)] site_map_videos: Vec<ResourceUrl>,
	#[serde(default)] rss: Option<Rc<HtmlDocumentItem>>,
	#[serde(default, deserialize_with = "HtmlPipeline::rss_channels_deserialize_with")] rss_channels: OrderMap<Rc<RssChannelName>, ()>,
	#[serde(default)] author: Option<ResourceUrl>,
	// open graph, RSS, schema.org
	#[serde(default)] publication_date: Option<DateTime<Utc>>,
	// modification_date - used by open graph, schema.org. should be a list of changes, with changes detailed in all languages. Not the same as HTTP last-modified date.
	// empty modifications imply use of publication date
	#[serde(default)] modifications: BTreeMap<DateTime<Utc>, HashMap<Iso639Dash1Alpha2Language, Rc<String>>>,
	// open graph
	#[serde(default)] expiration_date: Option<DateTime<Utc>>,
	#[serde(default)] abstracts: HashMap<Iso639Dash1Alpha2Language, Rc<HtmlAbstract>>,
	// a resource URL; if missing, then rss should be set to false
	#[serde(default)] article_image: Option<ResourceUrl>,
	#[serde(default)] output_format: HtmlOutputFormat,
	#[serde(default = "HtmlPipeline::header_id_prefix_with_trailing_dash_default")] header_id_prefix_with_trailing_dash: String,
	#[serde(default = "HtmlPipeline::pjax_css_selector_default")] pjax_css_selector: String,
	#[serde(default)] previous: Option<ResourceUrl>,
	#[serde(default)] next: Option<ResourceUrl>,
	#[serde(default)] help: Option<ResourceUrl>,
	#[serde(default)] license: Option<ResourceUrl>,
	#[serde(default)] manifest: Option<ResourceUrl>,
	#[serde(default)] open_graph: Rc<FacebookOpenGraph>,
	#[serde(default)] twitter_card: Rc<TwitterCard>,
	#[serde(default)] theme_css_color: Option<Rc<String>>,
	#[serde(default)] automatic_telephone_number_detection: bool, // Safari and Edge / IE 11 on mobile, sort-of
	#[serde(default)] favicon: Option<FavIcon>,
	#[serde(default)] svg_favicon: Option<ResourceUrl>,
	#[serde(default)] safari_styling: Option<SafariStyling>,
	#[serde(default)] windows_tiles_browser_config: Option<ResourceUrl>,
}

impl Default for HtmlPipeline
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			max_age_in_seconds: Self::max_age_in_seconds_none_default(),
			input_format: Default::default(),
			is_leaf: false,
			redirect_nearly_identical_url: Self::redirect_nearly_identical_url_default(),
			site_map: Self::site_map_default(),
			site_map_change_frequency: Default::default(),
			site_map_priority: Default::default(),
			site_map_images: Default::default(),
			site_map_audios: Default::default(),
			site_map_videos: Default::default(),
			rss: None,
			rss_channels: Default::default(),
			author: None,
			publication_date: None,
			modifications: Default::default(),
			expiration_date: None,
			abstracts: Default::default(),
			article_image: None,
			output_format: Default::default(),
			header_id_prefix_with_trailing_dash: Self::header_id_prefix_with_trailing_dash_default(),
			pjax_css_selector: Self::pjax_css_selector_default(),
			previous: None,
			next: None,
			help: None,
			license: None,
			manifest: None,
			open_graph: Default::default(),
			twitter_card: Default::default(),
			theme_css_color: None,
			automatic_telephone_number_detection: false,
			favicon: None,
			svg_favicon: None,
			safari_styling: None,
			windows_tiles_browser_config: None,
		}
	}
}

impl Pipeline for HtmlPipeline
{
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority
	{
		LinksToSubResourcesEgHtmlPage
	}
	
	#[inline(always)]
	fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		self.input_format.resourceInputContentFileNamesWithExtension(resourceInputName)
	}
	
	#[inline(always)]
	fn is<'a>(&self) -> (bool, bool)
	{
		const IsNotVersioned: bool = false;
		const IsLanguageAware: bool = true;
		
		(IsNotVersioned, IsLanguageAware)
	}
	
	#[inline(always)]
	fn anchorTitleAttribute(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<Option<Rc<String>>, CordialError>
	{
		match self.abstracts.get(&iso639Dash1Alpha2Language)
		{
			Some(htmlAbstract) => Ok(Some(htmlAbstract.description.clone())),
			None => match self.abstracts.get(&fallbackIso639Dash1Alpha2Language)
			{
				Some(htmlAbstract) => Ok(Some(htmlAbstract.description.clone())),
				None => Err(CordialError::Configuration("No HTML abstract".to_owned())),
			}
		}
	}
	
	#[inline(always)]
	fn execute(&self, resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResponse>, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		let htmlAbstract = match self.abstracts.get(&iso639Dash1Alpha2Language)
		{
			None => return Err(CordialError::Configuration(format!("No abstract translation for language '{}'", iso639Dash1Alpha2Language))),
			Some(htmlAbstract) => htmlAbstract.clone(),
		};
		let htmlDocumentData = HtmlDocumentData
		{
			resourceUrl,
			fallbackIso639Dash1Alpha2Language: configuration.fallbackIso639Dash1Alpha2Language(),
			
			markdownParser: MarkdownParser::defaultish(self.header_id_prefix_with_trailing_dash.as_str()),
			markdown: inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?,
			markdownPluginData: MarkdownPluginData
			{
				resources,
				configuration,
				languageData,
			},
			htmlAbstract: htmlAbstract.clone(),
			articleImage: match self.article_image
			{
				None => None,
				Some(ref resourceUrl) =>
				{
					let resourceRef = resourceUrl.resourceMandatory(resources)?;
					Some
					(
						(
							resourceUrl.clone(),
							resourceRef.imageMetaData()?.clone(),
						)
					)
				}
			},
			siteMapImages: &self.site_map_images,
			siteMapAudios: &self.site_map_audios,
			siteMapVideos: &self.site_map_videos,
			publicationDate: self.publication_date,
			lastModificationDateOrPublicationDate: match self.modifications.keys().rev().next()
			{
				Some(date) => Some(*date),
				None => self.publication_date
			},
			modifications: self.modifications(iso639Dash1Alpha2Language)?,
			expirationDate: self.expiration_date,
			configuration,
			htmlUrls: HtmlUrls
			{
				is_leaf: self.is_leaf,
				redirect_nearly_identical_url: self.redirect_nearly_identical_url,
				resourceUrl: resourceUrl.clone(),
				languageData,
				localization: &configuration.localization,
				resources,
				rssChannelNames: &self.rss_channels,
				canonicalShortlink: htmlAbstract.shortlink.clone(),
				pingback: htmlAbstract.pingback.clone(),
				previous: self.previous.clone(),
				next: self.next.clone(),
				help: self.help.clone(),
				author: self.author.clone(),
				license: self.license.clone(),
				manifest: self.manifest.clone(),
			},
			facebookOpenGraph: self.open_graph.clone(),
			twitterCard: self.twitter_card.clone(),
			themeCssColor: self.theme_css_color.clone(),
			automaticTelephoneNumberDetection: self.automatic_telephone_number_detection,
			favIcon: self.favicon.as_ref(),
			svgFavIcon: self.svg_favicon.as_ref(),
			safariStyling: self.safari_styling.as_ref(),
			windowsTilesBrowserConfig: self.windows_tiles_browser_config.as_ref(),
		};
		
		htmlDocumentData.addToRssChannels(resources, rssChannelsToRssItems, &self.rss, &self.rss_channels, inputContentFilePath, handlebars)?;
		if self.site_map
		{
			htmlDocumentData.addToSiteMaps(resources, siteMapWebPages, self.site_map_change_frequency, self.site_map_priority)?;
		}
		self.output_format.renderHtmlDocumentsAndRedirects(resources, &htmlDocumentData, headerGenerator, self.max_age_in_seconds, inputContentFilePath, handlebars)
	}
}

impl HtmlPipeline
{
	#[inline(always)]
	pub(crate) fn hasFacebookOpenGraphTypeDiscriminant(&self, facebookOpenGraphTypeDiscriminant: FacebookOpenGraphTypeDiscriminant) -> bool
	{
		self.open_graph.hasFacebookOpenGraphTypeDiscriminant(facebookOpenGraphTypeDiscriminant)
	}
	
	#[inline(always)]
	fn modifications(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<BTreeMap<DateTime<Utc>, Rc<String>>, CordialError>
	{
		let mut modifications = BTreeMap::new();
		for (date, modificationTranslations) in self.modifications.iter()
		{
			let translation = match modificationTranslations.get(&iso639Dash1Alpha2Language)
			{
				None => return Err(CordialError::Configuration(format!("No modification translation for date {} for language '{}'", date, iso639Dash1Alpha2Language))),
				Some(translation) => translation.clone(),
			};
			
			modifications.insert(*date, translation);
		}
		Ok(modifications)
	}
	
	#[inline(always)]
	fn max_age_in_seconds_none_default() -> u32
	{
		0
	}
	
	#[inline(always)]
	fn redirect_nearly_identical_url_default() -> bool
	{
		true
	}
	
	#[inline(always)]
	fn site_map_default() -> bool
	{
		true
	}
	
	#[inline(always)]
	fn header_id_prefix_with_trailing_dash_default() -> String
	{
		"header-".to_owned()
	}
	
	#[inline(always)]
	fn pjax_css_selector_default() -> String
	{
		"main".to_owned()
	}
	
	#[inline(always)]
	fn rss_channels_deserialize_with<'de, D: Deserializer<'de>>(deserializer: D) -> Result<OrderMap<Rc<RssChannelName>, ()>, D::Error>
	{
		struct RssChannelsVisitor;
		
		impl<'de> Visitor<'de> for RssChannelsVisitor
		{
			type Value = OrderMap<Rc<RssChannelName>, ()>;
			
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("an ordered unique list of RSS channel names")
			}
			
			fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error>
			{
				let mut rssChannels = if let Some(capacity) = seq.size_hint()
				{
					OrderMap::with_capacity(capacity)
				}
				else
				{
					OrderMap::new()
				};
				
				while let Some(rssChannelName) = seq.next_element()?
				{
					if !rssChannels.contains_key(&rssChannelName)
					{
						rssChannels.insert(rssChannelName, ());
					}
				}
				
				Ok(rssChannels)
			}
		}
		
		deserializer.deserialize_seq(RssChannelsVisitor)
	}
}
