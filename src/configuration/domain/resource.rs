// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub struct resource
{
	pipeline: pipeline, // html, sitemap/xml, robots/txt, rss/xml, json, ?js?, png, jpeg, gif, svg, sass, scss, temporary-redirect, permanent-redirect
	headers: HashMap<String, String>,
	compression: compression,
	
	#[serde(default, skip_deserializing)] canonicalParentFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] resourceInputName: String,
	#[serde(default, skip_deserializing)] resourceInputContentFileNamesWithExtension: Vec<String>,
	#[serde(default, skip_deserializing)] resourceOutputRelativeUrl: String,
	#[serde(default, skip_deserializing)] additionalContentFileNameIfAny: Option<&'static str>,
}

impl resource
{
	#[inline(always)]
	pub fn name(&self) -> PathBuf
	{
		self.canonicalParentFolderPath.join(format!("{}.resource.hjson", self.resourceInputName))
	}
	
	#[inline(always)]
	pub fn finishInitialization(&mut self, parentHierarchy: Vec<String>, resourceInputName: &str, canonicalParentFolderPath: PathBuf)
	{
		self.canonicalParentFolderPath = canonicalParentFolderPath;
		self.resourceInputName = resourceInputName.to_owned();
		self.resourceInputContentFileNamesWithExtension = self.pipeline.resourceInputContentFileNamesWithExtension(resourceInputName);
		let (resourceOutputRelativeUrl, additionalContentFileNameIfAny) = self.pipeline.resourceOutputRelativeUrl(&parentHierarchy, resourceInputName);
		self.resourceOutputRelativeUrl = resourceOutputRelativeUrl;
		self.additionalContentFileNameIfAny = additionalContentFileNameIfAny;
	}
	
	/// if language is some, then searches for resource by language, primary language or language-neutral name in descending order
	/// if language is none, the searches by language-neutral name
	#[inline(always)]
	pub fn inputContentFilePath(&self, primaryLanguage: &language, language: Option<&language>) -> Result<PathBuf, CordialError>
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
	pub fn createFinalResourceContent<ContentCreator: FnMut(PathBuf) -> Result<Vec<PathBuf>, CordialError>>(&self, language: &language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash: Option<&str>, siteOutputFolderPath: &Path, mut contentCreator: ContentCreator) -> Result<(), CordialError>
	{
		let relativeOutputContentFilePath = self.relativeOutputContentFilePath(language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash)?;
		let outputFilePath = siteOutputFolderPath.join(relativeOutputContentFilePath);
		let outputParentFolderPath = outputFilePath.parent().unwrap();
		outputParentFolderPath.createFolder().context(outputParentFolderPath)?;
		let resourcesToCompress = contentCreator(outputFilePath)?;
		
		for resourceToCompress in resourcesToCompress
		{
			self.compression.compress(&resourceToCompress)?
		}
		
		Ok(())
	}
	
	#[inline(always)]
	pub fn canonicalUrl(&self, primaryLanguage: &language) -> Result<Url, CordialError>
	{
		self.url(primaryLanguage, None)
	}
	
	#[inline(always)]
	pub fn ampUrl(&self, language: &language) -> Result<Url, CordialError>
	{
		self.url(language, Some("amp/"))
	}
	
	#[inline(always)]
	pub fn pjaxUrl(&self, language: &language) -> Result<Url, CordialError>
	{
		self.url(language, Some("pjax/"))
	}
	
	// variantSubPathEgAcceleratedMobilePagesWithTrailingSlash is also for PJAX
	#[inline(always)]
	pub fn url(&self, language: &language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash: Option<&str>) -> Result<Url, CordialError>
	{
		let baseUrl = language.baseUrl()?;
		
		let urlWithAmpOrPjaxPath = if let Some(variantSubPathEgAcceleratedMobilePagesWithTrailingSlash) = variantSubPathEgAcceleratedMobilePagesWithTrailingSlash
		{
			baseUrl.join(variantSubPathEgAcceleratedMobilePagesWithTrailingSlash).context(format!("Invalid variant '{}'", variantSubPathEgAcceleratedMobilePagesWithTrailingSlash))?
		}
		else
		{
			baseUrl
		};
		
		let resourceOutputRelativeUrl = &self.resourceOutputRelativeUrl;
		Ok(urlWithAmpOrPjaxPath.join(resourceOutputRelativeUrl).context(format!("Invalid resourceOutputRelativeUrl '{}'", resourceOutputRelativeUrl))?)
	}
	
	#[inline(always)]
	pub fn relativeOutputContentFilePath(&self, language: &language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash: Option<&str>) -> Result<PathBuf, CordialError>
	{
		let url = self.url(language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash)?;
		let fileLikeUrl = if let Some(additionalContentFileName) = self.additionalContentFileNameIfAny
		{
			url.join(additionalContentFileName).unwrap()
		}
		else
		{
			url
		};
		
		let resourceRelativePathString = String::with_capacity(1024);
		resourceRelativePathString.push_str(fileLikeUrl.host_str().unwrap());
		resourceRelativePathString.push_str(fileLikeUrl.path());
		Ok(PathBuf::from(resourceRelativePathString))
	}
}
