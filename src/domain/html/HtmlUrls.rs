// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct HtmlUrls<'a>
{
	pub(crate) is_leaf: bool,
	pub(crate) redirect_nearly_identical_url: bool,
	pub(crate) resourceUrl: ResourceUrl,
	pub(crate) languageData: &'a LanguageData<'a>,
	pub(crate) localization: &'a Localization,
	pub(crate) resources: &'a Resources,
	
	pub(crate) rssChannelNames: &'a OrderMap<Rc<RssChannelName>, ()>,
	pub(crate) canonicalShortlink: Option<Rc<UrlSerde>>,
	pub(crate) pingback: Option<Rc<UrlSerde>>,
	pub(crate) previous: Option<ResourceUrl>,
	pub(crate) next: Option<ResourceUrl>,
	pub(crate) author: Option<ResourceUrl>,
	pub(crate) help: Option<ResourceUrl>,
	pub(crate) license: Option<ResourceUrl>,
	pub(crate) manifest: Option<ResourceUrl>,
}

impl<'a> HtmlUrls<'a>
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn addLinkNodes(&self, endHeadNodes: &mut Vec<UnattachedNode>, addAmpLink: bool, ampLinkIsCanonical: bool) -> Result<(), CordialError>
	{
		endHeadNodes.push("link".with_rel_attribute("canonical").with_href_attribute(self.linkHeaderCanonicalUrl()?.as_ref()));
		
		if addAmpLink
		{
			let url = if ampLinkIsCanonical
			{
				self.htmlUrl()?
			}
			else
			{
				self.ampUrl()?
			};
			endHeadNodes.push("link".with_rel_attribute("amphtml").with_href_attribute(url.as_ref()));
		}
		
		for (iso639Dash1Alpha2Language, url) in self.linkHeaderAlternativeLanguageUrlsIncludingSelf()?.iter()
		{
			endHeadNodes.push("link".with_rel_attribute("alternate").with_attribute("hreflang".str_attribute(iso639Dash1Alpha2Language.to_iso_639_1_alpha_2_language_code())).with_href_attribute(url.as_ref()));
		}
		
		for rssUrl in self.linkHeaderRssUrls()?.iter()
		{
			endHeadNodes.push("link".with_rel_attribute("alternate").with_attribute("type".str_attribute("application/rss+xml")).with_href_attribute(rssUrl.as_ref()));
		}
		
		if let Some(ref canonicalShortLink) = self.canonicalShortlink
		{
			endHeadNodes.push("link".with_rel_attribute("shortlink").with_href_attribute(canonicalShortLink.0.as_ref()));
		}
		
		if let Some(ref pingback) = self.pingback
		{
			endHeadNodes.push("link".with_rel_attribute("pingback").with_href_attribute(pingback.0.as_ref()));
		}
		
		if let Some(ref previous) = self.linkHeaderPreviousUrl()?
		{
			endHeadNodes.push("link".with_rel_attribute("prev").with_href_attribute(previous.as_ref()));
		}
		
		if let Some(ref next) = self.linkHeaderNextUrl()?
		{
			endHeadNodes.push("link".with_rel_attribute("next").with_href_attribute(next.as_ref()));
		}
		
		if let Some(ref help) = self.linkHeaderHelpUrl()?
		{
			endHeadNodes.push("link".with_rel_attribute("help").with_href_attribute(help.as_ref()));
		}
		
		if let Some(ref author) = self.linkHeaderAuthorUrl()?
		{
			endHeadNodes.push("link".with_rel_attribute("author").with_href_attribute(author.as_ref()));
		}
		
		if let Some(ref license) = self.linkHeaderLicenseUrl()?
		{
			endHeadNodes.push("link".with_rel_attribute("license").with_href_attribute(license.as_ref()));
		}
		
		if let Some(ref manifest) = self.linkHeaderManifestUrl()?
		{
			endHeadNodes.push("link".with_rel_attribute("manifest").with_href_attribute(manifest.as_ref()));
		}
		
		Ok(())
	}
	
	// rel="canonical"
	#[inline(always)]
	pub(crate) fn linkHeaderCanonicalUrl(&self) -> Result<Url, CordialError>
	{
		self.canonicalUrl(self.languageData)
	}
	
	// Other languages?
	// link rel="alternate" hreflang="ISO 631 CODE"
	// Also, if there is a home-page that redirects to a particular language (eg based on browser) or there is a language selector page, then it should have hreflang="x-default"
	#[inline(always)]
	fn linkHeaderAlternativeLanguageUrlsIncludingSelf(&self) -> Result<BTreeMap<Iso639Dash1Alpha2Language, Url>, CordialError>
	{
		let mut urlsByLanguage = BTreeMap::new();
		self.localization.visitLanguagesWithPrimaryFirst(|languageData, _isPrimaryLanguage|
		{
			let url = self.canonicalUrl(languageData)?;
			urlsByLanguage.insert(languageData.iso639Dash1Alpha2Language, url);
			Ok(())
		})?;
		Ok(urlsByLanguage)
	}
	
	// link rel="alternate" type="application/rss+xml"
	// First feed in list is considered the default.
	#[inline(always)]
	fn linkHeaderRssUrls(&self) -> Result<Vec<Url>, CordialError>
	{
		let mut urls = Vec::with_capacity(self.rssChannelNames.len());
		for rssChannelName in self.rssChannelNames.keys()
		{
			let rssResourceUrl = ResourceUrl::rssUrl(rssChannelName, self.languageData.iso639Dash1Alpha2Language);
			urls.push(rssResourceUrl.url(self.languageData)?);
		}
		Ok(urls)
	}
	
	// link rel="prev"
	#[inline(always)]
	fn linkHeaderPreviousUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.previous, &content_type_text_html_utf8().0)
	}
	
	// link rel="next"
	#[inline(always)]
	fn linkHeaderNextUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.next, &content_type_text_html_utf8().0)
	}
	
	// link rel="help"
	#[inline(always)]
	fn linkHeaderHelpUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.help, &content_type_text_html_utf8().0)
	}
	
	// link rel="author"
	#[inline(always)]
	fn linkHeaderAuthorUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.author, &content_type_text_html_utf8().0)
	}
	
	// link rel="license"
	#[inline(always)]
	fn linkHeaderLicenseUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.license, &content_type_text_html_utf8().0)
	}
	
	// link rel="manifest"
	#[inline(always)]
	fn linkHeaderManifestUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.manifest, &content_type_application_manifest_json_utf8().0)
	}
	
	#[inline(always)]
	fn optionalUrl(&self, optionalUrl: &Option<ResourceUrl>, hasMimeType: &Mime) -> Result<Option<Url>, CordialError>
	{
		match optionalUrl
		{
			&None => Ok(None),
			&Some(ref resourceUrl) =>
			{
				resourceUrl.validateResourceExistsWithMimeTypeExcludingParameters(self.resources, hasMimeType, self.localization.fallbackIso639Dash1Alpha2Language(), Some(self.languageData.iso639Dash1Alpha2Language))?;
				Ok(Some(resourceUrl.url(self.languageData)?))
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn htmlUrl(&self) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			self.resourceUrl.leaf_url(self.languageData)
		}
		else
		{
			self.resourceUrl.url(self.languageData)
		}
	}
	
	#[inline(always)]
	pub(crate) fn redirectUrl(&self) -> Result<Option<Url>, CordialError>
	{
		if self.redirect_nearly_identical_url
		{
			let url = if self.is_leaf
			{
				self.resourceUrl.url(self.languageData)?
			}
			else
			{
				self.resourceUrl.leaf_url(self.languageData)?
			};
			Ok(Some(url))
		}
		else
		{
			Ok(None)
		}
	}
	
	#[inline(always)]
	pub(crate) fn ampUrl(&self) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			self.resourceUrl.amp_leaf_url(self.languageData)
		}
		else
		{
			self.resourceUrl.amp_url(self.languageData)
		}
	}
	
	#[inline(always)]
	pub(crate) fn ampRedirectUrl(&self) -> Result<Option<Url>, CordialError>
	{
		if self.redirect_nearly_identical_url
		{
			let url = if self.is_leaf
			{
				self.resourceUrl.amp_url(self.languageData)?
			}
			else
			{
				self.resourceUrl.amp_leaf_url(self.languageData)?
			};
			Ok(Some(url))
		}
		else
		{
			Ok(None)
		}
	}
	
	#[inline(always)]
	fn canonicalUrl(&self, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		if self.is_leaf
		{
			self.resourceUrl.leaf_url(languageData)
		}
		else
		{
			self.resourceUrl.url(languageData)
		}
	}
}
