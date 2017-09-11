// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub struct resource
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
	pub fn createOutput(&self, environment: &str, iso_639_1_alpha_2_language_code: &str, language: &language, localization: &localization, siteOutputFolderPath: &Path, inputFolderPath: &Path, httpsHandler: &mut HttpsStaticRequestHandler, deploymentVersion: &str) -> Result<(), CordialError>
	{
		let (maximumAge, isForPrimaryLanguageOnly, isForCanonicalUrlOnly, canBeCompressed, contentType, isDownloadable) = self.pipeline.isFor();
		
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
			let url = self.url(language, variant)?;
			let outputFilePath = self.outputContentFilePath(siteOutputFolderPath,language, variant)?;
			let regularHeaders = self.generateHeaders(environment, languageData, variant, deploymentVersion, localization, canBeCompressed, maximumAge, isDownloadable)?;
			let regularBody = self.pipeline.execute(&inputContentFilePath, variant, outputFilePath, inputFolderPath)?;
			
			let regularCompressed = if canBeCompressed
			{
				Some(self.compression.compress(&regularBody)?)
			}
			else
			{
				None
			};
			
			httpsHandler.addResource(url, RegularAndPjaxStaticResponse
			{
				regular: StaticResponse::new(StatusCode::Ok, contentType.clone(), regularHeaders, regularBody, regularCompressed),
				pjax: None,
			});
		}
		else
		{
			let url = self.url(language, Variant::Canonical)?;
			
			let variant = Variant::Canonical;
			let outputFilePath = self.outputContentFilePath(siteOutputFolderPath,language, variant)?;
			let regularHeaders = self.generateHeaders(environment, languageData, variant, deploymentVersion, localization, canBeCompressed, maximumAge, isDownloadable)?;
			let regularBody = self.pipeline.execute(&inputContentFilePath, variant, outputFilePath, inputFolderPath)?;
			
			let variant = Variant::PJAX;
			let outputFilePath = self.outputContentFilePath(siteOutputFolderPath,language, variant)?;
			let pjaxHeaders = self.generateHeaders(environment, languageData, variant, deploymentVersion, localization, canBeCompressed, maximumAge, isDownloadable)?;
			let pjaxBody = self.pipeline.execute(&inputContentFilePath, variant, outputFilePath, inputFolderPath)?;
			
			let (regularCompressed, pjaxCompressed) = if canBeCompressed
			{
				(Some(self.compression.compress(&regularBody)?), Some(self.compression.compress(&pjaxBody)?))
			}
			else
			{
				(None, None)
			};
			
			httpsHandler.addResource(url, RegularAndPjaxStaticResponse
			{
				regular: StaticResponse::new(StatusCode::Ok, contentType.clone(), regularHeaders, regularBody, regularCompressed),
				pjax: Some(StaticResponse::new(StatusCode::Ok, contentType.clone(), pjaxHeaders, pjaxBody, pjaxCompressed)),
			});
			
			
			let url = self.url(language, variant)?;
			let variant = Variant::AMP;
			let outputFilePath = self.outputContentFilePath(siteOutputFolderPath,language, variant)?;
			let ampHeaders = self.generateHeaders(environment, languageData, variant, deploymentVersion, localization, canBeCompressed, maximumAge, isDownloadable)?;
			let ampBody = self.pipeline.execute(&inputContentFilePath, variant, outputFilePath, inputFolderPath)?;
			let ampCompressed = if canBeCompressed
			{
				Some(self.compression.compress(&ampBody)?)
			}
			else
			{
				None
			};
			
			httpsHandler.addResource(url, RegularAndPjaxStaticResponse
			{
				regular: StaticResponse::new(StatusCode::Ok, contentType, ampHeaders, ampBody, ampCompressed),
				pjax: None,
			});
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
	fn outputContentFilePath(&self, siteOutputFolderPath: &Path, language: &language, variant: Variant) -> Result<PathBuf, CordialError>
	{
		let relativeOutputContentFilePath = self.relativeOutputContentFilePath(language, variant)?;
		let outputFilePath = siteOutputFolderPath.join(relativeOutputContentFilePath);
		{
			let outputParentFolderPath = outputFilePath.parent().unwrap();
			outputParentFolderPath.createFolder().context(outputParentFolderPath)?;
		}
		Ok(outputFilePath)
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
			
			// Provide a shortcode for Link: header
			// Provide a shortcode for Content-Disposition: header
			// Provide whatever is needed for 30x redirects: Location
			
			// SourceMap Header (for non-production)
			// Use deploymentVersion for unique URLs for css, etc
			
			//
			// Do not generate Expires, Last-Modified headers; just use etag & Cache-Control
			// Cache-Control
				// See https://developers.google.com/web/fundamentals/performance/optimizing-content-efficiency/http-caching
				// HTML pages should be Cache-Control: no-cache and rely on ETag
				// All others should use commonCacheControlHeader(X) where X is a year
				// We should version URLs, perhaps as http://example.com/assets/style.ABC.css, where ABC is a version or timestamp, perhaps Last-Modified
				// Versioned assets should also have a (temporary) redirect for the unversioned form
			
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
		resourceRelativePathString.push_str(variant.fileExtensionWithLeadingPeriod());
		Ok(PathBuf::from(resourceRelativePathString))
	}
}
