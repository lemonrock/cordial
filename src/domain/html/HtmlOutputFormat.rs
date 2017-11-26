// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum HtmlOutputFormat
{
	html_only
	{
		#[serde(default = "HtmlOutputFormat::html_template_default")] html_template: Rc<String>,
		#[serde(default = "HtmlOutputFormat::pjax_css_selector_default")] pjax_css_selector: Rc<String>,
	},
	html_and_amp
	{
		#[serde(default = "HtmlOutputFormat::html_template_default")] html_template: Rc<String>,
		#[serde(default = "HtmlOutputFormat::pjax_css_selector_default")] pjax_css_selector: Rc<String>,
		#[serde(default = "HtmlOutputFormat::amp_template_default")] amp_template: Rc<String>,
	},
	amp_only
	{
		#[serde(default = "HtmlOutputFormat::amp_template_default")] amp_template: Rc<String>,
	}
}
impl Default for HtmlOutputFormat
{
	#[inline(always)]
	fn default() -> Self
	{
		HtmlOutputFormat::amp_only
		{
			amp_template: Self::html_template_default(),
		}
	}
}

impl HtmlOutputFormat
{
	const IsNotAmp: bool = false;
	
	const IsAmp: bool = true;
	
	const IsNotPjax: bool = false;
	
	const CanBeCompressed: bool = true;
	
	const IsNotDownloadable: bool = true;
	
