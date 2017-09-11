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
	#[serde(default, skip_deserializing)] additionalContentFileNameIfAny: Option<&'static str>,
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
		let (resourceOutputRelativeUrl, additionalContentFileNameIfAny) = self.pipeline.resourceOutputRelativeUrl(&parentHierarchy, resourceInputName);
		self.resourceOutputRelativeUrl = resourceOutputRelativeUrl;
		self.additionalContentFileNameIfAny = additionalContentFileNameIfAny;
	}
	
	#[inline(always)]
	pub(crate) fn createOutput(&self, environment: &str, iso_639_1_alpha_2_language_code: &str, language: &language, localization: &localization, inputFolderPath: &Path, newResources: &mut Resources, oldResources: Arc<Resources>, deploymentVersion: &str) -> Result<(), CordialError>
	{
		let (maximumAge, isForPrimaryLanguageOnly, isForCanonicalUrlOnly, canBeCompressed, contentType, isDownloadable, version) = self.pipeline.isFor(deploymentVersion);
		
		let primaryLanguage = localization.primaryLanguage()?;
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
		
		if isForCanonicalUrlOnly
		{
			let variant = Variant::Canonical;
			let url = self.url(language, variant, version)?;
			let regularHeaders = self.generateHeaders(environment, languageData, variant, deploymentVersion, localization, canBeCompressed, maximumAge, isDownloadable)?;
			let regularBody = self.pipeline.execute(&inputContentFilePath, variant, inputFolderPath)?;
			
			let regularCompressed = if canBeCompressed
			{
				Some(self.compression.compress(&regularBody)?)
			}
			else
			{
				None
			};
			
			let newResponse = RegularAndPjaxStaticResponse::new(StaticResponse::new(StatusCode::Ok, contentType.clone(), regularHeaders, regularBody, regularCompressed), None);
			
			newResources.addResource(url, newResponse, oldResources);
		}
		else
		{
			let url = self.url(language, Variant::Canonical, version)?;
			
			let variant = Variant::Canonical;
			let regularHeaders = self.generateHeaders(environment, languageData, variant, deploymentVersion, localization, canBeCompressed, maximumAge, isDownloadable)?;
			let regularBody = self.pipeline.execute(&inputContentFilePath, variant, inputFolderPath)?;
			
			let variant = Variant::PJAX;
			let pjaxHeaders = self.generateHeaders(environment, languageData, variant, deploymentVersion, localization, canBeCompressed, maximumAge, isDownloadable)?;
			let pjaxBody = self.pipeline.execute(&inputContentFilePath, variant, inputFolderPath)?;
			
			let (regularCompressed, pjaxCompressed) = if canBeCompressed
			{
				(Some(self.compression.compress(&regularBody)?), Some(self.compression.compress(&pjaxBody)?))
			}
			else
			{
				(None, None)
			};
			
			let newResource = RegularAndPjaxStaticResponse::new(StaticResponse::new(StatusCode::Ok, contentType.clone(), regularHeaders, regularBody, regularCompressed), Some(StaticResponse::new(StatusCode::Ok, contentType.clone(), pjaxHeaders, pjaxBody, pjaxCompressed)));
			
			newResources.addResource(url, newResource, oldResources.clone());
			
			
			let url = self.url(language, variant, version)?;
			let variant = Variant::AMP;
			let ampHeaders = self.generateHeaders(environment, languageData, variant, deploymentVersion, localization, canBeCompressed, maximumAge, isDownloadable)?;
			let ampBody = self.pipeline.execute(&inputContentFilePath, variant, inputFolderPath)?;
			let ampCompressed = if canBeCompressed
			{
				Some(self.compression.compress(&ampBody)?)
			}
			else
			{
				None
			};
			
			let newResource = RegularAndPjaxStaticResponse::new(StaticResponse::new(StatusCode::Ok, contentType, ampHeaders, ampBody, ampCompressed), None);
			
			newResources.addResource(url, newResource, oldResources);
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
	fn generateHeaders(&self, environment: &str, languageData: Option<(&str, &language)>, variant: Variant, deploymentVersion: &str, localization: &localization, canBeCompressed: bool, maximumAge: u32, isDownloadable: bool) -> Result<Vec<(String, String)>, CordialError>
	{
		let mut headers = Vec::with_capacity(self.headers.len() * 2);
		
		if variant == Variant::PJAX
		{
			headers.push(("X-PJAX-Version".to_owned(), format!("{}", deploymentVersion)));
		}
		if canBeCompressed
		{
			if variant == Variant::PJAX
			{
				headers.push(("Vary".to_owned(), "content-encoding, x-pjax".to_owned()))
			}
			else
			{
				headers.push(("Vary".to_owned(), "content-encoding".to_owned()))
			}
		}
		if maximumAge == 0
		{
			headers.push(("Cache-Control".to_owned(), "no-cache".to_owned()))
		}
		else
		{
			headers.push(("Cache-Control".to_owned(), format!("max-age={}; no-transform; immutable", maximumAge)))
		}
		if isDownloadable
		{
			headers.push(("Content-Disposition".to_owned(), "attachment".to_owned()))
		}
		
		let (ourLanguage, otherLanguages) = match languageData
		{
			None => (None, None),
			Some((iso_639_1_alpha_2_language_code, language)) =>
			{
				headers.push(("Content-Language".to_owned(), iso_639_1_alpha_2_language_code.to_owned()));
				
				let mut ourLanguage = HashMap::with_capacity(2);
				ourLanguage.insert("iso_639_1_alpha_2_language_code", iso_639_1_alpha_2_language_code);
				ourLanguage.insert("iso_3166_1_alpha_2_country_code", language.iso_3166_1_alpha_2_country_code());
				(Some(ourLanguage), Some(localization.otherLanguages(iso_639_1_alpha_2_language_code)))
			}
		};
		
		for (headerName, headerTemplate) in self.headers.iter()
		{
			let json = &json!
			({
				"environment": environment,
				"variant": variant,
				"variant_path_with_trailing_slash": variant.pathWithTrailingSlash(),
				"our_language": ourLanguage,
				"localization": localization,
				"other_languages": otherLanguages,
				"can_be_compressed": canBeCompressed,
				"deployment_version": deploymentVersion,
				
				"header": headerName,
			});
			
			// SourceMap Header (for non-production)
			// Use deploymentVersion for unique URLs for css, etc
			
			let reg = Handlebars::new();
			let headerValue = reg.template_render(headerTemplate, &json)?;
			headers.push((headerName.to_owned(), headerValue));
		}
		
		headers.shrink_to_fit();
		Ok(headers)
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
	fn url(&self, language: &language, variant: Variant, version: Option<&str>) -> Result<Url, CordialError>
	{
		let baseUrl = language.baseUrl()?;
		
		let urlWithAmpOrPjaxPath = variant.appendToUrl(baseUrl);
		
		let mut url = urlWithAmpOrPjaxPath.join(&self.resourceOutputRelativeUrl).context(format!("Invalid resourceOutputRelativeUrl '{}'", self.resourceOutputRelativeUrl))?;
		if let Some(version) = version
		{
			url.set_query(Some(&format!("v={}", version)))
		}
		Ok(url)
	}
}
