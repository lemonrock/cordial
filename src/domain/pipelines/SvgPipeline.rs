// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct SvgPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] language_aware: bool,
	
	#[serde(default)] do_not_optimize: bool, // Exists solely because of potential bugs in svg optimizer
	
	// TODO: Add option to add height, width if missing
	// TODO: Add alternative output formats, eg ICO and PNG, with multiple sizes
}

impl Default for SvgPipeline
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
			do_not_optimize: false,
		}
	}
}

impl Pipeline for SvgPipeline
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
	fn execute(&self, inputContentFilePath: &Path, resourceRelativeUrl: &str, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, _siteMapWebPages: &mut Vec<SiteMapWebPage>, _rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		const CanBeCompressed: bool = true;
		
		let url = languageData.url(&replaceFileNameExtension(resourceRelativeUrl, ".svg"))?;
		
		let headers = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanBeCompressed, self.max_age_in_seconds, self.is_downloadable, &url)?;
		let body = if self.do_not_optimize
		{
			inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?
		}
		else
		{
			inputContentFilePath.fileContentsAsACleanedSvgFrom()?
		};
		Ok(vec![(url, hashmap! { default => Rc::new(JsonValue::Null) }, StatusCode::Ok, ContentType(mimeType("image/svg+xml")), headers, body, None, CanBeCompressed)])
	}
}
