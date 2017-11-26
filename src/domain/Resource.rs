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
	#[serde(default, skip_deserializing)] canonicalParentFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] resourceInputName: String,
	#[serde(default, skip_deserializing)] resourceInputContentFileNamesWithExtension: Vec<String>,
	#[serde(default, skip_deserializing)] urlData: HashMap<Iso639Dash1Alpha2Language, HashMap<ResourceTag, Rc<UrlData>>>,
}

impl Resource
{
	#[inline(always)]
	pub(crate) fn hasProcessingPriority(&self, processingPriority: ProcessingPriority) -> bool
	{
		self.processingPriority() == processingPriority
	}
	
	#[inline(always)]
	pub(crate) fn urlDataMandatory(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, resourceTag: &ResourceTag) -> Result<&Rc<UrlData>, CordialError>
	{
		self.urlData(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, resourceTag).ok_or_else(|| CordialError::Configuration(format!("Resource '{:?}' urlData missing", self.name())))
	}
	
	#[inline(always)]
	pub(crate) fn urlData(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, resourceTag: &ResourceTag) -> Option<&Rc<UrlData>>
	{
		let urlKey = self.urlKey(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language);
		match self.urlData.get(&urlKey)
		{
			None => None,
			Some(resourceTagToUrlDataMap) => resourceTagToUrlDataMap.get(resourceTag)
		}
	}
	
	#[inline(always)]
	fn htmlPipeline(&self) -> Result<&HtmlPipeline, CordialError>
	{
		match self.pipeline
		{
			ResourcePipeline::html => Ok(&self.html),
			_ => Err(CordialError::Configuration("Not a HTML resource".to_owned())),
		}
	}
	
	#[inline(always)]
	fn urlKey<'a>(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Iso639Dash1Alpha2Language
	{
		let (isForPrimaryLanguageOnly, _isVersioned) = self.is();
		if isForPrimaryLanguageOnly
		{
			fallbackIso639Dash1Alpha2Language
		}
		else if let Some(iso639Dash1Alpha2Language) = iso639Dash1Alpha2Language
		{
			if self.urlData.contains_key(&iso639Dash1Alpha2Language)
			{
				iso639Dash1Alpha2Language
			}
			else
			{
				fallbackIso639Dash1Alpha2Language
			}
		}
		else
		{
			fallbackIso639Dash1Alpha2Language
		}
	}
	
	#[inline(always)]
	pub(crate) fn name(&self) -> PathBuf
	{
		self.canonicalParentFolderPath.join(format!("{}.resource.hjson", self.resourceInputName))
	}
	
	#[inline(always)]
	pub(crate) fn finishInitialization(&mut self, parentHierarchy: Vec<String>, resourceInputName: &str, canonicalParentFolderPath: PathBuf) -> ResourceUrl
	{
		#[inline(always)]
		fn resourceUrl(parentHierarchy: &[String], resourceInputName: &str) -> ResourceUrl
		{
			let mut resourceRelativeUrl = String::with_capacity(1024);
			for parent in parentHierarchy
			{
				resourceRelativeUrl.push_str(parent);
				resourceRelativeUrl.push('/');
			}
			
			resourceRelativeUrl.push_str(resourceInputName);
			
			ResourceUrl::string(resourceRelativeUrl)
		}
		
		self.canonicalParentFolderPath = canonicalParentFolderPath;
		self.resourceInputName = resourceInputName.to_owned();
		self.resourceInputContentFileNamesWithExtension = self.resourceInputContentFileNamesWithExtension(resourceInputName);
		resourceUrl(&parentHierarchy, resourceInputName)
	}
	
