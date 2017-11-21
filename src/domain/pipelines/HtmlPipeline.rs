// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
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
	#[serde(default = "HtmlPipeline::rss_default")] rss: bool,
	#[serde(default)] rss_author: EMailAddress,
	#[serde(default)] rss_categories: BTreeSet<String>,
	// open graph, RSS, schema.org
	#[serde(default)] publication_date: Option<DateTime<Utc>>,
	// modification_date - used by open graph, schema.org. should be a list of changes, with changes detailed in all languages. Not the same as HTTP last-modified date.
	// empty modifications imply use of publication date
	#[serde(default)] modifications: BTreeMap<DateTime<Utc>, HashMap<Iso639Dash1Alpha2Language, String>>,
	// open graph
	#[serde(default)] expiration_date: Option<DateTime<Utc>>,
	#[serde(default)] abstracts: HashMap<Iso639Dash1Alpha2Language, Abstract>,
	// a resource URL; if missing, then rss should be set to false
	#[serde(default)] article_image: Option<ResourceUrl>,
	#[serde(default = "HtmlPipeline::template_default")] template: String,
	#[serde(default = "HtmlPipeline::amp_template_default")] amp_template: Option<String>,
	// Handlebars template default
	#[serde(default = "HtmlPipeline::header_id_prefix_with_trailing_dash_default")] header_id_prefix_with_trailing_dash: String,
	#[serde(default = "HtmlPipeline::pjax_css_selector_default")] pjax_css_selector: String,
	#[serde(default = "HtmlPipeline::rss_css_selector_default")] rss_css_selector: String,
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
			rss: Self::rss_default(),
			rss_author: Default::default(),
			rss_categories: Default::default(),
			publication_date: None,
			modifications: Default::default(),
			expiration_date: None,
			abstracts: Default::default(),
			article_image: None,
			template: Self::template_default(),
			amp_template: Self::amp_template_default(),
			header_id_prefix_with_trailing_dash: Self::header_id_prefix_with_trailing_dash_default(),
			pjax_css_selector: Self::pjax_css_selector_default(),
			rss_css_selector: Self::rss_css_selector_default(),
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
		(Self::is_versioned, Self::language_aware)
	}
	
	// TODO: How do we minify CSS across multiple HTML pages?
	// TODO: Take HTML and run it through languagetool
	// TODO: Validate length of title and description, content, etc
	// TODO: Sitemap videos?
	// TODO: More markdown plugins, external + internal (eg for a URL, SVG, IMAGE)
	// TODO: markdown plugins using `` syntax, markdown plugins using arguments from ```CODE``` syntax
	
	#[inline(always)]
	fn execute(&self, resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, siteMapWebPages: &mut Vec<SiteMapWebPage>, rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<ResourceTag, Rc<UrlDataDetails>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		let markdown = inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?;
		let markdownParser = MarkdownParser::defaultish(&self.header_id_prefix_with_trailing_dash);
		let pluginData = MarkdownPluginData
		{
			resources,
			configuration,
			language: languageData,
		};
		
		let abstract_ = self.abstract_(languageData.iso639Dash1Alpha2Language)?;
		let lastModificationDateOrPublicationDate = self.lastModificationDateOrPublicationDate();
		let articleImage = self.articleImage(resources)?;
		
		self.addSiteMapEntry(configuration, siteMapWebPages, resourceUrl, &articleImage, resources, languageData)?;
		
		let document = self.renderHandlebarsTemplateToHtml(&self.template, &markdown, &markdownParser, languageData, &articleImage, lastModificationDateOrPublicationDate, inputContentFilePath, configuration, handlebars, abstract_, &pluginData, false)?;
		let regularBody = document.minify_to_bytes(true);
		
		self.addRssItem(configuration, rssItems, resourceUrl, &articleImage, resources, lastModificationDateOrPublicationDate, &document, regularBody.len(), languageData, abstract_)?;
		
		const CanBeCompressed: bool = true;
		
		let mut result = Vec::with_capacity(4);
		
		let inputCanonicalUrl = self.canonicalUrl(languageData, resourceUrl)?;
		
		if let Some(ref amp_template) = self.amp_template
		{
			let ampUrl = self.ampUrl(languageData, resourceUrl)?;
			
			self.addRedirect(true, &mut result, languageData, resourceUrl, &inputCanonicalUrl)?;
			
			let ampHeaders = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::AMP, configuration, CanBeCompressed, self.max_age_in_seconds, Self::is_downloadable, &ampUrl)?;
			
			let ampDocument = self.renderHandlebarsTemplateToHtml(amp_template, &markdown, &markdownParser, languageData, &articleImage, lastModificationDateOrPublicationDate, inputContentFilePath, configuration, handlebars, abstract_, &pluginData, true)?;
			let ampBody = ampDocument.minify_to_bytes(false);
			
			result.push((ampUrl, hashmap! { amp => Rc::new(UrlDataDetails::Empty) }, StatusCode::Ok, ContentType::html(), ampHeaders, ampBody, None, CanBeCompressed));
		}
		
		{
			self.addRedirect(false, &mut result, languageData, resourceUrl, &inputCanonicalUrl)?;
			
			let regularHeaders = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanBeCompressed, self.max_age_in_seconds, Self::is_downloadable, &inputCanonicalUrl)?;
			
			let pjaxHeaders = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::PJAX, configuration, CanBeCompressed, self.max_age_in_seconds, Self::is_downloadable, &inputCanonicalUrl)?;
			let pjaxBody = Self::extractNodes(&self.pjax_css_selector, &document, "pjax_css_selector", regularBody.len())?;
			
			result.push((inputCanonicalUrl, hashmap! { default => Rc::new(UrlDataDetails::Empty) }, StatusCode::Ok, ContentType::html(), regularHeaders, regularBody, Some((pjaxHeaders, pjaxBody)), CanBeCompressed));
		}
		
		Ok(result)
	}
}

