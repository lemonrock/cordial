// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum pipeline
{
	raw
	{
		#[serde(default = "pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		can_be_compressed: Option<bool>, // default is to use filename
		mime_type: Option<String>, // default is to use filename, and sniff text formats, with US-ASCII interpreted as UTF-8
	},
	
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
		#[serde(default = "pipeline::precision_default")] precision: u8,
		#[serde(default)] is_template: bool,
	},
	
	scss
	{
		#[serde(default = "pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		#[serde(default = "pipeline::precision_default")] precision: u8,
		#[serde(default)] is_template: bool,
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
	#[inline(always)]
	pub(crate) fn processingPriority(&self) -> ProcessingPriority
	{
		use self::pipeline::*;
		use self::ProcessingPriority::*;
		match *self
		{
			raw { .. } => NoDependenciesEgImage,
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
			raw { is_versioned, language_aware, .. } => (language_aware, is_versioned),
			md { .. } => (false, false),
			raster_image { is_versioned, language_aware, .. } => (language_aware, is_versioned),
			sass { is_versioned, language_aware, .. } => (language_aware, is_versioned),
			scss { is_versioned, language_aware, .. } => (language_aware, is_versioned),
			svg { is_versioned, language_aware, .. } => (language_aware, is_versioned),
		}
	}
	
	#[inline(always)]
	pub(crate) fn execute(&mut self, inputContentFilePath: &Path, unversionedCanonicalUrl: Url, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: Option<(&str, &language)>, configuration: &Configuration, siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<(Url, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		let mut canBeCompressed = true;
		
		use self::pipeline::*;
		match self
		{
			&mut raw { max_age_in_seconds, is_downloadable, can_be_compressed, ref mime_type, .. } =>
			{
				canBeCompressed = if can_be_compressed.is_none()
				{
					!inputContentFilePath.hasCompressedFileExtension()?
				}
				else
				{
					can_be_compressed.unwrap()
				};
				
				let mimeType = if mime_type.is_none()
				{
					inputContentFilePath.guessMimeTypeWithCharacterSet()?
				}
				else
				{
					match mime_type.as_ref().unwrap().parse()
					{
						Err(error) => return Err(CordialError::Configuration(format!("Could not parse mime type '{:?}' because {:?} for {:?}", mime_type, error, inputContentFilePath))),
						Ok(mime) => mime,
					}
				};
				
				let headers = generateHeaders(handlebars, headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, &unversionedCanonicalUrl)?;
				let body = inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?;
				Ok(vec![(unversionedCanonicalUrl, ContentType(mimeType), headers, body, None, canBeCompressed)])
			}
			
			&mut md { max_age_in_seconds: _, .. } =>
			{
//				let mut result = Vec::with_capacity(2);
//
//				let regularHeaders = generateHeaders(headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, false)?;
//				let pjaxHeaders = generateHeaders(headerTemplates, languageData, HtmlVariant::PJAX, configuration, canBeCompressed, max_age_in_seconds, false)?;
//				//result.push((unversionedUrl, ContentType::html(), regularHeaders, regularBody, Some((pjaxHeaders, pjaxBody)), canBeCompressed));
//
//				let ampHeaders = generateHeaders(headerTemplates, languageData, HtmlVariant::AMP, deploymentVersion, localization, canBeCompressed, max_age_in_seconds, false)?;
//				// have to adjust unversionedUrl URL for amp
//
//				Add to WebPageSiteMaps; detect videos and images  (see https://developers.google.com/webmasters/videosearch/sitemaps)
				// Supporting video: https://www.html5rocks.com/en/tutorials/video/basics/

//				Ok(result)
				panic!("Implement me");
			}
			
			&mut raster_image { max_age_in_seconds, is_downloadable, input_format, jpeg_quality, ref transformations, ref img_srcset, ref mut primary_image_dimensions, ref mut image_source_set, .. } =>
			{
				canBeCompressed = false;
				
				let (dimensions, imageSourceSet, result) = Self::raster_image(inputContentFilePath, unversionedCanonicalUrl, canBeCompressed, input_format, jpeg_quality, transformations, img_srcset, |url, canBeCompressed|
				{
					generateHeaders(handlebars, headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, url)
				})?;
				*primary_image_dimensions = dimensions;
				*image_source_set = imageSourceSet;
				Ok(result)
			},
			
			&mut sass { max_age_in_seconds, is_downloadable, precision, is_template, .. } =>
			{
				let headers = generateHeaders(handlebars, headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, &unversionedCanonicalUrl)?;
				let inputFolderPath = &configuration.inputFolderPath;
				let body = Self::sass_or_scss(inputContentFilePath, precision, inputFolderPath, true)?;
				Ok(vec![(unversionedCanonicalUrl, ContentType(TEXT_CSS), headers, body, None, canBeCompressed)])
			}
			
			&mut scss { max_age_in_seconds, is_downloadable, precision, is_template, .. } =>
			{
				let headers = generateHeaders(handlebars, headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, &unversionedCanonicalUrl)?;
				let inputFolderPath = &configuration.inputFolderPath;
				let body = Self::sass_or_scss(inputContentFilePath, precision, is_template, inputFolderPath, false)?;
				Ok(vec![(unversionedCanonicalUrl, ContentType(TEXT_CSS), headers, body, None, canBeCompressed)])
			}
			
			&mut svg { max_age_in_seconds, is_downloadable, do_not_optimize, .. } =>
			{
				let headers = generateHeaders(handlebars, headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, &unversionedCanonicalUrl)?;
				let body = if do_not_optimize
				{
					inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?
				}
				else
				{
					inputContentFilePath.fileContentsAsACleanedSvgFrom()?
				};
				Ok(vec![(unversionedCanonicalUrl, ContentType("image/svg+xml".parse().unwrap()), headers, body, None, canBeCompressed)])
			}
		}
	}
	
	#[inline(always)]
	fn raster_image<F: for<'r> Fn(&'r Url, bool) -> Result<Vec<(String, String)>, CordialError>>(inputContentFilePath: &Path, unversionedUrl: Url, canBeCompressed: bool, input_format: InputImageFormat, jpeg_quality: Option<u8>, transformations: &[ImageTransformation], img_srcset: &[ImageSourceSetEntry], headerGenerator: F) -> Result<((u32, u32), Vec<(Url, u32)>, Vec<(Url, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>), CordialError>
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
	
	#[inline(always)]
	fn sass_or_scss(inputContentFilePath: &Path, precision: u8, preProcessWithHandlebars: bool, inputFolderPath: &Path, isSass: bool) -> Result<Vec<u8>, CordialError>
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
		
		
		let content = inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?;
		let sassInput = if preProcessWithHandlebars
		{
			handlebars.register_escape_fn(::handlerbars::no_escape);
			let sassInput = handlebars.template_render(&sassInput, &json)?;
			handlebars.unregister_escape_fn();
			sassInput
		}
		else
		{
			content
		};
		
		match ::sass_rs::compile_string(content, options)
		{
			Err(error) => return Err(CordialError::CouldNotCompileSass(inputContentFilePath.to_path_buf(), error)),
			Ok(css) => Ok(css.as_bytes().to_owned()),
		}
	}
	
	#[inline(always)]
	fn precision_default() -> u8
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
}
