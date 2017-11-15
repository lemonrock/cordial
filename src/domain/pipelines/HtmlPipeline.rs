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
	#[serde(default = "HtmlPipeline::rss_default")] rss: bool,
	#[serde(default)] rss_author: EMailAddress,
	#[serde(default)] rss_categories: BTreeSet<String>,
	// open graph, RSS, schema.org
	#[serde(default)] publication_date: Option<DateTime<Utc>>,
	// modification_date - used by open graph, schema.org. should be a list of changes, with changes detailed in all languages. Not the same as HTTP last-modified date.
	// empty modifications imply use of publication date
	#[serde(default)] modifications: BTreeMap<DateTime<Utc>, HashMap<String, String>>,
	// open graph
	#[serde(default)] expiration_date: Option<DateTime<Utc>>,
	#[serde(default)] abstracts: HashMap<String, Abstract>,
	// a resource URL; if missing, then rss should be set to false
	#[serde(default)] article_image: Option<String>,
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
	
	// TODO: How do we minify CSS across mutliple HTML pages?
	// TODO: Take HTML and generate AMP page
	// TODO: Take HTML and run it through languagetool
	// TODO: Validate length of title and description, content, etc
	// TODO: Add images within web page to site map
	// TODO: JSON/handlebars: deployment_version
	// TODO: JSON/handlebars: Article Image
	
	#[inline(always)]
	fn execute(&self, resources: &Resources, inputContentFilePath: &Path, resourceRelativeUrl: &str, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, siteMapWebPages: &mut Vec<SiteMapWebPage>, rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		let htmlFromMarkdown = self.renderMarkdown(inputContentFilePath)?;
		let abstract_ = self.abstract_(languageData)?;
		let lastModificationDateOrPublicationDate = self.lastModificationDateOrPublicationDate();
		let articleImage = self.articleImage(resources)?;
		
		self.addSiteMapEntry(configuration, siteMapWebPages, resourceRelativeUrl, articleImage, resources, languageData);
		
		let document = self.renderHandlebarsTemplateToHtml(&self.template, &htmlFromMarkdown, languageData, articleImage, lastModificationDateOrPublicationDate, inputContentFilePath, configuration, handlebars, abstract_)?;
		let regularBody = document.minify_to_bytes(true);
		
		self.addRssItem(configuration, rssItems, resourceRelativeUrl, articleImage, resources, lastModificationDateOrPublicationDate, &document, regularBody.len(), languageData, abstract_)?;
		
		const CanBeCompressed: bool = true;
		
		let mut result = Vec::with_capacity(4);
		
		let inputCanonicalUrl = self.canonicalUrl(languageData, resourceRelativeUrl)?;
		
		if let Some(ref amp_template) = self.amp_template
		{
			let ampUrl = self.ampUrl(languageData, resourceRelativeUrl)?;
			
			self.addRedirect(true, &mut result, languageData, resourceRelativeUrl, &inputCanonicalUrl);
			
			let ampHeaders = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::AMP, configuration, CanBeCompressed, self.max_age_in_seconds, Self::is_downloadable, &ampUrl)?;
			
			let ampDocument = self.renderHandlebarsTemplateToHtml(amp_template, &htmlFromMarkdown, languageData, articleImage, lastModificationDateOrPublicationDate, inputContentFilePath, configuration, handlebars, abstract_)?;
			let ampBody = ampDocument.minify_to_bytes(false);
			
			result.push((ampUrl, hashmap! { amp => Rc::new(JsonValue::Null) }, StatusCode::Ok, ContentType::html(), ampHeaders, ampBody, None, CanBeCompressed));
		}
		
		{
			self.addRedirect(false, &mut result, languageData, resourceRelativeUrl, &inputCanonicalUrl);
			
			let regularHeaders = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanBeCompressed, self.max_age_in_seconds, Self::is_downloadable, &inputCanonicalUrl)?;
			
			let pjaxHeaders = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::PJAX, configuration, CanBeCompressed, self.max_age_in_seconds, Self::is_downloadable, &inputCanonicalUrl)?;
			let pjaxBody = Self::extractNodes(&self.pjax_css_selector, &document, "pjax_css_selector", regularBody.len())?;
			
			result.push((inputCanonicalUrl, hashmap! { default => Rc::new(JsonValue::Null) }, StatusCode::Ok, ContentType::html(), regularHeaders, regularBody, Some((pjaxHeaders, pjaxBody)), CanBeCompressed));
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
	fn addRedirect(&self, isForAmp: bool, result: &mut Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, languageData: &LanguageData, resourceRelativeUrl: &str, inputCanonicalUrl: &Url) -> Result<(), CordialError>
	{
		const RedirectsCanNotBeCompressed: bool = false;
		
		if self.redirect_nearly_identical_url
		{
			let redirectUrl = if isForAmp
			{
				self.ampRedirectUrl(languageData, resourceRelativeUrl)?
			}
			else
			{
				self.redirectUrl(languageData, resourceRelativeUrl)?
			};
			result.push((redirectUrl, hashmap! { redirect => Rc::new(JsonValue::Null) }, StatusCode::MovedPermanently, ContentType::plaintext(), Self::redirectHeaders(inputCanonicalUrl), Vec::new(), None, RedirectsCanNotBeCompressed));
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
	fn canonicalUrl(&self, languageData: &LanguageData, resourceRelativeUrl: &str) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			languageData.leaf_url(resourceRelativeUrl)
		}
		else
		{
			languageData.url(resourceRelativeUrl)
		}
	}
	
	#[inline(always)]
	fn redirectUrl(&self, languageData: &LanguageData, resourceRelativeUrl: &str) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			languageData.url(resourceRelativeUrl)
		}
		else
		{
			languageData.leaf_url(resourceRelativeUrl)
		}
	}
	
	#[inline(always)]
	fn ampUrl(&self, languageData: &LanguageData, resourceRelativeUrl: &str) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			languageData.amp_leaf_url(resourceRelativeUrl)
		}
		else
		{
			languageData.amp_url(resourceRelativeUrl)
		}
	}
	
	#[inline(always)]
	fn ampRedirectUrl(&self, languageData: &LanguageData, resourceRelativeUrl: &str) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			languageData.amp_url(resourceRelativeUrl)
		}
		else
		{
			languageData.amp_leaf_url(resourceRelativeUrl)
		}
	}
	
	#[inline(always)]
	fn addSiteMapEntry(&self, configuration: &Configuration, siteMapWebPages: &mut Vec<SiteMapWebPage>, resourceRelativeUrl: &str, articleImage: Option<(&str, &ImageMetaData)>, resources: &Resources, languageData: &LanguageData) -> Result<(), CordialError>
	{
		if self.site_map
		{
			let mut images = vec![];
			if let Some((imageResourceUrl, articleImage)) = articleImage
			{
				images.push(articleImage.siteMapWebPageImage(imageResourceUrl, configuration.primary_iso_639_1_alpha_2_language_code(), languageData.iso_639_1_alpha_2_language_code, resources)?);
			};
			
			let mut urlsByIsoLanguageCode = BTreeMap::new();
			configuration.localization.visitLanguagesWithPrimaryFirst(|languageData, _isPrimaryLanguage|
			{
				let url = self.canonicalUrl(languageData, resourceRelativeUrl)?;
				urlsByIsoLanguageCode.insert(languageData.iso_639_1_alpha_2_language_code.to_owned(), url);
				Ok(())
			});
			
			siteMapWebPages.push
			(
				SiteMapWebPage
				{
					lastModified: self.lastModificationDateOrPublicationDate(),
					changeFrequency: self.site_map_change_frequency,
					priority: self.site_map_priority,
					urlsByIsoLanguageCode,
					images
				}
			);
		}
		Ok(())
	}
	
	#[inline(always)]
	fn addRssItem(&self, configuration: &Configuration, rssItems: &mut Vec<RssItem>, resourceRelativeUrl: &str, articleImage: Option<(&str, &ImageMetaData)>, resources: &Resources, lastModificationDateOrPublicationDate: Option<DateTime<Utc>>, document: &RcDom, capacityHint: usize, languageData: &LanguageData, abstract_: &Abstract) -> Result<(), CordialError>
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
						languageSpecificUrl: self.canonicalUrl(languageData, resourceRelativeUrl)?,
						primaryImage: match articleImage
						{
							None => None,
							Some((imageResourceUrl, articleImage)) => Some(articleImage.rssImage(imageResourceUrl, configuration.primary_iso_639_1_alpha_2_language_code(), languageData.iso_639_1_alpha_2_language_code, resources)?)
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
	fn lastModificationDateOrPublicationDate(&self) -> Option<DateTime<Utc>>
	{
		match self.modifications.keys().rev().next()
		{
			Some(date) => Some(*date),
			None => self.publication_date
		}
	}
	
	#[inline(always)]
	fn modifications(&self, iso_639_1_alpha_2_language_code: &str) -> Result<BTreeMap<DateTime<Utc>, &str>, CordialError>
	{
		let mut modifications = BTreeMap::new();
		for (date, modificationTranslations) in self.modifications.iter()
		{
			let translation = match modificationTranslations.get(iso_639_1_alpha_2_language_code)
			{
				None => return Err(CordialError::Configuration(format!("No modification translation for date {} for language '{}'", date, iso_639_1_alpha_2_language_code))),
				Some(translation) => translation.as_str(),
			};
			
			modifications.insert(*date, translation);
		}
		Ok(modifications)
	}
	
	#[inline(always)]
	fn abstract_(&self, languageData: &LanguageData) -> Result<&Abstract, CordialError>
	{
		let iso_639_1_alpha_2_language_code = languageData.iso_639_1_alpha_2_language_code;
		match self.abstracts.get(iso_639_1_alpha_2_language_code)
		{
			None => Err(CordialError::Configuration(format!("No abstract translation for language '{}'", iso_639_1_alpha_2_language_code))),
			Some(abstract_) => Ok(abstract_),
		}
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
	fn articleImage<'a>(&'a self, resources: &'a Resources) -> Result<Option<(&'a str, &'a ImageMetaData)>, CordialError>
	{
		if let Some(ref article_image) = self.article_image
		{
			Ok(ImageMetaData::find(article_image, resources)?.map(|metadata| (article_image.as_str(), metadata)))
		}
		else
		{
			Ok(None)
		}
	}
	
	#[inline(always)]
	fn renderMarkdown(&self, inputContentFilePath: &Path) -> Result<Vec<u8>, CordialError>
	{
		let markdown = inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?;
		MarkdownParser::defaultishParse(&self.header_id_prefix_with_trailing_dash, &markdown)
	}
	
	#[inline(always)]
	fn renderHandlebarsTemplateToHtml(&self, template: &str, htmlFromMarkdown: &[u8], languageData: &LanguageData, articleImage: Option<(&str, &ImageMetaData)>, lastModificationDateOrPublicationDate: Option<DateTime<Utc>>, inputContentFilePath: &Path, configuration: &Configuration, handlebars: &mut Handlebars, abstract_: &Abstract) -> Result<RcDom, CordialError>
	{
		let html =
		{
			let iso_639_1_alpha_2_language_code = languageData.iso_639_1_alpha_2_language_code;
			let imageAbstract = match articleImage
			{
				None => None,
				Some((_, imageMetaData)) => Some(imageMetaData.abstract_(iso_639_1_alpha_2_language_code)?),
			};
			handlebars.template_render(template, &json!
			({
				"environment": &configuration.environment,
				"our_language": languageData,
				"localization": &configuration.localization,
				"deployment_date": configuration.deploymentDate,
				//"deployment_version": deploymentVersion,
				
				"markdown": htmlFromMarkdown,
				"publication_date": self.publication_date,
				"lastModificationDateOrPublicationDate": lastModificationDateOrPublicationDate,
				"modifications": self.modifications(iso_639_1_alpha_2_language_code)?,
				"expiration_date": self.expiration_date,
				"abstract": abstract_,
				"image_abstract": imageAbstract,
				// TODO: Article Image
			}))?
		};
		let document = RcDom::from_bytes_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type(html.as_bytes(), inputContentFilePath)?;
		Ok(document)
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
