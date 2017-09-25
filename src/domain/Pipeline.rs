// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum Pipeline
{
	raw
	{
		#[serde(default = "Pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "Pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "Pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		#[serde(default)] can_be_compressed: Option<bool>, // default is to use filename
		#[serde(default)] mime_type: Option<String>, // default is to use filename, and sniff text formats, with US-ASCII interpreted as UTF-8
	},
	
	html
	{
		#[serde(default = "Pipeline::max_age_in_seconds_none_default")] max_age_in_seconds: u32,
		#[serde(default)] input_format: HtmlInputFormat,
		
		#[serde(default)] is_leaf: bool,
		
		// open graph, RSS, schema.org
		publication_date: SystemTime,
		
		// modification_date - used by open graph, schema.org. should be a list of changes, with changes detailed in all languages. Not the same as HTTP last-modified date.
		// empty modifications imply use of publication date
		#[serde(default)] modifications: BTreeMap<SystemTime, HashMap<String, String>>,
		
		// open graph
		#[serde(default)] expiration_date: Option<SystemTime>,
		
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
	
	font
	{
		#[serde(default = "Pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "Pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "Pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		#[serde(default)] input_format: Option<FontInputFormat>,
		
		#[serde(default)] utf8_xml_metadata: Vec<u8>,
		#[serde(default)] woff1_private_data: Vec<u8>,
		#[serde(default = "Pipeline::woff1_iterations_default")] woff1_iterations: u16,
		#[serde(default = "Pipeline::woff2_brotli_quality_default")] woff2_brotli_quality: u8,
		#[serde(default)] woff2_disallow_transforms: bool,
		#[serde(default)] include_ttf: bool,
	},
	
	raster_image
	{
		#[serde(default = "Pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "Pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "Pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		#[serde(default)] input_format: Option<ImageInputFormat>,
		#[serde(default)] jpeg_quality: Option<u8>,
		#[serde(default)] jpeg_speed_over_compression: bool,
		#[serde(default)] transformations: Vec<ImageTransformation>,
		
		// eg  "(min-width: 36em) 33.3vw, 100vw"  from  https://ericportis.com/posts/2014/srcset-sizes/
		img_sizes: Option<String>,
		
		// Additional to original image
		#[serde(default)] img_srcset: Vec<ImageSourceSetEntry>,
		
		#[serde(default, skip_deserializing)] primary_image_dimensions: (u32, u32),
		#[serde(default, skip_deserializing)] image_source_set: Vec<(Url, u32)>,
	},
	
	css
	{
		#[serde(default = "Pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "Pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "Pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		#[serde(default)] input_format: Option<CssInputFormat>,
		#[serde(default = "Pipeline::precision_default")] precision: u8,
		#[serde(default)] is_template: bool,
	},
	
	svg
	{
		#[serde(default = "Pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "Pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "Pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		do_not_optimize: bool, // Exists solely because of potential bugs in svg optimizer
	
		// TODO: Add option to add height, width if missing
		// TODO: Add alternative output formats, eg ICO and PNG, with multiple sizes
	},
	
	engiffen
	{
		#[serde(default = "Pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "Pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "Pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		#[serde(default)] input_format: Option<ImageInputFormat>,
		
		// eg  "(min-width: 36em) 33.3vw, 100vw"  from  https://ericportis.com/posts/2014/srcset-sizes/
		img_sizes: Option<String>,
		
		#[serde(default)] source_set: Vec<EngiffenSource>,
		#[serde(default)] quantizer: EngiffenQuantizer,
		#[serde(default)] loops: EngiffenLoops,
	}


	
//	json,
//	// js,
//  favicon
//	redirect,
	// eg for temp or perm redirect
	// empty body
	// Needs access to site configuration in order to write out the 'Location' header
	// header field values are visible US-ASCII, ie 32 - 126 incl
}

impl Default for Pipeline
{
	#[inline(always)]
	fn default() -> Self
	{
		Pipeline::raw
		{
			max_age_in_seconds: Self::max_age_in_seconds_long_default(),
			is_downloadable: Self::is_downloadable_false_default(),
			is_versioned: Self::is_versioned_true_default(),
			language_aware: false,
			can_be_compressed: None,
			mime_type: None,
		}
	}
}

impl Pipeline
{
	#[inline(always)]
	pub(crate) fn processingPriority(&self) -> ProcessingPriority
	{
		use self::Pipeline::*;
		use self::ProcessingPriority::*;
		match *self
		{
			raw { .. } => NoDependenciesEgImage,
			html { .. } => LinksToSubResourcesEgHtmlPage,
			font { .. } => NoDependenciesEgImage,
			raster_image { .. } => NoDependenciesEgImage,
			css { .. } => DependsOnOthersEgStylesheet,
			svg { .. } => NoDependenciesEgImage,
			engiffen { .. } => NoDependenciesEgImage,
		}
	}
	
	#[inline(always)]
	pub(crate) fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		use self::Pipeline::*;
		
		match *self
		{
			html { input_format, .. } => input_format.resourceInputContentFileNamesWithExtension(resourceInputName),
			
			font { input_format, .. } => input_format.resourceInputContentFileNamesWithExtension(resourceInputName),
			
			raster_image { input_format, .. } => input_format.resourceInputContentFileNamesWithExtension(resourceInputName),
			
			css { input_format, .. } => input_format.resourceInputContentFileNamesWithExtension(resourceInputName),
			
			_ => vec![resourceInputName.to_owned()],
		}
	}
	
	#[inline(always)]
	pub(crate) fn is<'a>(&self) -> (bool, bool)
	{
		use self::Pipeline::*;
		
		match *self
		{
			raw { is_versioned, language_aware, .. } => (is_versioned, language_aware),
			html { .. } => (false, true),
			font { is_versioned, language_aware, .. } => (is_versioned, language_aware),
			raster_image { is_versioned, language_aware, .. } => (is_versioned, language_aware),
			css { is_versioned, language_aware, .. } => (is_versioned, language_aware),
			svg { is_versioned, language_aware, .. } => (is_versioned, language_aware),
			engiffen { is_versioned, language_aware, .. } => (is_versioned, language_aware),
		}
	}
	
	#[inline(always)]
	pub(crate) fn execute(&mut self, inputContentFilePath: &Path, resourceRelativeUrl: &str, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, siteMapWebPages: &mut Vec<SiteMapWebPage>, rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		use self::Pipeline::*;
		
		use self::UrlTag::*;
		
		match self
		{
			&mut raw { max_age_in_seconds, is_downloadable, can_be_compressed, ref mime_type, .. } =>
			{
				let inputCanonicalUrl = languageData.url(resourceRelativeUrl)?;
				
				let canBeCompressed = if can_be_compressed.is_none()
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
				
				let headers = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, &inputCanonicalUrl)?;
				let body = inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?;
				Ok(vec![(inputCanonicalUrl, hashmap! { default => Rc::new(JsonValue::Null) }, ContentType(mimeType), headers, body, None, canBeCompressed)])
			}
			
			&mut html { max_age_in_seconds, is_leaf, .. } =>
			{
				const CanBeCompressed: bool = true;
				
				let inputCanonicalUrl = if is_leaf
				{
					let mut leafPath = String::with_capacity(resourceRelativeUrl.len() + 1);
					leafPath.push_str(resourceRelativeUrl);
					leafPath.push('/');
					languageData.url(&leafPath)?
				}
				else
				{
					languageData.url(resourceRelativeUrl)?
				};
				
//				let mut result = Vec::with_capacity(2);
//
//				let regularHeaders = generateHeaders(headerTemplates, languageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, false)?;
//				let pjaxHeaders = generateHeaders(headerTemplates, languageData, HtmlVariant::PJAX, configuration, canBeCompressed, max_age_in_seconds, false)?;
//				//result.push((unversionedUrl, ContentType::html(), regularHeaders, regularBody, Some((pjaxHeaders, pjaxBody)), CanBeCompressed));
//
//				let ampHeaders = generateHeaders(headerTemplates, languageData, HtmlVariant::AMP, deploymentVersion, localization, CanBeCompressed, max_age_in_seconds, false)?;
//				// have to adjust unversionedUrl URL for amp
//
//				Add to WebPageSiteMaps; detect videos and images  (see https://developers.google.com/webmasters/videosearch/sitemaps)
				// Supporting video: https://www.html5rocks.com/en/tutorials/video/basics/

				//
				
				//let synopsisHtml = markdown_to_html(&rssItemLanguageVariant.webPageSynopsisMarkdown, markdownOptions);
				
//				Ok(result)
				panic!("Implement me");
			}
			
			&mut font { max_age_in_seconds, is_downloadable, ref utf8_xml_metadata, ref woff1_private_data, woff1_iterations, woff2_brotli_quality, woff2_disallow_transforms, include_ttf, input_format, .. } =>
			{
				FontInputFormat::toWebFonts(input_format, resourceRelativeUrl, configuration, inputContentFilePath, handlebars, headerTemplates, ifLanguageAwareLanguageData, languageData, max_age_in_seconds, is_downloadable, &utf8_xml_metadata[..], &woff1_private_data[..], woff1_iterations, woff2_brotli_quality, woff2_disallow_transforms, include_ttf)
			}
			
			&mut raster_image { max_age_in_seconds, is_downloadable, input_format, jpeg_quality, jpeg_speed_over_compression, ref transformations, ref img_srcset, ref mut primary_image_dimensions, ref mut image_source_set, .. } =>
			{
				const CanNotBeCompressed: bool = false;
				
				let (dimensions, imageSourceSet, result) = Self::raster_image(inputContentFilePath, Self::withoutFileNameExtension(resourceRelativeUrl), languageData, input_format, jpeg_quality, jpeg_speed_over_compression, transformations, img_srcset, |url| generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanNotBeCompressed, max_age_in_seconds, is_downloadable, url))?;
				*primary_image_dimensions = dimensions;
				*image_source_set = imageSourceSet;
				Ok(result)
			}
			
			&mut css { max_age_in_seconds, is_downloadable, input_format, precision, is_template, .. } =>
			{
				const CanBeCompressed: bool = true;
				
				let url = languageData.url(&Self::replaceFileNameExtension(resourceRelativeUrl, ".css"))?;
				
				let headers = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanBeCompressed, max_age_in_seconds, is_downloadable, &url)?;
				
				let handlebars = if is_template
				{
					Some(handlebars)
				}
				else
				{
					None
				};
				let body = CssInputFormat::toCss(input_format, inputContentFilePath, precision, configuration, handlebars)?;
				
				Ok(vec![(url, hashmap! { default => Rc::new(JsonValue::Null) }, ContentType(TEXT_CSS), headers, body, None, CanBeCompressed)])
			}
			
			&mut svg { max_age_in_seconds, is_downloadable, do_not_optimize, .. } =>
			{
				const CanBeCompressed: bool = true;
				
				let url = languageData.url(&Self::replaceFileNameExtension(resourceRelativeUrl, ".svg"))?;
				
				let headers = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanBeCompressed, max_age_in_seconds, is_downloadable, &url)?;
				let body = if do_not_optimize
				{
					inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?
				}
				else
				{
					inputContentFilePath.fileContentsAsACleanedSvgFrom()?
				};
				Ok(vec![(url, hashmap! { default => Rc::new(JsonValue::Null) }, ContentType(Self::mimeType("image/svg+xml")), headers, body, None, CanBeCompressed)])
			}
			
			&mut engiffen { max_age_in_seconds, is_downloadable, input_format, ref source_set, ref quantizer, loops, .. } =>
			{
				const CanNotBeCompressed: bool = false;
				
				let engiffenPipeline = Engiffen::new(inputContentFilePath, source_set, quantizer, loops, input_format);
				engiffenPipeline.process(Self::withoutFileNameExtension(resourceRelativeUrl), languageData, |url| generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanNotBeCompressed, max_age_in_seconds, is_downloadable, url))
			}
		}
	}
	
	#[inline(always)]
	fn mimeType(string: &str) -> Mime
	{
		string.parse().unwrap()
	}
	
	#[inline(always)]
	fn raster_image<'a, F: for<'r> FnMut(&'r Url) -> Result<Vec<(String, String)>, CordialError>>(inputContentFilePath: &Path, resourceRelativeUrlWithoutFileNameExtension: &str, languageData: &'a LanguageData, imageInputFormat: Option<ImageInputFormat>, jpegQuality: Option<u8>, jpegSpeedOverCompression: bool, transformations: &[ImageTransformation], imageSourceSetEntries: &[ImageSourceSetEntry], headerGenerator: F) -> Result<((u32, u32), Vec<(Url, u32)>, Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>), CordialError>
	{
		// load original
		let mut imageBeforeTransformation = match ImageInputFormat::load(imageInputFormat, inputContentFilePath)
		{
			Some(result) =>
			{
				match result
				{
					Err(error) => return Err(error),
					Ok(image) => image,
				}
			},
			None => panic!("Should not be possible"),
		};
		
		// transform
		let imageAfterTransformation = if let Some(transformedImage) = ImageTransformation::applyTransformations(&mut imageBeforeTransformation, transformations)?
		{
			transformedImage
		}
		else
		{
			imageBeforeTransformation
		};
		
		// generate image src set
		let mut imageSourceSet = ImageSourceSet::new(inputContentFilePath, resourceRelativeUrlWithoutFileNameExtension, jpegQuality, jpegSpeedOverCompression, imageAfterTransformation, languageData);
		imageSourceSet.generate(imageSourceSetEntries)?;
		
		let primaryImageDimensions = imageSourceSet.primaryImageDimensions();
		let processedImageSourceSet = imageSourceSet.processedImageSourceSet()?;
		let urls = imageSourceSet.urls(headerGenerator)?;
		
		Ok((primaryImageDimensions, processedImageSourceSet, urls))
	}
	
	#[inline(always)]
	fn withoutFileNameExtension<'a>(resourceRelativeUrl: &'a str) -> &'a str
	{
		match resourceRelativeUrl.rfind('.')
		{
			None => resourceRelativeUrl,
			Some(index) => resourceRelativeUrl.split_at(index).0,
		}
	}
	
	#[inline(always)]
	fn appendFileNameExtension<'a>(withoutFileNameExtension: &str, extension: &str) -> String
	{
		let mut string = String::with_capacity(withoutFileNameExtension.len() + extension.len());
		string.push_str(withoutFileNameExtension.as_ref());
		string.push_str(extension);
		string
	}
	
	#[inline(always)]
	fn replaceFileNameExtension(resourceRelativeUrl: &str, extension: &str) -> String
	{
		Self::appendFileNameExtension(Self::withoutFileNameExtension(resourceRelativeUrl), extension)
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
	
	#[inline(always)]
	fn woff1_iterations_default() -> u16
	{
		DefaultNumberOfIterations
	}
	
	#[inline(always)]
	fn woff2_brotli_quality_default() -> u8
	{
		11
	}
}
