// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub struct localization
{
	#[serde(with = "url_serde", default = "localization::language_tool_base_url_default")] language_tool_base_url: Url,
	primary_iso_639_1_alpha_2_language_code: Option<String>,
	languages: HashMap<String, language>,
}

impl localization
{
	#[inline(always)]
	fn language_tool_base_url_default() -> Url
	{
		Url::parse("https://languagetool.org").unwrap()
	}
	
	#[inline(always)]
	pub fn language_tool_base_url(&self) -> &Url
	{
		&self.language_tool_base_url
	}
	
	#[inline(always)]
	pub fn primary_iso_639_1_alpha_2_language_code(&self) -> &str
	{
		let iso_639_1_alpha_2_language_code = match self.primary_iso_639_1_alpha_2_language_code
		{
			Some(ref code) => code,
			None => "en",
		};
		
		if !self.languages.contains_key(iso_639_1_alpha_2_language_code)
		{
			panic!("primary_iso_639_1_alpha_2_language_code '{}' does not have a defined language", iso_639_1_alpha_2_language_code);
		}
		iso_639_1_alpha_2_language_code
	}
	
	#[inline(always)]
	pub fn language(&self, iso_639_1_alpha_2_language_code: &str) -> &language
	{
		match self.languages.get(iso_639_1_alpha_2_language_code)
		{
			None => panic!("iso_639_1_alpha_2_language_code '{}' does not have a defined language", iso_639_1_alpha_2_language_code),
			Some(language) => language,
		}
	}
}
