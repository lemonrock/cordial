// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum pipeline
{
	md
	{
		#[serde(default = "pipeline::max_age_in_seconds_none_default")] max_age_in_seconds: u32,
		#[serde(default)] is_leaf: bool,
		
		abstracts: HashMap<String, Abstract>,
		/*
		{
			en:
			{
				title: String,
				description: String,
				extract: String, // markdown / handlebars template
			}
		}
		*/
	},
	raster_image
	{
		#[serde(default = "pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		input_format: InputImageFormat,
		jpeg_quality: Option<u8>,
		#[serde(default)] transformations: Vec<ImageTransformation>,
		
		// img tag sizes and srcset
		
		// By language code. Used in alt tag
		descriptions: HashMap<String, ImageAbstract>,
		
		// eg  "(min-width: 36em) 33.3vw, 100vw"  from  https://ericportis.com/posts/2014/srcset-sizes/
		img_sizes: Option<String>,
		
		// Additional to original image
		img_srcset: Vec<ImageSourceSetEntry>,
		
		#[serde(default, skip_deserializing)] primary_image_dimensions: (u32, u32),
		#[serde(default, skip_deserializing)] image_source_set: Vec<(Url, u32)>,
	},
	sass
	{
		#[serde(default = "pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		#[serde(default = "pipeline::cssDefaultPrecision")] precision: u8,
	},
	scss
	{
		#[serde(default = "pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		#[serde(default = "pipeline::cssDefaultPrecision")] precision: u8,
	},
	svg
	{
		#[serde(default = "pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		do_not_optimize: bool, // Exists solely because of potential bugs in svg optimizer
		
		// By language code. Used in alt tag
		descriptions: HashMap<String, ImageAbstract>,
	}
//	sitemap, // xml
//	robots,
//	rss, // xml
//	json,
//	// js,
//	gif (animations only),
//  favicon
//	redirect,
	// eg for temp or perm redirect
	// empty body
	// Needs access to site configuration in order to write out the 'Location' header
	// header field values are visible US-ASCII, ie 32 - 126 incl
}

impl pipeline
{
	fn cssDefaultPrecision() -> u8
	{
		5
	}
	
	#[inline(always)]
	fn max_age_in_seconds_none_default() -> u32
	{
		0
	}
	
	#[inline(always)]
	fn max_age_in_seconds_long_default() -> u32
	{
		31536000
	}
	
	#[inline(always)]
	fn is_downloadable_false_default() -> bool
	{
		false
	}
	
	#[inline(always)]
	fn is_versioned_true_default() -> bool
	{
		true
	}
	
	#[inline(always)]
	pub(crate) fn processingPriority(&self) -> ProcessingPriority
	{
		use self::pipeline::*;
		use self::ProcessingPriority::*;
		match *self
		{
			md { .. } => LinksToSubResourcesEgHtmlPage,
			raster_image { .. } => NoDependenciesEgImage,
			sass { .. } => DependsOnOthersEgStylesheet,
			scss { .. } => DependsOnOthersEgStylesheet,
			svg { .. } => NoDependenciesEgImage,
		}
	}
	
	#[inline(always)]
	pub(crate) fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		let mut result = Vec::with_capacity(4);
		
		use self::pipeline::*;
		match *self
		{
			md { ..} =>
			{
				for fileExtension in vec![".markdown", ".md"]
				{
					let mut withExtension = String::with_capacity(resourceInputName.len() + 3);
					withExtension.push_str(resourceInputName);
					withExtension.push_str(fileExtension);
					result.push(withExtension);
				}
			}
			raster_image { input_format, .. } =>
			{
				let first = resourceInputName.rmatch_indices(".").next();
				
				for fileExtension in input_format.fileExtensions()
				{
					let index = first.unwrap().0;
					let mut withExtension = String::with_capacity(index + fileExtension.len());
					let slice = if first.is_some()
					{
						&resourceInputName[0..index]
					}
					else
					{
						resourceInputName
					};
					withExtension.push_str(slice);
					withExtension.push_str(fileExtension);
					
					result.push(withExtension);
				}
			}
			
			_ => result.push(resourceInputName.to_owned()),
		}
		
