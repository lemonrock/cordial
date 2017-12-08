// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// See https://www.w3.org/TR/appmanifest/#webappmanifest-dictionary
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct WebAppManifestPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default = "WebAppManifestPipeline::language_aware_default")] language_aware: bool,
	#[serde(default)] input_format: Option<WebAppManifestInputFormat>,
	
	#[serde(default)] pub(crate) abstracts: HashMap<Iso639Dash1Alpha2Language, WebAppManifestAbstract>,
	#[serde(default)] pub(crate) icons: BTreeSet<WebAppManifestIcon>,
	#[serde(default)] pub(crate) screenshots: BTreeSet<WebAppManifestScreenshot>,
	#[serde(default)] pub(crate) categories: BTreeSet<WebAppManifestCategory>,
	#[serde(default)] pub(crate) iarc_rating_id: Option<String>,
	#[serde(default)] pub(crate) start_url: ResourceUrl,
	#[serde(default)] pub(crate) display: WebAppManifestDisplay,
	#[serde(default)] pub(crate) orientation: Option<WebAppManifestOrientation>,
	#[serde(default)] pub(crate) theme_css_color: Option<Rc<String>>,
	#[serde(default)] pub(crate) background_css_color: Option<Rc<String>>,
	#[serde(default)] pub(crate) scope: ResourceUrl,
	#[serde(default)] pub(crate) service_worker: Option<WebAppManifestServiceWorker>,
	#[serde(default)] pub(crate) related_applications: HashSet<WebAppManifestRelatedApplication>,
	#[serde(default)] pub(crate) prefer_related_applications: bool,
}

impl Default for WebAppManifestPipeline
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			max_age_in_seconds: max_age_in_seconds_long_default(),
			is_downloadable: is_downloadable_false_default(),
			is_versioned: Self::language_aware_default(),
			language_aware: false,
			input_format: None,
		
			abstracts: Default::default(),
			icons: Default::default(),
			screenshots: Default::default(),
			categories: Default::default(),
			iarc_rating_id: None,
			start_url: Default::default(),
			display: Default::default(),
			orientation: Default::default(),
			theme_css_color: None,
			background_css_color: None,
			scope: Default::default(),
			service_worker: None,
			related_applications: Default::default(),
			prefer_related_applications: false,
		}
	}
}

impl Pipeline for WebAppManifestPipeline
{
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority
	{
		DependsOnOthersEgStylesheetOrVideo
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
	fn execute(&self, resources: &Resources, _inputContentFilePath: &Path, resourceUrl: &ResourceUrl, _handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, _rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, _siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResponse>, CordialError>
	{
		let url = resourceUrl.replaceFileNameExtension(".json").url(languageData)?;
		
		const CanBeCompressed: bool = true;
		let headers = headerGenerator.generateHeadersForAsset(CanBeCompressed, self.max_age_in_seconds, self.is_downloadable, &url)?;
		
		let body = WebAppManifestJsonRoot
		{
			ourUrlToMinifyAgainst: &url,
			languageData,
			webAppManifestPipeline: self,
		}.to_json_bytes(resources, configuration.fallbackIso639Dash1Alpha2Language())?;
		
		Ok(vec![(url, hashmap! { default => Rc::new(UrlDataDetails::generic(&body)) }, StatusCode::Ok, content_type_application_manifest_json_utf8(), headers, ResponseBody::utf8(body), None, CanBeCompressed)])
	}
}

impl WebAppManifestPipeline
{
	#[inline(always)]
	fn language_aware_default() -> bool
	{
		true
	}
}
