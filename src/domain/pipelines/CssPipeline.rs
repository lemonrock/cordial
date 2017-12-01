// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct CssPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] language_aware: bool,
	#[serde(default)] input_format: Option<CssInputFormat>,

	#[serde(default = "CssPipeline::precision_default")] precision: u8,
	#[serde(default)] template_parameters: Option<JsonMap<String, JsonValue>>,
	#[serde(default = "CssPipeline::maximum_release_age_from_can_i_use_database_last_updated_in_weeks_default")] maximum_release_age_from_can_i_use_database_last_updated_in_weeks: u16,
	#[serde(default = "CssPipeline::minimum_usage_threshold_default")] minimum_usage_threshold: UsagePercentage,
	#[serde(default = "CssPipeline::regional_usages_default")] regional_usages: Vec<RegionalUsages>,
}

impl Default for CssPipeline
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
			precision: Self::precision_default(),
			template_parameters: None,
			maximum_release_age_from_can_i_use_database_last_updated_in_weeks: Self::maximum_release_age_from_can_i_use_database_last_updated_in_weeks_default(),
			minimum_usage_threshold: Self::minimum_usage_threshold_default(),
			regional_usages: Self::regional_usages_default(),
		}
	}
}

impl Pipeline for CssPipeline
{
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority
	{
		DependsOnOthersEgStylesheet
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
	fn execute(&self, _resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, _rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, _siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResource>, CordialError>
	{
		let url = resourceUrl.replaceFileNameExtension(".css").url(languageData)?;
		
		const CanBeCompressed: bool = true;
		let headers = headerGenerator.generateHeadersForAsset(CanBeCompressed, self.max_age_in_seconds, self.is_downloadable, &url)?;
		
		let templateData = if let Some(ref templateParameters) = self.template_parameters
		{
			Some((handlebars, templateParameters, languageData))
		}
		else
		{
			None
		};
		let body = CssInputFormat::toCss(self.input_format, inputContentFilePath, self.precision, configuration, templateData, self.maximum_release_age_from_can_i_use_database_last_updated_in_weeks, self.minimum_usage_threshold, &self.regional_usages[..])?;

		Ok(vec![(url, hashmap! { default => Rc::new(UrlDataDetails::generic(&body)) }, StatusCode::Ok, ContentType(TEXT_CSS), headers, body, None, CanBeCompressed)])
	}
}

impl CssPipeline
{
	#[inline(always)]
	fn precision_default() -> u8
	{
		1
	}

	#[inline(always)]
	fn maximum_release_age_from_can_i_use_database_last_updated_in_weeks_default() -> u16
	{
		const FirefoxCycleLengthInWeeks: u16 = 6;
		const FirefoxCyclesPerYear: u16 = (52 / FirefoxCycleLengthInWeeks + 1);

		(FirefoxCyclesPerYear + 2) * FirefoxCycleLengthInWeeks
	}

	#[inline(always)]
	fn minimum_usage_threshold_default() -> UsagePercentage
	{
		UsagePercentage::OnePerMille
	}

	#[inline(always)]
	fn regional_usages_default() -> Vec<RegionalUsages>
	{
		vec!
		[
			RegionalUsages::WorldWide,
		]
	}
}