	#[inline(always)]
	pub(crate) fn renderHtmlDocumentsAndRedirects<'a>(&self, htmlDocumentData: &HtmlDocumentData, headerGenerator: &mut HeaderGenerator<'a>, maximumAge: u32, inputContentFilePath: &Path, handlebars: &HandlebarsWrapper) -> Result<Vec<PipelineResource>, CordialError>
	{
		let mut result = Vec::new();
		
		let htmlUrls = &htmlDocumentData.htmlUrls;
		
		let redirectToCanonicalUrl = htmlUrls.redirectUrl()?;
		let canonicalUrl = htmlUrls.linkHeaderCanonicalUrl()?;
		
		use self::HtmlOutputFormat::*;
		match *self
		{
			html_only { ref html_template, ref pjax_css_selector } =>
			{
				Self::redirectToDocument(redirectToCanonicalUrl, &canonicalUrl, &mut result)?;
				Self::regularHtmlDocument(canonicalUrl, pjax_css_selector,  html_template.as_str(), htmlDocumentData, headerGenerator, maximumAge, inputContentFilePath, handlebars, &mut result)?;
			}
			
			html_and_amp { ref html_template, ref pjax_css_selector, ref amp_template } =>
			{
				Self::redirectToDocument(redirectToCanonicalUrl, &canonicalUrl, &mut result)?;
				Self::regularHtmlDocument(canonicalUrl, pjax_css_selector, html_template.as_str(), htmlDocumentData, headerGenerator, maximumAge, inputContentFilePath, handlebars, &mut result)?;
				
				let redirectToAmpUrl = htmlUrls.ampRedirectUrl()?;
				let ampUrl = htmlUrls.ampUrl()?;
				Self::redirectToDocument(redirectToAmpUrl, &ampUrl, &mut result)?;
				Self::ampDocument(ampUrl, false, amp_template.as_str(), htmlDocumentData, headerGenerator, maximumAge, inputContentFilePath, handlebars, &mut result)?;
			}
			
			amp_only { ref amp_template } =>
			{
				Self::redirectToDocument(redirectToCanonicalUrl, &canonicalUrl, &mut result)?;
				Self::ampDocument(canonicalUrl, true, amp_template.as_str(), htmlDocumentData, headerGenerator, maximumAge, inputContentFilePath, handlebars, &mut result)?;
			}
		}
		
		Ok(result)
	}
	
	#[inline(always)]
	fn extractNodes(selector: &str, document: &RcDom, selectorName: &str) -> Result<Vec<u8>, CordialError>
	{
		const html_head_and_body_tags_are_optional: bool = true;
		const PreserveComments: bool = false;
		const PreserveProcessingInstructions: bool = false;
		
		let mut writer = Vec::with_capacity(8192);
		{
			let mut serializer = UltraMinifyingHtmlSerializer::new(html_head_and_body_tags_are_optional, PreserveComments, PreserveProcessingInstructions, &mut writer);
			
			match parse_css_selector(selector)
			{
				Err(_) => return Err(CordialError::Configuration(format!("CSS {} {} was invalid", selectorName, selector))),
				Ok(selector) => document.find_all_matching_child_nodes_depth_first_excluding_this_one(&selector, &mut |node|
				{
					const collapse_whitespace: bool = true;
					const flush_when_serialized: bool = false;
					if serializer.serialize_node(node, collapse_whitespace, flush_when_serialized).is_err()
					{
						//return Err(CordialError::Configuration("Could not serialize node - is this even possible?".to_owned()));
					}
					false
				}),
			};
		}
		writer.shrink_to_fit();
		
		Ok(writer)
	}
	
	#[inline(always)]
	fn redirectToDocument(redirectFromUrl: Option<Url>, canonicalUrl: &Url, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		// Redirect to Canonical HTML document
		if let Some(redirectUrl) = redirectFromUrl
		{
			let urlTags = hashmap! { redirect => Rc::new(UrlDataDetails::Empty) };
			
			let redirectToCanonicalUrlHeaders = Self::redirectHeaders(canonicalUrl);
			let redirectToCanonicalUrlBody = vec![];
			
			const RedirectsCanNotBeCompressed: bool = false;
			result.push((redirectUrl, urlTags, StatusCode::MovedPermanently, ContentType::plaintext(), redirectToCanonicalUrlHeaders, redirectToCanonicalUrlBody, None, RedirectsCanNotBeCompressed));
		}
		Ok(())
	}
	
	#[inline(always)]
	fn regularHtmlDocument(htmlUrl: Url, pjax_css_selector: &str, template: &str, htmlDocumentData: &HtmlDocumentData, headerGenerator: &mut HeaderGenerator, maximumAge: u32, inputContentFilePath: &Path, handlebars: &HandlebarsWrapper, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		let urlTags = hashmap! { default => Rc::new(UrlDataDetails::Empty) };
		
		// Canonical HTML document
		let htmlHeaders = headerGenerator.generateHeaders(Self::IsNotPjax, Self::CanBeCompressed, maximumAge, Self::IsNotDownloadable, &htmlUrl)?;
		let (htmlDocument, htmlBody) = htmlDocumentData.renderHtmlDocument(inputContentFilePath, Self::IsNotAmp, handlebars, template)?;
		
		// PJAX variant of HTML document
		const IsPjax: bool = true;
		let pjaxHeaders = headerGenerator.generateHeaders(IsPjax, Self::CanBeCompressed, maximumAge, Self::IsNotDownloadable, &htmlUrl)?;
		let pjaxBody = Self::extractNodes(pjax_css_selector, &htmlDocument, "pjax_css_selector")?;
		let pjax = Some((pjaxHeaders, pjaxBody));
		
		result.push((htmlUrl, urlTags, StatusCode::Ok, ContentType::html(), htmlHeaders, htmlBody, pjax, Self::CanBeCompressed));
		Ok(())
	}
	
	#[inline(always)]
	fn ampDocument(ampUrl: Url, isAlsoCanonical: bool, template: &str, htmlDocumentData: &HtmlDocumentData, headerGenerator: &mut HeaderGenerator, maximumAge: u32, inputContentFilePath: &Path, handlebars: &HandlebarsWrapper, result: &mut Vec<PipelineResource>) -> Result<(), CordialError>
	{
		let urlDataDetails = Rc::new(UrlDataDetails::Empty);
		let mut urlTags = hashmap!
		{
			amp => urlDataDetails.clone(),
		};
		if isAlsoCanonical
		{
			urlTags.insert(default, urlDataDetails);
		}
		
		// Canonical HTML document
		let htmlHeaders = headerGenerator.generateHeaders(Self::IsNotPjax, Self::CanBeCompressed, maximumAge, Self::IsNotDownloadable, &ampUrl)?;
		let (_htmlDocument, htmlBody) = htmlDocumentData.renderHtmlDocument(inputContentFilePath, Self::IsAmp, handlebars, template)?;
		
		// PJAX variant of HTML document
		let pjax = None;
		
		result.push((ampUrl, urlTags, StatusCode::Ok, ContentType::html(), htmlHeaders, htmlBody, pjax, Self::CanBeCompressed));
		Ok(())
	}
	
	#[inline(always)]
	fn redirectHeaders(redirectToUrl: &Url) -> Vec<(String, String)>
	{
		vec!
		[
			("Cache-Control".to_owned(), format!("{}", commonCacheControlHeader(31536000))),
			("Location".to_owned(), redirectToUrl.as_str().to_owned()),
		]
	}
	
	#[inline(always)]
	fn html_template_default() -> Rc<String>
	{
		Rc::new("article".to_owned())
	}
	
	#[inline(always)]
	fn pjax_css_selector_default() -> Rc<String>
	{
		Rc::new("main".to_owned())
	}
	
	#[inline(always)]
	fn amp_template_default() -> Rc<String>
	{
		Rc::new("amp-article".to_owned())
	}
}
