// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Language
{
	iso_3166_1_alpha_2_country_code: String,
	language_tool_long_code: String,
	#[serde(default = "Language::host_default")] pub(crate) host: String,
	#[serde(default)] relative_root_url: RelativeRootUrl,
}

impl Default for Language
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			iso_3166_1_alpha_2_country_code: "US".to_owned(),
			language_tool_long_code: "en-US".to_owned(),
			host: Self::host_default(),
			relative_root_url: RelativeRootUrl::default(),
		}
	}
}

impl Language
{
	#[inline(always)]
	pub(crate) fn iso_3166_1_alpha_2_country_code(&self) -> &str
	{
		&self.iso_3166_1_alpha_2_country_code
	}
	
	#[inline(always)]
	pub(crate) fn host(&self) -> &str
	{
		&self.host
	}
	
	#[inline(always)]
	pub(crate) fn relative_root_url(&self, iso_639_1_alpha_2_language_code: &str) -> Cow<'static, str>
	{
		use self::RelativeRootUrl::*;
		match self.relative_root_url
		{
			host => Cow::Borrowed("/"),
			iso => Cow::Owned(format!("/{}/", iso_639_1_alpha_2_language_code))
		}
	}
	
	#[inline(always)]
	pub(crate) fn baseUrl(&self, iso_639_1_alpha_2_language_code: &str) -> Result<Url, CordialError>
	{
		let relative_root_url = self.relative_root_url(iso_639_1_alpha_2_language_code);
		let formattedUrl = format!("https://{}{}", &self.host, relative_root_url);
		let parsed = Url::parse(&formattedUrl);
		let result = parsed.context(format!("either the host '{}' or relative root url '{}' is invalid for the language '{}'", &self.host, relative_root_url, self.iso_3166_1_alpha_2_country_code()))?;
		Ok(result)
	}
	
	#[inline(always)]
	fn host_default() -> String
	{
		"localhost".to_owned()
	}
}
