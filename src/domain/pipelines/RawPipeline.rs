// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct RawPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] language_aware: bool,
	
	#[serde(default)] can_be_compressed: Option<bool>, // default is to use filename
	#[serde(default)] mime_type: Option<String>, // default is to use filename, and sniff text formats, with US-ASCII interpreted as UTF-8
}

impl Default for RawPipeline
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
			can_be_compressed: None,
			mime_type: None,
		}
	}
}

impl Pipeline for RawPipeline
{
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
	fn execute(&self, inputContentFilePath: &Path, resourceRelativeUrl: &str, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, _siteMapWebPages: &mut Vec<SiteMapWebPage>, _rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		let inputCanonicalUrl = languageData.url(resourceRelativeUrl)?;
		
		let canBeCompressed = if self.can_be_compressed.is_none()
		{
			!inputContentFilePath.hasCompressedFileExtension()?
		}
		else
		{
			self.can_be_compressed.unwrap()
		};
		
		let mimeType = if self.mime_type.is_none()
		{
			inputContentFilePath.guessMimeTypeWithCharacterSet()?
		}
		else
		{
			match self.mime_type.as_ref().unwrap().parse()
			{
				Err(error) => return Err(CordialError::Configuration(format!("Could not parse mime type '{:?}' because {:?} for {:?}", self.mime_type, error, inputContentFilePath))),
				Ok(mime) => mime,
			}
		};
		
		let headers = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, canBeCompressed, self.max_age_in_seconds, self.is_downloadable, &inputCanonicalUrl)?;
		let body = inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?;
		Ok(vec![(inputCanonicalUrl, hashmap! { default => Rc::new(JsonValue::Null) }, ContentType(mimeType), headers, body, None, canBeCompressed)])
	}
}
