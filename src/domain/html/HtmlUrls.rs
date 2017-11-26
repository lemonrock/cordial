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
	
	pub(crate) rssChannelNames: OrderMap<Rc<RssChannelName>, ()>,
	pub(crate) canonicalShortlink: Option<Rc<UrlSerde>>,
	pub(crate) pingback: Option<Rc<UrlSerde>>,
	pub(crate) previous: Option<ResourceUrl>,
	pub(crate) next: Option<ResourceUrl>,
	pub(crate) author: Option<ResourceUrl>,
	pub(crate) help: Option<ResourceUrl>,
	pub(crate) license: Option<ResourceUrl>,
}

impl<'a> HtmlUrls<'a>
{
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
	pub(crate) fn linkHeaderAlternativeLanguageUrlsIncludingSelf(&self) -> Result<BTreeMap<Iso639Dash1Alpha2Language, Url>, CordialError>
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
	pub(crate) fn linkHeaderRssUrls(&self) -> Result<Vec<Url>, CordialError>
	{
		let mut urls = Vec::with_capacity(self.rssChannelNames.len());
		for rssChannelName in self.rssChannelNames.keys()
		{
			let rssResourceUrl = ResourceUrl::rssUrl(rssChannelName, self.languageData.iso639Dash1Alpha2Language);
			urls.push(rssResourceUrl.url(self.languageData)?);
		}
		Ok(urls)
	}
	
	// link rel="shortlink"
	// The shortlink will always be on an external service. It is canonical for the particular language of this HTML document.
	#[inline(always)]
	pub(crate) fn linkHeaderCanonicalShortLinkUrl(&self) -> Option<Rc<UrlSerde>>
	{
		self.canonicalShortlink.clone()
	}
	
	// link rel="pingback"
	// Can also be used in X-Pingback: Header, which takes precedence.
	// Since we are a static system, the pingback URL may not be under our control. It is canonical for the particular language of this HTML document.
	#[inline(always)]
	pub(crate) fn linkHeaderPingbackUrl(&self) -> Option<Rc<UrlSerde>>
	{
		self.pingback.clone()
	}
	
	// link rel="prev"
	#[inline(always)]
	pub(crate) fn linkHeaderPreviousUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.previous)
	}
	
	// link rel="next"
	#[inline(always)]
	pub(crate) fn linkHeaderNextUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.next)
	}
	
	// link rel="help"
	#[inline(always)]
	pub(crate) fn linkHeaderHelpUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.help)
	}
	
	// link rel="author"
	#[inline(always)]
	pub(crate) fn linkHeaderAuthorUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.author)
	}
	
	// link rel="license"
	#[inline(always)]
	pub(crate) fn linkHeaderLicenseUrl(&self) -> Result<Option<Url>, CordialError>
	{
		self.optionalUrl(&self.license)
	}
	
	#[inline(always)]
	fn optionalUrl(&self, optionalUrl: &Option<ResourceUrl>) -> Result<Option<Url>, CordialError>
	{
		match optionalUrl
		{
			&None => Ok(None),
			&Some(ref resourceUrl) =>
			{
				resourceUrl.validateResourceExists(self.resources)?;
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
