// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct resource
{
	pipeline: pipeline,
	headers: HashMap<String, String>,
	compression: compression,
	#[serde(default, skip_deserializing)] canonicalParentFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] resourceInputName: String,
	#[serde(default, skip_deserializing)] resourceInputContentFileNamesWithExtension: Vec<String>,
	#[serde(default, skip_deserializing)] resourceOutputRelativeUrl: String,
}

impl resource
{
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
	
	#[inline(always)]
	pub(crate) fn render(&mut self, iso_639_1_alpha_2_language_code: &str, language: &language, newResources: &mut Resources, oldResources: Arc<Resources>, configuration: &Configuration, handlebars: &mut Handlebars, siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<(), CordialError>
	{
		let (isForPrimaryLanguageOnly, isVersioned) = self.pipeline.is();
		
		let primaryLanguage = configuration.localization.primaryLanguage()?;
		if language != primaryLanguage && isForPrimaryLanguageOnly
		{
			return Ok(());
		}
		
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
		
		let unversionedCanonicalUrl = self.unversionedUrl(language)?;
		
		let result = self.pipeline.execute(&inputContentFilePath, unversionedCanonicalUrl, handlebars, &self.headers, languageData, configuration, siteMapWebPages)?;
		for (mut url, contentType, regularHeaders, regularBody, pjax, canBeCompressed) in result
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
				
				RegularAndPjaxStaticResponse::both(StaticResponse::new(StatusCode::Ok, contentType.clone(), regularHeaders, regularBody, regularCompressed), Some(StaticResponse::new(StatusCode::Ok, contentType, pjaxHeaders, pjaxBody, pjaxCompressed)))
			}
			else
			{
				RegularAndPjaxStaticResponse::regular(StaticResponse::new(StatusCode::Ok, contentType, regularHeaders, regularBody, regularCompressed))
			};
			
			if isVersioned
			{
				url.set_query(Some(&format!("v={}", newResponse.entityTag())))
			}
			
			newResources.addResource(url, newResponse, oldResources.clone());
		}
		
		Ok(())
	}
	
	/// if language is some, then searches for resource by language, primary language or language-neutral name in descending order
	/// if language is none, the searches by language-neutral name
	#[inline(always)]
	fn inputContentFilePath(&self, primaryLanguage: &language, language: Option<&language>) -> Result<PathBuf, CordialError>
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
	fn languageNeutralInputContentFilePath(&self, primaryLanguage: &language, language: Option<&language>) -> Result<PathBuf, CordialError>
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
	fn unversionedUrl(&self, language: &language) -> Result<Url, CordialError>
	{
		self.url(language, HtmlVariant::Canonical, None)
	}
	
	#[inline(always)]
	fn url(&self, language: &language, htmlVariant: HtmlVariant, version: Option<&str>) -> Result<Url, CordialError>
	{
		let baseUrl = language.baseUrl()?;
		
		let urlWithAmpOrPjaxPath = htmlVariant.appendToUrl(baseUrl);
		
		let mut url = urlWithAmpOrPjaxPath.join(&self.resourceOutputRelativeUrl).context(format!("Invalid resourceOutputRelativeUrl '{}'", self.resourceOutputRelativeUrl))?;
		if let Some(version) = version
		{
			url.set_query(Some(&format!("v={}", version)))
		}
		Ok(url)
	}
}
