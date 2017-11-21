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
	
	#[serde(default)] metadata: Rc<ImageMetaData>,
	#[serde(default)] source_set_excluding_original: Vec<ImageSourceSetEntry>,
	
	#[serde(default)] jpeg_quality: Option<u8>,
	#[serde(default)] jpeg_speed_over_compression: bool,
	#[serde(default)] transformations: Vec<ImageTransformation>,

	#[serde(default, skip_deserializing)] primaryImageDimensions: Cell<(u32, u32)>,
	#[serde(default = "ProcessedImageSourceSet::processedImageSourceSet_default", skip_deserializing)] processedImageSourceSet: RefCell<ProcessedImageSourceSet>,
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
			metadata: Default::default(),
			source_set_excluding_original: Default::default(),
			jpeg_quality: None,
			jpeg_speed_over_compression: false,
			transformations: Default::default(),
			primaryImageDimensions: Default::default(),
			processedImageSourceSet: ProcessedImageSourceSet::processedImageSourceSet_default(),
		}
	}
}

impl Pipeline for RasterImagePipeline
{
	#[inline(always)]
	fn imageMetaData(&self) -> Result<&Rc<ImageMetaData>, CordialError>
	{
		Ok(&self.metadata)
	}
	
	#[inline(always)]
	fn addToImgAttributes(&self, attributes: &mut Vec<Attribute>) -> Result<(), CordialError>
	{
		let dimensions = self.primaryImageDimensions.get();
		attributes.push("width".u32_attribute(dimensions.0));
		attributes.push("height".u32_attribute(dimensions.1));
		
		if self.source_set_excluding_original.len() > 0
		{
			ProcessedImageSourceSet::addToImgAttributes(&self.processedImageSourceSet, attributes)
		}
		else
		{
			Ok(())
		}
	}
	
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
	fn execute(&self, _resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, _siteMapWebPages: &mut Vec<SiteMapWebPage>, _rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<ResourceTag, Rc<UrlDataDetails>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		// load original
		let mut imageBeforeTransformation = match ImageInputFormat::load(self.input_format, inputContentFilePath)
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
		let imageAfterTransformation = if let Some(transformedImage) = ImageTransformation::applyTransformations(&mut imageBeforeTransformation, &self.transformations)?
		{
			transformedImage
		}
		else
		{
			imageBeforeTransformation
		};
		
		// generate image src set
		let mut imageSourceSet = ImageSourceSet::new(inputContentFilePath, &resourceUrl, self.jpeg_quality, self.jpeg_speed_over_compression, imageAfterTransformation, languageData);
		imageSourceSet.add(&self.source_set_excluding_original)?;
		
		self.primaryImageDimensions.set(imageSourceSet.primaryImageDimensions());
		{
			let mut borrowed = self.processedImageSourceSet.try_borrow_mut()?;
			imageSourceSet.processedImageSourceSet(&mut borrowed.2)?;
		}
		
		const CanNotBeCompressed: bool = false;
		let urls = imageSourceSet.urls(|url| generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanNotBeCompressed, self.max_age_in_seconds, self.is_downloadable, url))?;
		Ok(urls)
	}
}
