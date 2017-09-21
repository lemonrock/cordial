// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Resource
{
	#[serde(default)] pipeline: Pipeline,
	#[serde(default)] headers: HashMap<String, String>,
	#[serde(default)] compression: Compression,
	#[serde(default)] is_data_uri: bool,
	#[serde(default, skip_deserializing)] canonicalParentFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] resourceInputName: String,
	#[serde(default, skip_deserializing)] resourceInputContentFileNamesWithExtension: Vec<String>,
	#[serde(default, skip_deserializing)] resourceOutputRelativeUrl: String,
	#[serde(default, skip_deserializing)] urls: HashMap<String, HashMap<UrlTag, (Url, Rc<JsonValue>)>>,
	#[serde(default, skip_deserializing)] resourceIfDataUri: HashMap<Url, (RegularAndPjaxStaticResponse, Url)>,
}

impl Resource
{
	#[inline(always)]
	pub(crate) fn resourceOutputRelativeUrl(&self) -> &str
	{
		&self.resourceOutputRelativeUrl
	}
	
	#[inline(always)]
	pub(crate) fn hasProcessingPriority(&self, processingPriority: ProcessingPriority) -> bool
	{
		self.pipeline.processingPriority() == processingPriority
	}
	
	#[inline(always)]
	pub(crate) fn urlAndResource<'a, 'b: 'a>(&'a self, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, urlTag: &UrlTag, newResources: &'b Resources) -> Option<(&'a Url, &'a RegularAndPjaxStaticResponse)>
	{
		match self.url(primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code, urlTag)
		{
			None => None,
			Some(url) =>
			{
				if self.is_data_uri
				{
					let &(ref resource, ref dataUri) = self.resourceIfDataUri.get(url).expect("BUG: data-uri resource missing");
					Some((dataUri, resource))
				}
				else
				{
					Some((url, newResources.getLatestResponse(url).expect("BUG: newResources resource missing")))
				}
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn urlAndJsonValue<'a>(&'a self, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, urlTag: &UrlTag) -> Option<(&'a Url, Rc<JsonValue>)>
	{
		match self.urls.get(self.urlKey(primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code))
		{
			None => None,
			Some(urlTags) => match urlTags.get(urlTag)
			{
				None => None,
				Some(&(ref url, ref jsonValue)) => Some((url, jsonValue.clone()))
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn url<'a>(&'a self, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, urlTag: &UrlTag) -> Option<&'a Url>
	{
		match self.urls.get(self.urlKey(primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code))
		{
			None => None,
			Some(urlTags) => match urlTags.get(urlTag)
			{
				None => None,
				Some(&(ref url, _)) => Some(url)
			}
		}
	}
	
	#[inline(always)]
	fn urlKey<'a>(&self, primary_iso_639_1_alpha_2_language_code: &'a str, iso_639_1_alpha_2_language_code: Option<&'a str>) -> &'a str
	{
		let (isForPrimaryLanguageOnly, _isVersioned) = self.pipeline.is();
		if isForPrimaryLanguageOnly
		{
			primary_iso_639_1_alpha_2_language_code
		}
		else if let Some(iso_639_1_alpha_2_language_code) = iso_639_1_alpha_2_language_code
		{
			if self.urls.contains_key(iso_639_1_alpha_2_language_code)
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
		self.canonicalParentFolderPath = canonicalParentFolderPath;
		self.resourceInputName = resourceInputName.to_owned();
		self.resourceInputContentFileNamesWithExtension = self.pipeline.resourceInputContentFileNamesWithExtension(resourceInputName);
		self.resourceOutputRelativeUrl = self.pipeline.resourceOutputRelativeUrl(&parentHierarchy, resourceInputName);
	}
	
	// SiteMap, RSS hashmaps are by language ISO code
	#[inline(always)]
	pub(crate) fn render(&mut self, newResources: &mut Resources, oldResources: &Arc<Resources>, configuration: &Configuration, handlebars: &mut Handlebars, siteMapWebPages: &mut HashMap<String, Vec<SiteMapWebPage>>, rssItems: &mut HashMap<String, Vec<RssItem>>) -> Result<(), CordialError>
	{
		let primaryLanguage = configuration.localization.primaryLanguage()?;
		
		configuration.visitLanguagesWithPrimaryFirst(|iso_639_1_alpha_2_language_code, language, isPrimaryLanguage|
		{
			let (isForPrimaryLanguageOnly, isVersioned) = self.pipeline.is();
			
			if !isPrimaryLanguage && isForPrimaryLanguageOnly
			{
			}
			else
			{
				let languageData = if isForPrimaryLanguageOnly
				{
					Some((iso_639_1_alpha_2_language_code, language))
				}
				else
				{
					None
				};
				
				let inputContentFilePath = if isForPrimaryLanguageOnly
				{
					self.languageNeutralInputContentFilePath(primaryLanguage, None)?
				}
				else
				{
					self.inputContentFilePath(primaryLanguage, Some(language))?
				};
				
				let unversionedCanonicalUrl = self.unversionedUrl(iso_639_1_alpha_2_language_code, language)?;
				
				let mut siteMapWebPages = siteMapWebPages.entry(iso_639_1_alpha_2_language_code.to_owned()).or_insert_with(|| Vec::with_capacity(4096));
				
				let mut rssItems = rssItems.entry(iso_639_1_alpha_2_language_code.to_owned()).or_insert_with(|| Vec::with_capacity(4096));
				
				let result = self.pipeline.execute(&inputContentFilePath, unversionedCanonicalUrl, handlebars, &self.headers, languageData, configuration, &mut siteMapWebPages, &mut rssItems)?;
				
				let urls = self.urls.entry(iso_639_1_alpha_2_language_code.to_owned()).or_insert(HashMap::new());
				for (mut url, urlTagAndJsonValuePairs, contentType, regularHeaders, regularBody, pjax, canBeCompressed) in result
				{
					debug_assert!(!urlTagAndJsonValuePairs.is_empty(), "urlTags is empty");
					
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
						
						RegularAndPjaxStaticResponse::both(StaticResponse::new(StatusCode::Ok, contentType.clone(), regularHeaders, regularBody, regularCompressed), Some(StaticResponse::new(StatusCode::Ok, contentType, pjaxHeaders, pjaxBody, pjaxCompressed)))
					}
					else
					{
						RegularAndPjaxStaticResponse::regular(StaticResponse::new(StatusCode::Ok, contentType, regularHeaders, regularBody, regularCompressed))
					};
					
					if isVersioned
					{
						url.set_query(Some(&format!("v={}", newResponse.entityTag())));
					}
					
					for (urlTag, jsonValue) in urlTagAndJsonValuePairs.iter()
					{
						urls.insert(*urlTag, (url.clone(), jsonValue.clone()));
					}
					
					if self.is_data_uri
					{
						let dataUri = newResponse.toDataUri();
						self.resourceIfDataUri.insert(url, (newResponse, dataUri));
					}
					else
					{
						newResources.addResource(url, newResponse, oldResources.clone());
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
	fn unversionedUrl(&self, iso_639_1_alpha_2_language_code: &str, language: &Language) -> Result<Url, CordialError>
	{
		let baseUrl = language.baseUrl(iso_639_1_alpha_2_language_code)?;
		let url = baseUrl.join(&self.resourceOutputRelativeUrl).context(format!("Invalid resourceOutputRelativeUrl '{}'", self.resourceOutputRelativeUrl))?;
		Ok(url)
	}
}
