// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Serialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct LanguageData<'a>
{
	pub(crate) iso_639_1_alpha_2_language_code: &'a str,
	pub(crate) language: &'a Language,
}

impl<'a> LanguageData<'a>
{
	pub(crate) fn new(iso_639_1_alpha_2_language_code: &'a str, language: &'a Language) -> Self
	{
		Self
		{
			iso_639_1_alpha_2_language_code,
			language,
		}
	}
	
	#[inline(always)]
	pub(crate) fn baseUrl(&self, is_for_amp: bool) -> Result<Url, CordialError>
	{
		self.language.baseUrl(self.iso_639_1_alpha_2_language_code, is_for_amp)
	}
	
	#[inline(always)]
	pub(crate) fn url(&self, resourceRelativeUrl: &str) -> Result<Url, CordialError>
	{
		self._url(resourceRelativeUrl, false)
	}
	
	#[inline(always)]
	pub(crate) fn leaf_url(&self, resourceRelativeUrl: &str) -> Result<Url, CordialError>
	{
		self._leaf_url(resourceRelativeUrl, false)
	}
	
	#[inline(always)]
	pub(crate) fn amp_url(&self, resourceRelativeUrl: &str) -> Result<Url, CordialError>
	{
		self._url(resourceRelativeUrl, true)
	}
	
	#[inline(always)]
	pub(crate) fn amp_leaf_url(&self, resourceRelativeUrl: &str) -> Result<Url, CordialError>
	{
		self._leaf_url(resourceRelativeUrl, true)
	}
	
	#[inline(always)]
	fn _url(&self, resourceRelativeUrl: &str, is_for_amp: bool) -> Result<Url, CordialError>
	{
		let baseUrl = self.baseUrl(is_for_amp)?;
		let url = baseUrl.join(resourceRelativeUrl).context(format!("Invalid resourceRelativeUrl '{}'", resourceRelativeUrl))?;
		Ok(url)
	}
	
	#[inline(always)]
	fn _leaf_url(&self, resourceRelativeUrl: &str, is_for_amp: bool) -> Result<Url, CordialError>
	{
		let mut leafPath = String::with_capacity(resourceRelativeUrl.len() + 1);
		leafPath.push_str(resourceRelativeUrl);
		leafPath.push('/');
		self._url(&leafPath, is_for_amp)
	}
}