impl HtmlPipeline
{
	const is_versioned: bool = false;
	
	const language_aware: bool = true;
	
	const is_downloadable: bool = false;
	
	#[inline(always)]
	pub(crate) fn lastModificationDateOrPublicationDate(&self) -> Option<DateTime<Utc>>
	{
		match self.modifications.keys().rev().next()
		{
			Some(date) => Some(*date),
			None => self.publication_date
		}
	}
	
	#[inline(always)]
	pub(crate) fn modifications<'a>(&'a self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<BTreeMap<DateTime<Utc>, &'a str>, CordialError>
	{
		let mut modifications = BTreeMap::new();
		for (date, modificationTranslations) in self.modifications.iter()
		{
			let translation = match modificationTranslations.get(&iso639Dash1Alpha2Language)
			{
				None => return Err(CordialError::Configuration(format!("No modification translation for date {} for language '{}'", date, iso639Dash1Alpha2Language))),
				Some(translation) => translation.as_str(),
			};
			
			modifications.insert(*date, translation);
		}
		Ok(modifications)
	}
	
	#[inline(always)]
	pub(crate) fn expirationDate(&self) -> Option<DateTime<Utc>>
	{
		self.expiration_date
	}
	
	#[inline(always)]
	pub(crate) fn abstract_<'a>(&'a self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&'a Abstract, CordialError>
	{
		match self.abstracts.get(&iso639Dash1Alpha2Language)
		{
			None => Err(CordialError::Configuration(format!("No abstract translation for language '{}'", iso639Dash1Alpha2Language))),
			Some(abstract_) => Ok(abstract_),
		}
	}
	