		result
	}
	
	#[inline(always)]
	pub(crate) fn resourceOutputRelativeUrl(&self, parentHierarchy: &[String], resourceInputName: &str) -> String
	{
		let mut relativeUrl = String::with_capacity(1024);
		for parent in parentHierarchy
		{
			relativeUrl.push_str(parent);
			relativeUrl.push('/');
		}
		
		use self::pipeline::*;
		match *self
		{
			md { is_leaf, .. } =>
			{
				relativeUrl.push_str(resourceInputName);
				
				if !is_leaf
				{
					relativeUrl.push('/');
				}
			}
			raster_image { jpeg_quality, .. } =>
			{
				let first = resourceInputName.rmatch_indices(".").next();
				let index = first.unwrap().0;
				
				let slice = if first.is_some()
				{
					&resourceInputName[0..index]
				}
				else
				{
					resourceInputName
				};
				relativeUrl.push_str(slice);
				
				let fileExtension = if jpeg_quality.is_none()
				{
					".png"
				}
				else
				{
					".jpg"
				};
				relativeUrl.push_str(fileExtension);
			}
			_ =>
			{
				relativeUrl.push_str(resourceInputName);
			}
		};
		
		relativeUrl
	}
	
	#[inline(always)]
	pub(crate) fn is<'a>(&self) -> (bool, bool)
	{
		use self::pipeline::*;
		match *self
		{
			md { .. } => (false, false),
			raster_image { is_versioned, language_aware, .. } => (language_aware, is_versioned),
			sass { is_versioned, language_aware, .. } => (language_aware, is_versioned),
			scss { is_versioned, language_aware, .. } => (language_aware, is_versioned),
			svg { is_versioned, language_aware, .. } => (language_aware, is_versioned),
		}
	}
	
	#[inline(always)]
	pub(crate) fn execute(&mut self, inputContentFilePath: &Path, unversionedCanonicalUrl: Url, headerTemplates: &HashMap<String, String>, languageData: Option<(&str, &language)>, configuration: &Configuration) -> Result<Vec<(Url, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		let mut canBeCompressed = true;
		
		use self::pipeline::*;
		match self
		{
			&mut md { max_age_in_seconds: _, .. } =>
			{
//				let mut result = Vec::with_capacity(2);
//
//				let regularHeaders = Self::generateHeaders(headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, false)?;
//				let pjaxHeaders = Self::generateHeaders(headerTemplates, languageData, HtmlVariant::PJAX, configuration, canBeCompressed, max_age_in_seconds, false)?;
//				//result.push((unversionedUrl, ContentType::html(), regularHeaders, regularBody, Some((pjaxHeaders, pjaxBody)), canBeCompressed));
//
//				let ampHeaders = Self::generateHeaders(headerTemplates, languageData, HtmlVariant::AMP, deploymentVersion, localization, canBeCompressed, max_age_in_seconds, false)?;
//				// have to adjust unversionedUrl URL for amp
//
//
//				Ok(result)
				panic!("Implement me");
			}
			
			&mut raster_image { max_age_in_seconds, is_downloadable, input_format, jpeg_quality, ref transformations, ref img_srcset, ref mut primary_image_dimensions, ref mut image_source_set, .. } =>
			{
				canBeCompressed = false;
				
				let headerGenerator = |url, canBeCompressed|
				{
					Self::generateHeaders(headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, url)
				};
				
				let (dimensions, imageSourceSet, result) = Self::raster_image(inputContentFilePath, unversionedCanonicalUrl, headerGenerator, canBeCompressed, input_format, jpeg_quality, transformations, img_srcset)?;
				*primary_image_dimensions = dimensions;
				*image_source_set = imageSourceSet;
				Ok(result)
			},
			
			&mut sass { max_age_in_seconds, is_downloadable, precision, .. } =>
			{
				let headers = Self::generateHeaders(headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, &unversionedCanonicalUrl)?;
				let inputFolderPath = &configuration.inputFolderPath;
				let body = Self::sass_or_scss(inputContentFilePath, precision, inputFolderPath, true)?;
				Ok(vec![(unversionedCanonicalUrl, ContentType(TEXT_CSS), headers, body, None, canBeCompressed)])
			}
			
			&mut scss { max_age_in_seconds, is_downloadable, precision, .. } =>
			{
				let headers = Self::generateHeaders(headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, &unversionedCanonicalUrl)?;
				let inputFolderPath = &configuration.inputFolderPath;
				let body = Self::sass_or_scss(inputContentFilePath, precision, inputFolderPath, false)?;
				Ok(vec![(unversionedCanonicalUrl, ContentType(TEXT_CSS), headers, body, None, canBeCompressed)])
			}
			
			&mut svg { max_age_in_seconds, is_downloadable, do_not_optimize, .. } =>
			{
				let headers = Self::generateHeaders(headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, &unversionedCanonicalUrl)?;
				let body = if do_not_optimize
				{
					inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?
				}
				else
				{
					inputContentFilePath.fileContentsAsACleanedSvgFrom()?
				};
				Ok(vec![(unversionedCanonicalUrl, ContentType(Mime::from_str("image/svg+xml").unwrap()), headers, body, None, canBeCompressed)])
			}
		}
	}
	
	// Primary body; secondary bodies by file-name-variant
	fn raster_image<F: Fn(&Url, bool) -> Result<Vec<(String, String)>, CordialError>>(inputContentFilePath: &Path, unversionedUrl: Url, headerGenerator: F, canBeCompressed: bool, input_format: InputImageFormat, jpeg_quality: Option<u8>, transformations: &[ImageTransformation], img_srcset: &[ImageSourceSetEntry]) -> Result<((u32, u32), Vec<(Url, u32)>, Vec<(Url, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>), CordialError>
	{
		let imageBeforeTransformation = inputContentFilePath.fileContentsAsImage(input_format)?;
		
		// transform
		let imageAfterTransformation = ImageTransformation::applyTransformations(imageBeforeTransformation, transformations)?;
		
		// generate image src set
		let mut imageSourceSet = ImageSourceSet::new(inputContentFilePath, unversionedUrl, jpeg_quality, imageAfterTransformation);
		imageSourceSet.generate(img_srcset)?;
		
		let primaryImageDimensions = imageSourceSet.primaryImageDimensions();
		let processedImageSourceSet = imageSourceSet.processedImageSourceSet();
		let urls = imageSourceSet.urls(headerGenerator, canBeCompressed)?;
		
		Ok((primaryImageDimensions, processedImageSourceSet, urls))
	}
	
	fn sass_or_scss(inputContentFilePath: &Path, precision: u8, inputFolderPath: &Path, isSass: bool) -> Result<Vec<u8>, CordialError>
	{
		fn findImportPaths(sassFolderPath: &Path) -> Result<Vec<String>, CordialError>
		{
			let mut importPaths = Vec::with_capacity(16);
			let sassImportsPath = sassFolderPath.join("sass-imports");
			for entry in sassImportsPath.read_dir().context(&sassImportsPath)?
			{
				let entry = entry.context(&sassImportsPath)?;
				
				let path = entry.path();
				
				if entry.file_type().context(&path)?.is_dir()
				{
					match path.into_os_string().into_string()
					{
						Err(_) => return Err(CordialError::InvalidFile(entry.path(), "a component of the path is not valid UTF-8".to_owned())),
						Ok(importPath) => importPaths.push(importPath),
					}
				}
			}
			
			Ok(importPaths)
		}
		
		let options = ::sass_rs::Options
		{
			output_style: ::sass_rs::OutputStyle::Compressed,
			precision: precision as usize,
			indented_syntax: isSass,
			include_paths: findImportPaths(inputFolderPath)?,
		};
		
		match ::sass_rs::compile_file(inputContentFilePath, options)
		{
			Err(error) => return Err(CordialError::CouldNotCompileSass(inputContentFilePath.to_path_buf(), error)),
			Ok(css) => Ok(css.as_bytes().to_owned()),
		}
	}
	
	#[inline(always)]
	fn generateHeaders(headerTemplates: &HashMap<String, String>, languageData: Option<(&str, &language)>, htmlVariant: HtmlVariant, configuration: &Configuration, canBeCompressed: bool, maximumAge: u32, isDownloadable: bool, url: &Url) -> Result<Vec<(String, String)>, CordialError>
	{
		let localization = &configuration.localization;
		let deploymentVersion = &configuration.deploymentVersion;
		
		let mut headers = Vec::with_capacity(headerTemplates.len() * 2);
		
		let isPjax = htmlVariant == HtmlVariant::PJAX;
		
		let vary = if isPjax
		{
			headers.push(("X-PJAX-Version".to_owned(), format!("{}", deploymentVersion)));
			
			if canBeCompressed
			{
				Some("content-encoding, x-pjax")
			}
			else
			{
				Some("x-pjax")
			}
		}
		else
		{
			if canBeCompressed
			{
				Some("content-encoding")
			}
			else
			{
				None
			}
		};
		if let Some(vary) = vary
		{
			headers.push(("Vary".to_owned(), vary.to_owned()));
		}
		
		if maximumAge == 0
		{
			headers.push(("Cache-Control".to_owned(), "no-cache".to_owned()))
		}
		else
		{
			headers.push(("Cache-Control".to_owned(), format!("max-age={}; no-transform; immutable", maximumAge)))
		}
		
		let fileNameUtf8 = url.fileNameOrIndexNamePercentDecodedUntrusted(".html").to_owned();
		let variant = if isDownloadable
		{
			"attachment"
		}
		else
		{
			"inline"
		};
		headers.push(("Content-Disposition".to_owned(), format!("{}; filename*=utf-8''{}", variant, utf8_percent_encode(&fileNameUtf8, USERINFO_ENCODE_SET))));
		
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
		
		for (headerName, headerTemplate) in headerTemplates.iter()
		{
			let json = &json!
			({
				"environment": &configuration.environment,
				"html_variant": htmlVariant,
				"variant_path_with_trailing_slash": htmlVariant.pathWithTrailingSlash(),
				"our_language": ourLanguage,
				"localization": localization,
				"other_languages": otherLanguages,
				"can_be_compressed": canBeCompressed,
				"deployment_date": configuration.deploymentDate,
				"deployment_version": deploymentVersion,
				
				"header": headerName,
			});
			
			let reg = Handlebars::new();
			let headerValue = reg.template_render(headerTemplate, &json)?;
			if !headerName.is_ascii()
			{
				return Err(CordialError::Configuration(format!("Non-ASCII header name '{}' for {}", headerName, url)))
			}
			if !headerValue.is_ascii()
			{
				return Err(CordialError::Configuration(format!("Non-ASCII header value '{}' for header name '{}' for {}", headerValue, headerName, url)))
			}
			headers.push((headerName.to_owned(), headerValue));
		}
		
		headers.shrink_to_fit();
		Ok(headers)
	}
}