	#[inline(always)]
	pub(crate) fn renderResource(&mut self, resourceUrl: &ResourceUrl, resources: &Resources, newResponses: &mut Responses, oldResponses: &Arc<Responses>, configuration: &Configuration, handlebars: &HandlebarsWrapper, rssChannelsByLanguage: &mut HashMap<Iso639Dash1Alpha2Language, HashMap<Rc<RssChannelName>, Vec<RssItem>>>, siteMapWebPagesByLanguage: &mut HashMap<Iso639Dash1Alpha2Language, Vec<SiteMapWebPage>>) -> Result<(), CordialError>
	{
		#[inline(always)]
		fn getOrDefault<'a, T>(map: &'a mut HashMap<Iso639Dash1Alpha2Language, Vec<T>>, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> &'a mut Vec<T>
		{
			map.entry(iso639Dash1Alpha2Language).or_insert_with(|| Vec::with_capacity(4096))
		}
		
		let primaryLanguage = configuration.localization.fallbackIso639Dash1Alpha2Language();
		
		configuration.visitLanguagesWithPrimaryFirst(|languageData, isPrimaryLanguage|
		{
			let (isVersioned, isLanguageAware) = self.is();
			
			let isLanguageAgnosticSoProcessOnlyForPrimaryLanguage = !isLanguageAware;
			
			if isLanguageAgnosticSoProcessOnlyForPrimaryLanguage && !isPrimaryLanguage
			{
			}
			else
			{
				let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
				
				let result =
				{
					let (ifLanguageAwareLanguageData, inputContentFilePath) = if isLanguageAware
					{
						(Some(languageData), self.inputContentFilePath(primaryLanguage, Some(languageData.iso639Dash1Alpha2Language))?)
					}
					else
					{
						(None, self.languageNeutralInputContentFilePath(primaryLanguage, None)?)
					};
					
					let mut rssChannelsToRssItems = rssChannelsByLanguage.get_mut(&iso639Dash1Alpha2Language).unwrap();
					let mut siteMapWebPages = getOrDefault(siteMapWebPagesByLanguage, iso639Dash1Alpha2Language);
					
					let mut headerGenerator = HeaderGenerator
					{
						handlebars,
						headerTemplates: &self.headers,
						ifLanguageAwareLanguageData,
						configuration,
					};
					
					self.execute(resources, &inputContentFilePath, resourceUrl, handlebars, &mut headerGenerator, languageData, configuration, &mut rssChannelsToRssItems, &mut siteMapWebPages)?
				};
				
				// Always inserts, as this language code will only occur once.
				let urls = self.urlData.entry(iso639Dash1Alpha2Language).or_insert(HashMap::with_capacity(result.len()));
				
				for (mut url, mut resourceTagsWithJsonValues, statusCode, contentType, regularHeaders, regularBody, pjax, canBeCompressed) in result
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
					
					let mimeType = contentType.0.clone();
					
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
					
					debug_assert!(!resourceTagsWithJsonValues.is_empty(), "resourceTagsWithJsonValues is empty");
					
					let url =
					{
						if isVersioned
						{
							url.set_query(Some(&format!("v={}", newResponse.entityTag())));
						}
						
						newResponses.addResponse(url.clone(), newResponse, oldResponses.clone());
						
						Rc::new(url)
					};
					
					for (resourceTags, urlDataDetails) in resourceTagsWithJsonValues.drain()
					{
						urls.insert(resourceTags, Rc::new(UrlData
						{
							url: url.clone(),
							mimeType: mimeType.clone(),
							urlDataDetails,
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
	fn inputContentFilePath(&self, primaryLanguage: Iso639Dash1Alpha2Language, language: Option<Iso639Dash1Alpha2Language>) -> Result<PathBuf, CordialError>
	{
		if language.is_some()
		{
			let nonPrimaryLanguage = language.unwrap();
			
			for resourceInputContentFileNameWithExtension in self.resourceInputContentFileNamesWithExtension.iter()
			{
				let languageSpecificFilePath = self.canonicalParentFolderPath.join(format!("{:?}.{}", nonPrimaryLanguage, resourceInputContentFileNameWithExtension));
				if languageSpecificFilePath.exists()
				{
					return Ok(languageSpecificFilePath);
				}
			}
			
			if primaryLanguage != nonPrimaryLanguage
			{
				for resourceInputContentFileNameWithExtension in self.resourceInputContentFileNamesWithExtension.iter()
				{
					let primaryLanguageSpecificFilePath = self.canonicalParentFolderPath.join(format!("{:?}.{}", primaryLanguage, resourceInputContentFileNameWithExtension));
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
	fn languageNeutralInputContentFilePath(&self, primaryLanguage: Iso639Dash1Alpha2Language, language: Option<Iso639Dash1Alpha2Language>) -> Result<PathBuf, CordialError>
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
	pub(crate) fn anchorTitleAttribute(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<Option<Rc<String>>, CordialError>
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			css => self.css.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			font => self.font.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			gif_animation => self.gif_animation.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			html => self.html.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			raster_image => self.raster_image.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			raw => self.raw.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			svg => self.svg.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
		}
	}
	
	#[inline(always)]
	pub(crate) fn addToImgAttributes(&self, attributes: &mut Vec<Attribute>) -> Result<(), CordialError>
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			css => self.css.addToImgAttributes(attributes),
			font => self.font.addToImgAttributes(attributes),
			gif_animation => self.gif_animation.addToImgAttributes(attributes),
			html => self.html.addToImgAttributes(attributes),
			raster_image => self.raster_image.addToImgAttributes(attributes),
			raw => self.raw.addToImgAttributes(attributes),
			svg => self.svg.addToImgAttributes(attributes),
		}
	}
	
	#[inline(always)]
	pub(crate) fn imageMetaData(&self) -> Result<&Rc<ImageMetaData>, CordialError>
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
	fn execute(&self, resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResource>, CordialError>
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			css => self.css.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			font => self.font.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			gif_animation => self.gif_animation.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			html => self.html.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			raster_image => self.raster_image.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			raw => self.raw.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			svg => self.svg.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
		}
	}
}
