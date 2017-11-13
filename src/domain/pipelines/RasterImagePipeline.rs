// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct RasterImagePipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] language_aware: bool,
	#[serde(default)] input_format: Option<ImageInputFormat>,
	
	#[serde(default)] jpeg_quality: Option<u8>,
	#[serde(default)] jpeg_speed_over_compression: bool,
	#[serde(default)] transformations: Vec<ImageTransformation>,
	// eg  "(min-width: 36em) 33.3vw, 100vw"  from  https://ericportis.com/posts/2014/srcset-sizes/
	#[serde(default)] img_sizes: Option<String>,
	// Additional to original image
	#[serde(default)] img_srcset: Vec<ImageSourceSetEntry>,
}

impl Default for RasterImagePipeline
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			max_age_in_seconds: max_age_in_seconds_long_default(),
			is_downloadable: is_downloadable_false_default(),
			is_versioned: is_versioned_true_default(),
			language_aware: false,
			input_format: None,
			jpeg_quality: None,
			jpeg_speed_over_compression: false,
			transformations: Default::default(),
			img_sizes: None,
			img_srcset: Default::default(),
		}
	}
}

impl Pipeline for RasterImagePipeline
{
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority
	{
		NoDependenciesEgImage
	}
	
	#[inline(always)]
	fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		self.input_format.resourceInputContentFileNamesWithExtension(resourceInputName)
	}
	
	#[inline(always)]
	fn is<'a>(&self) -> (bool, bool)
	{
		(self.is_versioned, self.language_aware)
	}
	
	#[inline(always)]
	fn execute(&self, inputContentFilePath: &Path, resourceRelativeUrl: &str, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, _siteMapWebPages: &mut Vec<SiteMapWebPage>, _rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		const CanNotBeCompressed: bool = false;
		
		let resourceRelativeUrlWithoutFileNameExtension = withoutFileNameExtension(resourceRelativeUrl);
		let imageInputFormat = self.input_format;
		let jpegQuality = self.jpeg_quality;
		let jpegSpeedOverCompression = self.jpeg_speed_over_compression;
		let transformations = &self.transformations;
		let imageSourceSetEntries = &self.img_srcset;
		
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
		
		// TODO: Need a way to pass this back
		let primaryImageDimensions = imageSourceSet.primaryImageDimensions();
		let processedImageSourceSet = imageSourceSet.processedImageSourceSet();
		
		let urls = imageSourceSet.urls(|url| generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanNotBeCompressed, self.max_age_in_seconds, self.is_downloadable, url))?;
		Ok(urls)
	}
}
