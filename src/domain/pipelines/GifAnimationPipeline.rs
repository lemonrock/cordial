// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct GifAnimationPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] language_aware: bool,
	#[serde(default)] input_format: Option<ImageInputFormat>,
	
	#[serde(default)] metadata: Rc<ImageMetaData>,
	#[serde(default)] source_set: Vec<EngiffenSource>,
	
	#[serde(default)] quantizer: EngiffenQuantizer,
	#[serde(default)] loops: EngiffenLoops,
	
	#[serde(default = "ProcessedImageSourceSet::processedImageSourceSet_default", skip_deserializing)] processedImageSourceSet: RefCell<ProcessedImageSourceSet>,
}

impl Default for GifAnimationPipeline
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
			source_set: Default::default(),
			quantizer: Default::default(),
			loops: Default::default(),
			processedImageSourceSet: ProcessedImageSourceSet::processedImageSourceSet_default(),
		}
	}
}

impl Pipeline for GifAnimationPipeline
{
	#[inline(always)]
	fn imageMetaData(&self) -> Result<&Rc<ImageMetaData>, CordialError>
	{
		Ok(&self.metadata)
	}
	
	#[inline(always)]
	fn addToImgAttributes(&self, attributes: &mut Vec<Attribute>) -> Result<(), CordialError>
	{
		let first = self.source_set.get(0).expect("Already validated in execute");
		attributes.push("width".u16_attribute(first.width));
		attributes.push("height".u16_attribute(first.height));
		
		if self.source_set.len() > 1
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
	fn is<'a>(&self) -> (bool, bool)
	{
		(self.is_versioned, self.language_aware)
	}
	
	#[inline(always)]
	fn execute(&self, _resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, _siteMapWebPages: &mut Vec<SiteMapWebPage>, _rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<ResourceTag, Rc<UrlDataDetails>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		let engiffen = Engiffen::new(inputContentFilePath, &self.source_set, &self.quantizer, self.loops, self.input_format, resourceUrl, languageData)?;
		
		engiffen.processedImageSourceSet(self.processedImageSourceSet.try_borrow_mut()?.deref_mut())?;
		
		const CanNotBeCompressed: bool = false;
		engiffen.process(|url| generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanNotBeCompressed, self.max_age_in_seconds, self.is_downloadable, url))
	}
}
