// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Resource
{
	#[serde(default)] pipeline: ResourcePipeline,
	#[serde(default)] css: CssPipeline,
	#[serde(default)] font: FontPipeline,
	#[serde(default)] gif_animation: GifAnimationPipeline,
	#[serde(default)] html: HtmlPipeline,
	#[serde(default)] raster_image: RasterImagePipeline,
	#[serde(default)] raw: RawPipeline,
	#[serde(default)] svg: SvgPipeline,
	#[serde(default)] headers: HashMap<String, String>,
	#[serde(default)] compression: Compression,
	#[serde(default)] is_data_uri: bool,
	#[serde(default, skip_deserializing)] canonicalParentFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] resourceInputName: String,
	#[serde(default, skip_deserializing)] resourceInputContentFileNamesWithExtension: Vec<String>,
	#[serde(default, skip_deserializing)] resourceRelativeUrl: String,
	#[serde(default, skip_deserializing)] urls_by_iso_639_1_alpha_2_language_code: HashMap<String, HashMap<UrlTag, Rc<UrlData>>>,
}

impl Resource
{
	#[inline(always)]
	pub(crate) fn resourceRelativeUrl(&self) -> &str
	{
		&self.resourceRelativeUrl
	}
	
	#[inline(always)]
	pub(crate) fn hasProcessingPriority(&self, processingPriority: ProcessingPriority) -> bool
	{
		self.processingPriority() == processingPriority
	}
	
	#[inline(always)]
	pub(crate) fn urlData(&self, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, urlTag: &UrlTag) -> Option<Rc<UrlData>>
	{
		let urlKey = self.urlKey(primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code);
		match self.urls_by_iso_639_1_alpha_2_language_code.get(urlKey)
		{
			None => None,
			Some(urlTags) => urlTags.get(urlTag).map(|urlData| urlData.clone())
		}
	}
	
	#[inline(always)]
	fn urlKey<'a>(&self, primary_iso_639_1_alpha_2_language_code: &'a str, iso_639_1_alpha_2_language_code: Option<&'a str>) -> &'a str
	{
		let (isForPrimaryLanguageOnly, _isVersioned) = self.is();
		if isForPrimaryLanguageOnly
		{
			primary_iso_639_1_alpha_2_language_code
		}
		else if let Some(iso_639_1_alpha_2_language_code) = iso_639_1_alpha_2_language_code
		{
			if self.urls_by_iso_639_1_alpha_2_language_code.contains_key(iso_639_1_alpha_2_language_code)
			{
				iso_639_1_alpha_2_language_code
			}
			else
			{
				primary_iso_639_1_alpha_2_language_code
			}
		}
		else
		{
			primary_iso_639_1_alpha_2_language_code
		}
	}
	
	#[inline(always)]
	pub(crate) fn name(&self) -> PathBuf
	{
		self.canonicalParentFolderPath.join(format!("{}.resource.hjson", self.resourceInputName))
	}
	
	#[inline(always)]
	pub(crate) fn finishInitialization(&mut self, parentHierarchy: Vec<String>, resourceInputName: &str, canonicalParentFolderPath: PathBuf)
	{
		#[inline(always)]
		fn resourceRelativeUrl(parentHierarchy: &[String], resourceInputName: &str) -> String
		{
			let mut resourceRelativeUrl = String::with_capacity(1024);
			for parent in parentHierarchy
			{
				resourceRelativeUrl.push_str(parent);
				resourceRelativeUrl.push('/');
			}
			
			resourceRelativeUrl.push_str(resourceInputName);
			
			resourceRelativeUrl
		}
		
		self.canonicalParentFolderPath = canonicalParentFolderPath;
		self.resourceInputName = resourceInputName.to_owned();
		self.resourceInputContentFileNamesWithExtension = self.resourceInputContentFileNamesWithExtension(resourceInputName);
		self.resourceRelativeUrl = resourceRelativeUrl(&parentHierarchy, resourceInputName);
	}
	
	#[inline(always)]
	pub(crate) fn render(&mut self, resources: &Resources, newResponses: &mut Responses, oldResponses: &Arc<Responses>, configuration: &Configuration, handlebars: &mut Handlebars, siteMapWebPagesByLanguage: &mut HashMap<String, Vec<SiteMapWebPage>>, rssItemsByLanguage: &mut HashMap<String, Vec<RssItem>>) -> Result<(), CordialError>
	{
		#[inline(always)]
		fn getOrDefault<'a, T>(map: &'a mut HashMap<String, Vec<T>>, iso_639_1_alpha_2_language_code: &str) -> &'a mut Vec<T>
		{
			map.entry(iso_639_1_alpha_2_language_code.to_owned()).or_insert_with(|| Vec::with_capacity(4096))
		}
		
