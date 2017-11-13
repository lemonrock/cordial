// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.

#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FontPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] language_aware: bool,
	#[serde(default)] input_format: Option<FontInputFormat>,
	
	#[serde(default)] utf8_xml_metadata: Vec<u8>,
	#[serde(default)] woff1_private_data: Vec<u8>,
	#[serde(default = "FontPipeline::woff1_iterations_default")] woff1_iterations: u16,
	#[serde(default = "FontPipeline::woff2_brotli_quality_default")] woff2_brotli_quality: u8,
	#[serde(default)] woff2_disallow_transforms: bool,
	#[serde(default)] include_ttf: bool,
}

impl Default for FontPipeline
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
			
			utf8_xml_metadata: Default::default(),
			woff1_private_data: Default::default(),
			woff1_iterations: Self::woff1_iterations_default(),
			woff2_brotli_quality: Self::woff2_brotli_quality_default(),
			woff2_disallow_transforms: false,
			include_ttf: false,
		}
	}
}

impl Pipeline for FontPipeline
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
		FontInputFormat::toWebFonts(self.input_format, resourceRelativeUrl, configuration, inputContentFilePath, handlebars, headerTemplates, ifLanguageAwareLanguageData, languageData, self.max_age_in_seconds,self. is_downloadable, &self.utf8_xml_metadata[..], &self.woff1_private_data[..], self.woff1_iterations, self.woff2_brotli_quality, self.woff2_disallow_transforms, self.include_ttf)
	}
}

impl FontPipeline
{
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
