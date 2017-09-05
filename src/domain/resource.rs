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
	
	#[inline(always)]
	pub fn createOutput(&self, primaryLanguage: &language, language: &language, variant: Variant, siteOutputFolderPath: &Path, canonicalizedInputFolderPath: &Path) -> Result<(), CordialError>
	{
		let (isForPrimaryLanguageOnly, isForCanonicalUrlOnly) = self.pipeline.isForPrimaryLanguageAndCanonicalUrlOnly();
		
		if language != primaryLanguage && isForPrimaryLanguageOnly
		{
			return Ok(());
		}
		
		if variant != Variant::Canonical && isForCanonicalUrlOnly
		{
			return Ok(());
		}
		
		let inputContentFilePath = if isForPrimaryLanguageOnly
		{
			self.languageNeutralInputContentFilePath(primaryLanguage, None)?
		}
		else
		{
			self.inputContentFilePath(primaryLanguage, Some(language))?
		};
		
		info!("Creating output for URL {}", self.url(language, variant).unwrap());
		
		let relativeOutputContentFilePath = self.relativeOutputContentFilePath(language, variant)?;
		let outputFilePath = siteOutputFolderPath.join(relativeOutputContentFilePath);
		{
			let outputParentFolderPath = outputFilePath.parent().unwrap();
			outputParentFolderPath.createFolder().context(outputParentFolderPath)?;
		}
		
		let resourcesToCompress = self.pipeline.execute(&inputContentFilePath, variant, outputFilePath, canonicalizedInputFolderPath)?;
		
		for resourceToCompress in resourcesToCompress
		{
			self.compression.compress(&resourceToCompress)?
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
	pub fn canonicalUrl(&self, primaryLanguage: &language) -> Result<Url, CordialError>
	{
		self.url(primaryLanguage, Variant::Canonical)
	}
	
	#[inline(always)]
	pub fn ampUrl(&self, language: &language) -> Result<Url, CordialError>
	{
		self.url(language, Variant::AMP)
	}
	
	#[inline(always)]
	pub fn pjaxUrl(&self, language: &language) -> Result<Url, CordialError>
	{
		self.url(language, Variant::PJAX)
	}
	
	#[inline(always)]
	pub fn url(&self, language: &language, variant: Variant) -> Result<Url, CordialError>
	{
		let baseUrl = language.baseUrl()?;
		
		let urlWithAmpOrPjaxPath = variant.appendToUrl(baseUrl);
		
		let resourceOutputRelativeUrl = &self.resourceOutputRelativeUrl;
		Ok(urlWithAmpOrPjaxPath.join(resourceOutputRelativeUrl).context(format!("Invalid resourceOutputRelativeUrl '{}'", resourceOutputRelativeUrl))?)
	}
	
	#[inline(always)]
	pub fn relativeOutputContentFilePath(&self, language: &language, variant: Variant) -> Result<PathBuf, CordialError>
	{
		let url = self.url(language, variant)?;
		let fileLikeUrl = if let Some(additionalContentFileName) = self.additionalContentFileNameIfAny
		{
			url.join(additionalContentFileName).unwrap()
		}
		else
		{
			url
		};
		
		let mut resourceRelativePathString = String::with_capacity(1024);
		resourceRelativePathString.push_str(fileLikeUrl.host_str().unwrap());
		resourceRelativePathString.push_str(fileLikeUrl.path());
		Ok(PathBuf::from(resourceRelativePathString))
	}
}
