// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Resource
{
	#[serde(default)] pipeline: ResourcePipeline,
	#[serde(default)] audio: AudioPipeline,
	#[serde(default)] browser_config: BrowserConfigPipeline,
	#[serde(default)] css: CssPipeline,
	#[serde(default)] font: FontPipeline,
	#[serde(default)] gif_animation: GifAnimationPipeline,
	#[serde(default)] html: HtmlPipeline,
	#[serde(default)] raster_image: RasterImagePipeline,
	#[serde(default)] raw: RawPipeline,
	#[serde(default)] svg: SvgPipeline,
	#[serde(default)] web_app_manifest: WebAppManifestPipeline,
	#[serde(default)] video: VideoPipeline,
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
	pub(crate) fn findGoogleVideoSiteMapImageThumbnail(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<&Rc<UrlData>, CordialError>
	{
		// Find the largest image that is acceptable
		// Images must be at least 160x90 pixels and at most 1920x1080 pixels
		// Ideally, they are a ratio of 16:9, but we do not optimize for this; we optimize for width
		
		let urlKey = self.urlKey(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language);
		match self.urlData.get(&urlKey)
		{
			None => Err(CordialError::Configuration(format!("Could not find an image for Google Video Sitemap image for '{:?}' for language choice", self.name()))),
			Some(resourceTagToUrlDataMap) =>
			{
				let mut idealImageWidth = 0;
				let mut idealImageUrlData = None;
				
				for (resourceTag, urlData) in resourceTagToUrlDataMap.iter()
				{
					if urlData.isSuitableForGoogleVideoSiteMapThumbnailImage()
					{
						match resourceTag
						{
							&ResourceTag::width_height_image(width, height) =>
							{
								if width > idealImageWidth && width >= 160 && width <= 1920 && height >= 90 && height <= 1080
								{
									idealImageWidth = width;
									idealImageUrlData = Some(urlData);
								}
							}
							
							_ => (),
						}
					}
				}
				
				match idealImageUrlData
				{
					Some(urlData) => Ok(urlData),
					None => Err(CordialError::Configuration(format!("Could not find an ideal image for iTunes Artwork image for '{:?}'", self.name()))),
				}
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn findGooglePlayRssArtwork(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<&Rc<UrlData>, CordialError>
	{
		// Find the largest image that is acceptable
		// Artwork must be an effective minimum size of 1200 x 1200 pixels (600 x 600 is minimum with warnings) and a maximum size of 7000 x 7000 pixels, in JPEG or PNG format
		
		let urlKey = self.urlKey(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language);
		match self.urlData.get(&urlKey)
		{
			None => Err(CordialError::Configuration(format!("Could not find an image for Google Play Artwork for '{:?}' for language choice", self.name()))),
			Some(resourceTagToUrlDataMap) =>
			{
				let mut idealImageWidth = 0;
				let mut idealImageUrlData = None;
				
				for (resourceTag, urlData) in resourceTagToUrlDataMap.iter()
				{
					if urlData.isSuitableForGooglePlayArtwork()
					{
						match resourceTag
						{
							&ResourceTag::width_height_image(width, height) =>
							{
								if width > idealImageWidth && width >= 1200 && width <= 7000 && height >= 1200 && height <= 7000
								{
									idealImageWidth = width;
									idealImageUrlData = Some(urlData);
								}
							}
							
							_ => (),
						}
					}
				}
				
				match idealImageUrlData
				{
					Some(urlData) => Ok(urlData),
					None => Err(CordialError::Configuration(format!("Could not find an ideal image for Google Play Artwork for '{:?}'", self.name()))),
				}
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn findITunesRssArtwork(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<&Rc<UrlData>, CordialError>
	{
		// Find the largest image that is acceptable
		// Artwork must be a minimum size of 1400 x 1400 pixels and a maximum size of 3000 x 3000 pixels, in JPEG or PNG format, 72 dpi, with appropriate file extensions (.jpg, .png), and in the RGB colorspace.
		// Under 500Kb
		
		const FiveHundredKilobytes: u64 = 500 * 1024;
		
		let urlKey = self.urlKey(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language);
		match self.urlData.get(&urlKey)
		{
			None => Err(CordialError::Configuration(format!("Could not find an image for iTunes Artwork for '{:?}' for language choice", self.name()))),
			Some(resourceTagToUrlDataMap) =>
			{
				let mut idealImageWidth = 0;
				let mut idealImageUrlData = None;
				
				for (resourceTag, urlData) in resourceTagToUrlDataMap.iter()
				{
					if urlData.isSuitableForITunesArtwork() && urlData.size() < FiveHundredKilobytes
					{
						match resourceTag
						{
							&ResourceTag::width_height_image(width, height) =>
							{
								if width > idealImageWidth && width >= 1400 && width <= 3000 && height >= 1400 && height <= 3000
								{
									idealImageWidth = width;
									idealImageUrlData = Some(urlData);
								}
							}
							
							_ => (),
						}
					}
				}
				
				match idealImageUrlData
				{
					Some(urlData) => Ok(urlData),
					None => Err(CordialError::Configuration(format!("Could not find an ideal image for iTunes Artwork for '{:?}'", self.name()))),
				}
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn findUrlForFacebookOpenGraph(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, resourceTag: &ResourceTag, facebookOpenGraphTypeDiscriminant: FacebookOpenGraphTypeDiscriminant) -> Result<&Rc<Url>, CordialError>
	{
		let htmlPipeline = self.htmlPipeline()?;
		if htmlPipeline.hasFacebookOpenGraphTypeDiscriminant(facebookOpenGraphTypeDiscriminant)
		{
			Ok(&self.urlDataMandatory(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, resourceTag)?.url)
		}
		else
		{
			Err(CordialError::Configuration(format!("{:?} does not has facebookOpenGraphTypeDiscriminant {:?}", self.name(),  facebookOpenGraphTypeDiscriminant)))
		}
	}
	
	#[inline(always)]
	pub(crate) fn findUrlForTwitterCardImage(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, twitterCardImageMatch: &TwitterCardImageMatch) -> Result<&Rc<UrlData>, CordialError>
	{
		let urlKey = self.urlKey(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language);
		match self.urlData.get(&urlKey)
		{
			None => Err(CordialError::Configuration(format!("Could not find an ideal image for Twitter Cards twitter::image for '{:?}' for language choice", self.name()))),
			Some(resourceTagToUrlDataMap) =>
			{
				// Find the largest image with the correct ratio and minimum dimensions under 5 Mb
				let mut idealImageWidth = 0;
				let mut idealImageUrlData = None;
				
				for (resourceTag, urlData) in resourceTagToUrlDataMap.iter()
				{
					if urlData.isSuitableForTwitterCardsImage()
					{
						match resourceTag
						{
							&ResourceTag::width_height_image(width, height) =>
							{
								if urlData.size() < twitterCardImageMatch.maximumSize && idealImageWidth < width
								{
									let remainder = width % height;
									let ratio = width / height;
									if remainder == 0 && ratio == twitterCardImageMatch.ratio && width >= twitterCardImageMatch.minimumWidth && width <= twitterCardImageMatch.maximumWidth && height >= twitterCardImageMatch.minimumHeight && height <= twitterCardImageMatch.maximumHeight
									{
										idealImageWidth = width;
										idealImageUrlData = Some(urlData);
									}
								}
							}
							
							_ => (),
						}
					}
				}
				
				match idealImageUrlData
				{
					Some(urlData) => Ok(urlData),
					None => Err(CordialError::Configuration(format!("Could not find an ideal image for Twitter Cards twitter::image for '{:?}'", self.name()))),
				}
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn findUrlDataForTwitterCardPlayerPlaceHolderImage(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<&Rc<UrlData>, CordialError>
	{
		const FiveMegabytesExclusive: u64 = 5 * 1024 * 1024;
		const MinimumPixelsInclusive: u64 = 68_800;
		
		let urlKey = self.urlKey(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language);
		match self.urlData.get(&urlKey)
		{
			None => Err(CordialError::Configuration(format!("Could not find an ideal image for Twitter Cards player twitter::image for '{:?}' for language choice", self.name()))),
			Some(resourceTagToUrlDataMap) =>
			{
				// Find the largest image with the correct ratio and minimum dimensions under 5 Mb and more than 68,600 pixels.
				let mut idealImageWidth = 0;
				let mut idealImageUrlData = None;
				
				for (resourceTag, urlData) in resourceTagToUrlDataMap.iter()
				{
					if urlData.isSuitableForTwitterCardsImage()
					{
						match resourceTag
						{
							&ResourceTag::width_height_image(width, height) =>
							{
								if urlData.size() < FiveMegabytesExclusive && idealImageWidth < width && (width as u64 * height as u64) >= MinimumPixelsInclusive
								{
									idealImageWidth = width;
									idealImageUrlData = Some(urlData);
								}
							}
							
							_ => (),
						}
					}
				}
				
				match idealImageUrlData
				{
					Some(urlData) => Ok(urlData),
					None => Err(CordialError::Configuration(format!("Could not find an ideal image for Twitter Cards player twitter::image for '{:?}'", self.name()))),
				}
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn findUrlDataForFacebookOpenGraphImage(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<&Rc<UrlData>, CordialError>
	{
		const EightMegabytes: u64 = 8 * 1024 * 1024;
		
		let urlKey = self.urlKey(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language);
		match self.urlData.get(&urlKey)
		{
			None => Err(CordialError::Configuration(format!("Could not find an ideal or acceptable image for Facebook OpenGraph og::image for '{:?}' for language choice", self.name()))),
			Some(resourceTagToUrlDataMap) =>
			{
				// https://developers.facebook.com/docs/sharing/best-practices#images
				
				// Find the largest image with the correct ratio and minimum dimensions under 8 Mb
				let mut idealImageWidth = 0;
				let mut idealImageUrlData = None;
				let mut acceptableImageWidth = 0;
				let mut acceptableImageUrlData = None;
				
				for (resourceTag, urlData) in resourceTagToUrlDataMap.iter()
				{
					if urlData.isSuitableForFacebookOpenGraphImage()
					{
						match resourceTag
						{
							&ResourceTag::width_height_image(width, height) =>
							{
								if urlData.size() <= EightMegabytes
								{
									// Facebook images should have a ratio of 1.91, but even facebook's own examples give a ratio of 1.9047619047619 for their ideal image.
									let ratio = ((width * 100) as f64) / (height as f64);
									if width > idealImageWidth && width >= 600 && height >= 315 && (ratio > 190.0 || ratio < 192.0)
									{
										idealImageWidth = width;
										idealImageUrlData = Some(urlData);
									}
									if width > acceptableImageWidth && width >= 200 && height >= 200 && idealImageUrlData.is_none()
									{
										acceptableImageWidth = width;
										acceptableImageUrlData = Some(urlData);
									}
								}
							}
							_ => (),
						}
					}
				}
				if let Some(idealImageUrl) = idealImageUrlData
				{
					Ok(idealImageUrl)
				}
				else if let Some(acceptableImageUrl) = acceptableImageUrlData
				{
					Ok(acceptableImageUrl)
				}
				else
				{
					Err(CordialError::Configuration(format!("Could not find an ideal or acceptable image for Facebook OpenGraph og::image for '{:?}'", self.name())))
				}
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn audioPipeline(&self) -> Result<&AudioPipeline, CordialError>
	{
		match self.pipeline
		{
			ResourcePipeline::audio => Ok(&self.audio),
			_ => Err(CordialError::Configuration("Not an audio resource".to_owned())),
		}
	}
	
	#[inline(always)]
	pub(crate) fn videoPipeline(&self) -> Result<&VideoPipeline, CordialError>
	{
		match self.pipeline
		{
			ResourcePipeline::video => Ok(&self.video),
			_ => Err(CordialError::Configuration("Not a video resource".to_owned())),
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
			audio => self.audio.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			browser_config => self.browser_config.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			css => self.css.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			font => self.font.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			gif_animation => self.gif_animation.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			html => self.html.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			raster_image => self.raster_image.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			raw => self.raw.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			svg => self.svg.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			web_app_manifest => self.web_app_manifest.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
			video => self.video.anchorTitleAttribute(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language),
		}
	}
	
	#[inline(always)]
	pub(crate) fn addToImgAttributes(&self, attributes: &mut Vec<Attribute>) -> Result<(), CordialError>
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			audio => self.audio.addToImgAttributes(attributes),
			browser_config => self.browser_config.addToImgAttributes(attributes),
			css => self.css.addToImgAttributes(attributes),
			font => self.font.addToImgAttributes(attributes),
			gif_animation => self.gif_animation.addToImgAttributes(attributes),
			html => self.html.addToImgAttributes(attributes),
			raster_image => self.raster_image.addToImgAttributes(attributes),
			raw => self.raw.addToImgAttributes(attributes),
			svg => self.svg.addToImgAttributes(attributes),
			web_app_manifest => self.web_app_manifest.addToImgAttributes(attributes),
			video => self.video.addToImgAttributes(attributes),
		}
	}
	
	#[inline(always)]
	pub(crate) fn imageMetaData(&self) -> Result<&Rc<ImageMetaData>, CordialError>
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			audio => self.audio.imageMetaData(),
			browser_config => self.browser_config.imageMetaData(),
			css => self.css.imageMetaData(),
			font => self.font.imageMetaData(),
			gif_animation => self.gif_animation.imageMetaData(),
			html => self.html.imageMetaData(),
			raster_image => self.raster_image.imageMetaData(),
			raw => self.raw.imageMetaData(),
			svg => self.svg.imageMetaData(),
			web_app_manifest => self.web_app_manifest.imageMetaData(),
			video => self.video.imageMetaData(),
		}
	}
	
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			audio => self.audio.processingPriority(),
			browser_config => self.browser_config.processingPriority(),
			css => self.css.processingPriority(),
			font => self.font.processingPriority(),
			gif_animation => self.gif_animation.processingPriority(),
			html => self.html.processingPriority(),
			raster_image => self.raster_image.processingPriority(),
			raw => self.raw.processingPriority(),
			svg => self.svg.processingPriority(),
			web_app_manifest => self.web_app_manifest.processingPriority(),
			video => self.video.processingPriority(),
		}
	}
	
	#[inline(always)]
	fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			audio => self.audio.resourceInputContentFileNamesWithExtension(resourceInputName),
			browser_config => self.browser_config.resourceInputContentFileNamesWithExtension(resourceInputName),
			css => self.css.resourceInputContentFileNamesWithExtension(resourceInputName),
			font => self.font.resourceInputContentFileNamesWithExtension(resourceInputName),
			gif_animation => self.gif_animation.resourceInputContentFileNamesWithExtension(resourceInputName),
			html => self.html.resourceInputContentFileNamesWithExtension(resourceInputName),
			raster_image => self.raster_image.resourceInputContentFileNamesWithExtension(resourceInputName),
			raw => self.raw.resourceInputContentFileNamesWithExtension(resourceInputName),
			svg => self.svg.resourceInputContentFileNamesWithExtension(resourceInputName),
			web_app_manifest => self.web_app_manifest.resourceInputContentFileNamesWithExtension(resourceInputName),
			video => self.video.resourceInputContentFileNamesWithExtension(resourceInputName),
		}
	}
	
	#[inline(always)]
	fn is<'a>(&self) -> (bool, bool)
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			audio => self.audio.is(),
			browser_config => self.browser_config.is(),
			css => self.css.is(),
			font => self.font.is(),
			gif_animation => self.gif_animation.is(),
			html => self.html.is(),
			raster_image => self.raster_image.is(),
			raw => self.raw.is(),
			svg => self.svg.is(),
			web_app_manifest => self.web_app_manifest.is(),
			video => self.video.is(),
		}
	}
	
	#[inline(always)]
	fn execute(&self, resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResource>, CordialError>
	{
		use self::ResourcePipeline::*;
		match self.pipeline
		{
			audio => self.audio.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			browser_config => self.browser_config.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			css => self.css.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			font => self.font.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			gif_animation => self.gif_animation.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			html => self.html.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			raster_image => self.raster_image.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			raw => self.raw.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			svg => self.svg.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			web_app_manifest => self.web_app_manifest.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
			video => self.video.execute(resources, inputContentFilePath, resourceUrl, handlebars, headerGenerator, languageData, configuration, rssChannelsToRssItems, siteMapWebPages),
		}
	}
}
