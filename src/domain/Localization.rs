// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Localization
{
	#[serde(with = "url_serde", default = "Localization::language_tool_base_url_default")] language_tool_base_url: Url,
	#[serde(default = "Localization::primary_iso_639_1_alpha_2_language_code_default")] primary_iso_639_1_alpha_2_language_code: String,
	#[serde(default = "Localization::languages_default")] languages: HashMap<String, Language>,
}

impl Default for Localization
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			language_tool_base_url: Self::language_tool_base_url_default(),
			primary_iso_639_1_alpha_2_language_code: Self::primary_iso_639_1_alpha_2_language_code_default(),
			languages: Self::languages_default(),
		}
	}
}

impl Localization
{
	#[inline(always)]
	pub(crate) fn primaryLanguage(&self) -> Result<&Language, CordialError>
	{
		self.language(&self.primary_iso_639_1_alpha_2_language_code)
	}
	
	#[inline(always)]
	pub(crate) fn language(&self, iso_639_1_alpha_2_language_code: &str) -> Result<&Language, CordialError>
	{
		match self.languages.get(iso_639_1_alpha_2_language_code)
		{
			None => Err(CordialError::Configuration(format!("iso_639_1_alpha_2_language_code '{}' does not have a defined language", iso_639_1_alpha_2_language_code))),
			Some(language) => Ok(language),
		}
	}
	
	#[inline(always)]
	pub(crate) fn otherLanguages(&self, iso_639_1_alpha_2_language_code: &str) -> HashMap<String, Language>
	{
		self.languages.iter().filter(|&(code, _)| code != iso_639_1_alpha_2_language_code).map(|(code, language)| (code.to_owned(), language.clone())).collect()
	}
	
	#[inline(always)]
	pub(crate) fn visitLanguagesWithPrimaryFirst<F: FnMut(&LanguageData, bool) -> Result<(), CordialError>>(&self, mut visitor: F) -> Result<(), CordialError>
	{
		visitor(&LanguageData::new(&self.primary_iso_639_1_alpha_2_language_code, self.primaryLanguage()?), true)?;
		for (iso_639_1_alpha_2_language_code, language) in self.languages.iter()
		{
			if iso_639_1_alpha_2_language_code != &self.primary_iso_639_1_alpha_2_language_code
			{
				visitor(&LanguageData::new(&iso_639_1_alpha_2_language_code, language), false)?;
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
	fn primary_iso_639_1_alpha_2_language_code_default() -> String
	{
		"en".to_owned()
	}
	
	#[inline(always)]
	fn languages_default() -> HashMap<String, Language>
	{
		hashmap!
		{
			Self::primary_iso_639_1_alpha_2_language_code_default() => Language::default(),
		}
	}
}
