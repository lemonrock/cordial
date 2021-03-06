// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Localization
{
	#[serde(with = "url_serde", default = "Localization::language_tool_base_url_default")] language_tool_base_url: Url,
	#[serde(default)] pub(crate) primary_iso_639_1_alpha_2_language: Iso639Dash1Alpha2Language,
	#[serde(default = "Localization::languages_default")] languages: HashMap<Iso639Dash1Alpha2Language, Language>,
}

impl Default for Localization
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			language_tool_base_url: Self::language_tool_base_url_default(),
			primary_iso_639_1_alpha_2_language: Default::default(),
			languages: Self::languages_default(),
		}
	}
}

impl Localization
{
	#[inline(always)]
	pub(crate) fn fallbackIso639Dash1Alpha2Language(&self) -> Iso639Dash1Alpha2Language
	{
		self.primary_iso_639_1_alpha_2_language
	}
	
	#[inline(always)]
	pub(crate) fn primaryLanguage(&self) -> Result<&Language, CordialError>
	{
		self.language(self.primary_iso_639_1_alpha_2_language)
	}
	
	#[inline(always)]
	pub(crate) fn languageData<'a>(&'a self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<LanguageData<'a>, CordialError>
	{
		match self.languages.get(&iso639Dash1Alpha2Language)
		{
			None => Err(CordialError::Configuration(format!("iso639Dash1Alpha2Language '{}' does not have a defined language", iso639Dash1Alpha2Language))),
			Some(language) => Ok(LanguageData::new(iso639Dash1Alpha2Language, language)),
		}
	}
	
	#[inline(always)]
	pub(crate) fn language(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&Language, CordialError>
	{
		match self.languages.get(&iso639Dash1Alpha2Language)
		{
			None => Err(CordialError::Configuration(format!("iso639Dash1Alpha2Language '{}' does not have a defined language", iso639Dash1Alpha2Language))),
			Some(language) => Ok(language),
		}
	}
	
	#[inline(always)]
	pub(crate) fn facebookOpenGraphLocaleStr(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&str, CordialError>
	{
		Ok(self.language(iso639Dash1Alpha2Language)?.facebookOpenGraphLocaleStr())
	}
	
	#[inline(always)]
	pub(crate) fn otherLanguages(&self, excludeIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> HashMap<Iso639Dash1Alpha2Language, Language>
	{
		self.languages.iter().filter(|&(code, _)| code != &excludeIso639Dash1Alpha2Language).map(|(code, language)| (code.to_owned(), language.clone())).collect()
	}
	
	#[inline(always)]
	pub(crate) fn visitLanguagesWithPrimaryFirst<F: FnMut(&LanguageData, bool) -> Result<(), CordialError>>(&self, mut visitor: F) -> Result<(), CordialError>
	{
		visitor(&LanguageData::new(self.primary_iso_639_1_alpha_2_language, self.primaryLanguage()?), true)?;
		for (iso639Dash1Alpha2Language, language) in self.languages.iter()
		{
			if iso639Dash1Alpha2Language != &self.primary_iso_639_1_alpha_2_language
			{
				visitor(&LanguageData::new(*iso639Dash1Alpha2Language, language), false)?;
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn numberOfLanguages(&self) -> usize
	{
		self.languages.len()
	}
	
	pub(crate) fn serverHostNames(&self) -> Result<HashSet<String>, CordialError>
	{
		let mut serverHostNames = HashSet::with_capacity(self.languages.len());
		
		for language in self.languages.values()
		{
			serverHostNames.insert(language.host().to_owned());
		}
		
		Ok(serverHostNames)
	}
	
	pub(crate) fn serverHostNamesWithPrimaryFirst(&self) -> Result<OrderMap<String, ()>, CordialError>
	{
		let mut serverHostNames = OrderMap::with_capacity(self.languages.len());
		
		let primaryLanguage = self.primaryLanguage()?;
		serverHostNames.insert(primaryLanguage.host().to_owned(), ());
		
		for language in self.languages.values()
		{
			let host = language.host();
			if serverHostNames.get(host).is_none()
			{
				serverHostNames.insert(host.to_owned(), ());
			}
		}
		
		Ok(serverHostNames)
	}
	
	#[inline(always)]
	fn language_tool_base_url_default() -> Url
	{
		Url::parse("https://languagetool.org").unwrap()
	}
	
	#[inline(always)]
	fn languages_default() -> HashMap<Iso639Dash1Alpha2Language, Language>
	{
		hashmap!
		{
			Iso639Dash1Alpha2Language::default() => Language::default(),
		}
	}
}
