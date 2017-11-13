// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct HtmlPipeline
{
	#[serde(default = "HtmlPipeline::max_age_in_seconds_none_default")] max_age_in_seconds: u32,
	#[serde(default)] input_format: HtmlInputFormat,
	#[serde(default)] is_leaf: bool,
	#[serde(default = "HtmlPipeline::amp_relative_root_url_default")] amp_relative_root_url: Option<String>,
	
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
}

impl Pipeline for HtmlPipeline
{
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority
	{
		LinksToSubResourcesEgHtmlPage
	}
	
	#[inline(always)]
	fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		self.input_format.resourceInputContentFileNamesWithExtension(resourceInputName)
	}
	
	#[inline(always)]
	fn is<'a>(&self) -> (bool, bool)
	{
		(Self::is_versioned, Self::language_aware)
	}
	
	#[inline(always)]
	fn execute(&mut self, inputContentFilePath: &Path, resourceRelativeUrl: &str, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, siteMapWebPages: &mut Vec<SiteMapWebPage>, rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		const CanBeCompressed: bool = true;
		
		let inputCanonicalUrl = if self.is_leaf
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
		
		let mut result = Vec::with_capacity(2);
		
		let regularHeaders = generateHeaders(handlebars, headerTemplates, Some(languageData), HtmlVariant::Canonical, configuration, CanBeCompressed, self.max_age_in_seconds, Self::is_downloadable, &inputCanonicalUrl)?;
		let pjaxHeaders = generateHeaders(handlebars, headerTemplates, Some(languageData), HtmlVariant::PJAX, configuration, CanBeCompressed, self.max_age_in_seconds, Self::is_downloadable, &inputCanonicalUrl)?;
		
		
		
		
		let regularBody = Vec::new();
		let pjaxBody = Vec::new();
		
		
		
		
		
		if let Some(ref amp_relative_root_url) = self.amp_relative_root_url
		{
			let baseAmpUrl = Url::parse(amp_relative_root_url).context("invalid AMP relative root URL".to_owned())?;
			let ampUrl = baseAmpUrl.join(inputCanonicalUrl.as_str()).context("invalid combination of AMP relative root URL joined with input canonical URL".to_owned())?;
			let ampHeaders = generateHeaders(handlebars, headerTemplates, Some(languageData), HtmlVariant::AMP, configuration, CanBeCompressed, self.max_age_in_seconds, Self::is_downloadable, &ampUrl)?;
			
			
			
			let ampBody = Vec::new();
			
			
			
			result.push((ampUrl, hashmap! { amp => Rc::new(JsonValue::Null) }, ContentType::html(), ampHeaders, ampBody, None, CanBeCompressed));
		}
		
		result.push((inputCanonicalUrl, hashmap! { default => Rc::new(JsonValue::Null) }, ContentType::html(), regularHeaders, regularBody, Some((pjaxHeaders, pjaxBody)), CanBeCompressed));
		
		
		panic!("Implement regularBody, pjaxBody and ampBody");
		
		Ok(result)
		
		//let synopsisHtml = markdown_to_html(&rssItemLanguageVariant.webPageSynopsisMarkdown, markdownOptions);
		
		/*
		
			There is a markdown blob that will go into a <main></main>
				- Needs extension functions for
					- svgbob
					- internal URLs (documents)
					- external URLs
			
			So we need a set of templates
			
			Take HTML and run it through languagetool
			
			Take HTML and minify it
			
			Take HTML and extract PJAX page for it (use css selector for 'main' unless another selector is given)
			
			Take HTML and generate AMP page
			
			Open problem: How do we minify CSS?
				
				- minifying CSS common to several pages is quite challenging
				
				- if we embed CSS into our document, AMP-style, then we need to be careful also embedding repeated images into the CSS
			
			Discover images and videos to add to site maps
			
			Add to RSS feed
			
			JSON to pass to handlebars
				- all the stuff we do for CssInputFormat
				- document
					title
		Add to WebPageSiteMaps; detect videos and images  (see https://developers.google.com/webmasters/videosearch/sitemaps)
		
		Supporting video: https://www.html5rocks.com/en/tutorials/video/basics/
		
		// RSS: Best practice is to embed a full RSS article, not a summary.
		// RSS: Not all items should be published.
		
		*/
	}
}

impl HtmlPipeline
{
	const is_versioned: bool = false;
	
	const language_aware: bool = true;
	
	const is_downloadable: bool = false;
	
	#[inline(always)]
	fn max_age_in_seconds_none_default() -> u32
	{
		0
	}
	
	#[inline(always)]
	fn amp_relative_root_url_default() -> Option<String>
	{
		Some("/amp".to_owned())
	}
}