	#[inline(always)]
	fn addRedirect(&self, isForAmp: bool, result: &mut Vec<(Url, HashMap<ResourceTag, Rc<UrlDataDetails>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, languageData: &LanguageData, resourceUrl: &ResourceUrl, inputCanonicalUrl: &Url) -> Result<(), CordialError>
	{
		const RedirectsCanNotBeCompressed: bool = false;
		
		if self.redirect_nearly_identical_url
		{
			let redirectUrl = if isForAmp
			{
				self.ampRedirectUrl(languageData, resourceUrl)?
			}
			else
			{
				self.redirectUrl(languageData, resourceUrl)?
			};
			result.push((redirectUrl, hashmap! { redirect => Rc::new(UrlDataDetails::Empty) }, StatusCode::MovedPermanently, ContentType::plaintext(), Self::redirectHeaders(inputCanonicalUrl), Vec::new(), None, RedirectsCanNotBeCompressed));
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn redirectHeaders(url: &Url) -> Vec<(String, String)>
	{
		let mut headers = Vec::new();
		
		headers.push(("Cache-Control".to_owned(), format!("{}", commonCacheControlHeader(31536000))));
		headers.push(("Location".to_owned(), url.as_str().to_owned()));
		
		headers.shrink_to_fit();
		headers
	}
	
	#[inline(always)]
	fn canonicalUrl(&self, languageData: &LanguageData, resourceUrl: &ResourceUrl) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			resourceUrl.leaf_url(languageData)
		}
		else
		{
			resourceUrl.url(languageData)
		}
	}
	
	#[inline(always)]
	fn redirectUrl(&self, languageData: &LanguageData, resourceUrl: &ResourceUrl) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			resourceUrl.url(languageData)
		}
		else
		{
			resourceUrl.leaf_url(languageData)
		}
	}
	
	#[inline(always)]
	fn ampUrl(&self, languageData: &LanguageData, resourceUrl: &ResourceUrl) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			resourceUrl.amp_leaf_url(languageData)
		}
		else
		{
			resourceUrl.amp_url(languageData)
		}
	}
	
	#[inline(always)]
	fn ampRedirectUrl(&self, languageData: &LanguageData, resourceUrl: &ResourceUrl) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			resourceUrl.amp_url(languageData)
		}
		else
		{
			resourceUrl.amp_leaf_url(languageData)
		}
	}
	
	#[inline(always)]
	fn renderHandlebarsTemplateToHtml(&self, template: &str, markdown: &str, markdownParser: &MarkdownParser, languageData: &LanguageData, articleImage: &Option<(ResourceReference, Ref<ImageMetaData>)>, lastModificationDateOrPublicationDate: Option<DateTime<Utc>>, inputContentFilePath: &Path, configuration: &Configuration, handlebars: &mut Handlebars, abstract_: &Abstract, pluginData: &MarkdownPluginData, isForAmp: bool) -> Result<RcDom, CordialError>
	{
		let htmlFromMarkdown = markdownParser.parse(&markdown, pluginData, isForAmp)?;
		
		let html =
		{
			let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
			let imageResourceReference = match articleImage
			{
				&None => None,
				&Some((ref imageResourceReference, _)) => Some(imageResourceReference),
			};
			let imageAbstract = match articleImage
			{
				&None => None,
				&Some((_, ref imageMetaData)) => Some((imageResourceReference,imageMetaData.abstract_(iso639Dash1Alpha2Language)?)),
			};
			
			let modifications = self.modifications(iso639Dash1Alpha2Language)?;
			handlebars.template_render(template, &json!
			({
				"environment": &configuration.environment,
				"our_language": languageData,
				"localization": &configuration.localization,
				"deployment_date": configuration.deploymentDate,
				"deployment_version": &configuration.deploymentVersion,

				"markdown": htmlFromMarkdown,
				"publication_date": self.publication_date,
				"lastModificationDateOrPublicationDate": lastModificationDateOrPublicationDate,
				"modifications": modifications,
				"expiration_date": self.expiration_date,
				"abstract": abstract_,
				"image_abstract": imageAbstract,
				"image_article": imageResourceReference,
				"site_map_images": self.site_map_images,
			}))?
		};
		let document = RcDom::from_bytes_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type(html.as_bytes(), inputContentFilePath)?;
		Ok(document)
	}
	
	#[inline(always)]
	fn addSiteMapEntry(&self, configuration: &Configuration, siteMapWebPages: &mut Vec<SiteMapWebPage>, resourceUrl: &ResourceUrl, articleImage: &Option<(ResourceReference, Ref<ImageMetaData>)>, resources: &Resources, languageData: &LanguageData) -> Result<(), CordialError>
	{
		if self.site_map
		{
			let primaryIso639Dash1Alpha2Language = configuration.primaryIso639Dash1Alpha2Language();
			let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
			
			let mut images = vec![];
			if let &Some((ref imageResourceUrl, ref articleImage)) = articleImage
			{
				images.push(articleImage.siteMapWebPageImage(imageResourceUrl, primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, resources)?);
			};
			
			for siteMapImageResourceUrl in self.site_map_images.iter()
			{
				if let Some(resourceRefCell) = siteMapImageResourceUrl.get(resources)
				{
					let resourceRef = resourceRefCell.try_borrow()?;
					let imageMetaData = resourceRef.imageMetaData()?;
					
					let internalImage = ResourceReference
					{
						resource: siteMapImageResourceUrl.clone(),
						tag: ResourceTag::largest_image,
					};
					images.push(imageMetaData.siteMapWebPageImage(&internalImage, primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, resources)?)
				}
			}
			
			let mut urlsByIso639Dash1Alpha2Language = BTreeMap::new();
			configuration.localization.visitLanguagesWithPrimaryFirst(|languageData, _isPrimaryLanguage|
			{
				let url = self.canonicalUrl(languageData, resourceUrl)?;
				urlsByIso639Dash1Alpha2Language.insert(languageData.iso639Dash1Alpha2Language, url);
				Ok(())
			})?;
			
			siteMapWebPages.push
			(
				SiteMapWebPage
				{
					lastModified: self.lastModificationDateOrPublicationDate(),
					changeFrequency: self.site_map_change_frequency,
					priority: self.site_map_priority,
					urlsByIso639Dash1Alpha2Language,
					images
				}
			);
		}
		Ok(())
	}
	
	#[inline(always)]
	fn addRssItem(&self, configuration: &Configuration, rssItems: &mut Vec<RssItem>, resourceUrl: &ResourceUrl, articleImage: &Option<(ResourceReference, Ref<ImageMetaData>)>, resources: &Resources, lastModificationDateOrPublicationDate: Option<DateTime<Utc>>, document: &RcDom, capacityHint: usize, languageData: &LanguageData, abstract_: &Abstract) -> Result<(), CordialError>
	{
		if self.rss
		{
			rssItems.push
			(
				RssItem
				{
					rssItemLanguageVariant: RssItemLanguageVariant
					{
						webPageDescription: abstract_.description.to_owned(),
						webPageUsefulContentHtml: Self::extractNodes(&self.rss_css_selector, &document, "rss_css_selector", capacityHint)?,
						languageSpecificUrl: self.canonicalUrl(languageData, resourceUrl)?,
						primaryImage: match articleImage
						{
							&None => None,
							&Some((ref imageResourceUrl, ref articleImage)) => Some(articleImage.rssImage(imageResourceUrl, configuration.primaryIso639Dash1Alpha2Language(), languageData.iso639Dash1Alpha2Language, resources)?)
						},
					},
					lastModificationDate: lastModificationDateOrPublicationDate,
					author: self.rss_author.clone(),
					categories: self.rss_categories.clone(),
				}
			);
		}
		Ok(())
	}
	
	#[inline(always)]
	fn extractNodes(selector: &str, document: &RcDom, selectorName: &str, capacityGuess: usize) -> Result<Vec<u8>, CordialError>
	{
		const html_head_and_body_tags_are_optional: bool = true;
		const PreserveComments: bool = false;
		const PreserveProcessingInstructions: bool = false;
		
		let mut writer = Vec::with_capacity(capacityGuess);
		{
			let mut serializer = UltraMinifyingHtmlSerializer::new(html_head_and_body_tags_are_optional, PreserveComments, PreserveProcessingInstructions, &mut writer);
			
			match parse_css_selector(selector)
			{
				Err(_) => return Err(CordialError::Configuration(format!("CSS {} {} was invalid", selectorName, selector))),
				Ok(selector) => document.find_all_matching_child_nodes_depth_first_excluding_this_one(&selector, &mut |node|
				{
					const collapse_whitespace: bool = true;
					const flush_when_serialized: bool = false;
					if serializer.serialize_node(node, collapse_whitespace, flush_when_serialized).is_err()
					{
						//return Err(CordialError::Configuration("Could not serialize node - is this even possible?".to_owned()));
					}
					false
				}),
			};
		}
		writer.shrink_to_fit();
		
		Ok(writer)
	}
	
	#[inline(always)]
	fn articleImage<'this, 'resources: 'this>(&'this self, resources: &'resources Resources) -> Result<Option<(ResourceReference, Ref<'resources, ImageMetaData>)>, CordialError>
	{
		fn x(resourceRefCell: &RefCell<Resource>) -> Result<Option<Ref<ImageMetaData>>, CordialError>
		{
			let resourceRef = resourceRefCell.try_borrow()?;
			let doneTwiceBecauseOfLimitationOnRefMapMethod = resourceRef.imageMetaData();
			if let Err(error) = doneTwiceBecauseOfLimitationOnRefMapMethod
			{
				return Err(error);
			}
			
			Ok(Some(Ref::map(resourceRef, |resource| resource.imageMetaData().unwrap())))
		}
		
		match self.article_image
		{
			None => Ok(None),
			Some(ref resourceUrl) =>
			{
				let resourceRefCell = match resourceUrl.get(resources)
				{
					None => return Ok(None),
					Some(resourceRefCell) => resourceRefCell,
				};
				
				let resourceReference = ResourceReference
				{
					resource: resourceUrl.clone(),
					tag: ResourceTag::largest_image,
				};
				
				Ok(x(resourceRefCell)?.map(|metadata| (resourceReference, metadata)))
			}
		}
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
	fn rss_default() -> bool
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
	fn rss_css_selector_default() -> String
	{
		"main".to_owned()
	}
	
	#[inline(always)]
	fn template_default() -> String
	{
		"article".to_owned()
	}
	
	#[inline(always)]
	fn amp_template_default() -> Option<String>
	{
		Some("amp-article".to_owned())
	}
}
