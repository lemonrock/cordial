// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct language
{
	iso_3166_1_alpha_2_country_code: String,
	language_tool_long_code: String,
	host: String,
	relative_root_url: String,
}

impl language
{
	#[inline(always)]
	pub fn iso_3166_1_alpha_2_country_code(&self) -> &str
	{
		&self.iso_3166_1_alpha_2_country_code
	}
	
	#[inline(always)]
	pub fn language_tool_long_code(&self) -> &str
	{
		&self.language_tool_long_code
	}
	
	#[inline(always)]
	pub fn host(&self) -> &str
	{
		&self.host
	}
	
	#[inline(always)]
	pub fn relative_root_url(&self) -> &str
	{
		if self.relative_root_url.is_empty()
		{
			"/"
		}
		else
		{
			&self.relative_root_url
		}
	}
	
	#[inline(always)]
	pub fn baseUrl(&self) -> Result<Url, CordialError>
	{
		let relative_root_url = self.relative_root_url();
		let formattedUrl = format!("https://{}{}", &self.host, relative_root_url);
		let parsed = Url::parse(&formattedUrl);
		let result = parsed.context(format!("either the host '{}' or relative root url '{}' is invalid for the language '{}'", &self.host, relative_root_url, self.iso_3166_1_alpha_2_country_code()))?;
		Ok(result)
	}
}