		let primaryLanguage = configuration.localization.primaryLanguage()?;
		
		configuration.visitLanguagesWithPrimaryFirst(|languageData, isPrimaryLanguage|
		{
			let (isVersioned, isLanguageAware) = self.is();
			
			let isLanguageAgnosticSoProcessOnlyForPrimaryLanguage = !isLanguageAware;
			
			if isLanguageAgnosticSoProcessOnlyForPrimaryLanguage && !isPrimaryLanguage
			{
			}
			else
			{
				let iso_639_1_alpha_2_language_code = languageData.iso_639_1_alpha_2_language_code;
				
				let result =
				{
					let (ifLanguageAwareLanguageData, inputContentFilePath) = if isLanguageAware
					{
						(Some(languageData), self.inputContentFilePath(primaryLanguage, Some(languageData.language))?)
					}
					else
					{
						(None, self.languageNeutralInputContentFilePath(primaryLanguage, None)?)
					};
					let mut siteMapWebPages = getOrDefault(siteMapWebPagesByLanguage, iso_639_1_alpha_2_language_code);
					let mut rssItems = getOrDefault(rssItemsByLanguage, iso_639_1_alpha_2_language_code);
					
					self.execute(resources, &inputContentFilePath, &self.resourceRelativeUrl, handlebars, &self.headers, languageData, ifLanguageAwareLanguageData, configuration, &mut siteMapWebPages, &mut rssItems)?
				};
				
				// Always inserts, as this language code will only occur once.
				let urls = self.urls_by_iso_639_1_alpha_2_language_code.entry(iso_639_1_alpha_2_language_code.to_owned()).or_insert(HashMap::with_capacity(result.len()));
				
				for (mut url, mut urlTagsWithJsonValues, statusCode, contentType, regularHeaders, regularBody, pjax, canBeCompressed) in result
				{
					let hasPjax = pjax.is_some();
					
					let regularCompressed = if canBeCompressed
					{
						Some(self.compression.compress(&regularBody)?)
					}
					else
					{
						None
					};
					
					let newResponse = if hasPjax
					{
						let (pjaxHeaders, pjaxBody) = pjax.unwrap();
						let pjaxCompressed = if canBeCompressed
						{
							Some(self.compression.compress(&pjaxBody)?)
						}
						else
						{
							None
						};
						
						RegularAndPjaxStaticResponse::both(StaticResponse::new(statusCode, contentType.clone(), regularHeaders, regularBody, regularCompressed), Some(StaticResponse::new(StatusCode::Ok, contentType, pjaxHeaders, pjaxBody, pjaxCompressed)))
					}
					else
					{
						if statusCode == StatusCode::Ok
						{
							RegularAndPjaxStaticResponse::regular(StaticResponse::new(statusCode, contentType, regularHeaders, regularBody, regularCompressed))
						}
						else
						{
							RegularAndPjaxStaticResponse::unadorned(StaticResponse::new(statusCode, contentType, regularHeaders, regularBody, None))
						}
					};
					
					debug_assert!(!urlTagsWithJsonValues.is_empty(), "urlTags is empty");
					let (urlOrDataUri, dataUriResponse) = if self.is_data_uri
					{
						(newResponse.toDataUri(), Some(Rc::new(newResponse)))
					}
					else
					{
						if isVersioned
						{
							url.set_query(Some(&format!("v={}", newResponse.entityTag())));
						}
						
						newResponses.addResponse(url.clone(), newResponse, oldResponses.clone());
						(url, None)
					};
					
					let urlOrDataUri = Rc::new(urlOrDataUri);
					for (urlTag, jsonValue) in urlTagsWithJsonValues.drain()
					{
						urls.insert(urlTag, Rc::new(UrlData
						{
							urlOrDataUri: urlOrDataUri.clone(),
							jsonValue,
							dataUriResponse: dataUriResponse.clone(),
						}));
					}
				}
			}
			
			Ok(())
		})?;
		Ok(())
	}
	
	/// if language is some, then searches for resource by language, primary language or language-neutral name in descending order
	/// if language is none, the searches by language-neutral name
	#[inline(always)]
	fn inputContentFilePath(&self, primaryLanguage: &Language, language: Option<&Language>) -> Result<PathBuf, CordialError>
	{
		if language.is_some()
		{
			let nonPrimaryLanguage = language.unwrap();
			
			for resourceInputContentFileNameWithExtension in self.resourceInputContentFileNamesWithExtension.iter()
			{
				let languageSpecificFilePath = self.canonicalParentFolderPath.join(format!("{}.{}", nonPrimaryLanguage.iso_3166_1_alpha_2_country_code(), resourceInputContentFileNameWithExtension));
				if languageSpecificFilePath.exists()
				{
					return Ok(languageSpecificFilePath);
				}
			}
			
			if primaryLanguage != nonPrimaryLanguage
			{
				for resourceInputContentFileNameWithExtension in self.resourceInputContentFileNamesWithExtension.iter()
				{
					let primaryLanguageSpecificFilePath = self.canonicalParentFolderPath.join(format!("{}.{}", primaryLanguage.iso_3166_1_alpha_2_country_code(), resourceInputContentFileNameWithExtension));
					if primaryLanguageSpecificFilePath.exists()
					{
						return Ok(primaryLanguageSpecificFilePath);
					}
				}
			}
		}
		
		return self.languageNeutralInputContentFilePath(primaryLanguage, language)
	}
	
	#[inline(always)]
	fn languageNeutralInputContentFilePath(&self, primaryLanguage: &Language, language: Option<&Language>) -> Result<PathBuf, CordialError>
	{
		for resourceInputContentFileNameWithExtension in self.resourceInputContentFileNamesWithExtension.iter()
		{
			let languageNeutralFilePath = self.canonicalParentFolderPath.join(resourceInputContentFileNameWithExtension);
			if languageNeutralFilePath.exists()
			{
				return Ok(languageNeutralFilePath);
			}
		}
		
		CordialError::couldNotFindResourceContentFile(self, primaryLanguage, language)
	}
	
	#[inline(always)]
	pub(crate) fn imageMetaData<'a>(&'a self) -> Option<&'a ImageMetaData>
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			css => self.css.imageMetaData(),
			font => self.font.imageMetaData(),
			gif_animation => self.gif_animation.imageMetaData(),
			html => self.html.imageMetaData(),
			raster_image => self.raster_image.imageMetaData(),
			raw => self.raw.imageMetaData(),
			svg => self.svg.imageMetaData(),
		}
	}
	
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			css => self.css.processingPriority(),
			font => self.font.processingPriority(),
			gif_animation => self.gif_animation.processingPriority(),
			html => self.html.processingPriority(),
			raster_image => self.raster_image.processingPriority(),
			raw => self.raw.processingPriority(),
			svg => self.svg.processingPriority(),
		}
	}
	
	#[inline(always)]
	fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			css => self.css.resourceInputContentFileNamesWithExtension(resourceInputName),
			font => self.font.resourceInputContentFileNamesWithExtension(resourceInputName),
			gif_animation => self.gif_animation.resourceInputContentFileNamesWithExtension(resourceInputName),
			html => self.html.resourceInputContentFileNamesWithExtension(resourceInputName),
			raster_image => self.raster_image.resourceInputContentFileNamesWithExtension(resourceInputName),
			raw => self.raw.resourceInputContentFileNamesWithExtension(resourceInputName),
			svg => self.svg.resourceInputContentFileNamesWithExtension(resourceInputName),
		}
	}
	
	#[inline(always)]
	fn is<'a>(&self) -> (bool, bool)
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			css => self.css.is(),
			font => self.font.is(),
			gif_animation => self.gif_animation.is(),
			html => self.html.is(),
			raster_image => self.raster_image.is(),
			raw => self.raw.is(),
			svg => self.svg.is(),
		}
	}
	
	#[inline(always)]
	fn execute(&self, resources: &Resources, inputContentFilePath: &Path, resourceRelativeUrl: &str, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, siteMapWebPages: &mut Vec<SiteMapWebPage>, rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			css => self.css.execute(resources, inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			font => self.font.execute(resources, inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			gif_animation => self.gif_animation.execute(resources, inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			html => self.html.execute(resources, inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			raster_image => self.raster_image.execute(resources, inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			raw => self.raw.execute(resources, inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
			svg => self.svg.execute(resources, inputContentFilePath, resourceRelativeUrl, handlebars, headerTemplates, languageData, ifLanguageAwareLanguageData, configuration, siteMapWebPages, rssItems),
		}
	}
}
