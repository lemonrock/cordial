// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Language
{
	iso_3166_1_alpha_2_country_code: Iso3166Dash1Alpha2CountryCode,
	language_tool_long_code: String,
	#[serde(default = "Language::host_default")] pub(crate) host: String,
	#[serde(default)] relative_root_url: RelativeRootUrl,
	#[serde(default)] pub(crate) assume_right_to_left_script: bool,
	native_name: String, // Native name for language, with correct Unicode accents, etc. See https://dribbble.com/shots/1202316-Language-menus-with-flags for an example of common Language descriptions
	required_translations: HashMap<RequiredTranslation, Rc<String>>,
}

impl Default for Language
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::RequiredTranslation::*;
		
		Self
		{
			iso_3166_1_alpha_2_country_code: Iso3166Dash1Alpha2CountryCode::US,
			language_tool_long_code: "en-US".to_owned(),
			host: Self::host_default(),
			relative_root_url: RelativeRootUrl::default(),
			assume_right_to_left_script: false,
			native_name: "English".to_owned(),
			required_translations: hashmap!
			{
				missing_image_fallback => Rc::new("Unfortunately, this content is unavailable at this time.".to_owned()),
			},
		}
	}
}

impl Language
{
	#[inline(always)]
	pub(crate) fn iso3166Dash1Alpha2CountryCode(&self) -> Iso3166Dash1Alpha2CountryCode
	{
		self.iso_3166_1_alpha_2_country_code
	}
	
	#[inline(always)]
	pub(crate) fn host(&self) -> &str
	{
		&self.host
	}
	
	#[inline(always)]
	pub(crate) fn required_translation(&self, requiredTranslation: RequiredTranslation) -> Result<&Rc<String>, CordialError>
	{
		match self.required_translations.get(&requiredTranslation)
		{
			None => Err(CordialError::Configuration(format!("Missing translation for '{:?}'", requiredTranslation))),
			Some(translation) => Ok(translation)
		}
	}
	
	#[inline(always)]
	pub(crate) fn baseUrl(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, is_for_amp: bool) -> Result<Url, CordialError>
	{
		let relative_root_url = if is_for_amp
		{
			self.amp_relative_root_url(iso639Dash1Alpha2Language)
		}
		else
		{
			self.relative_root_url(iso639Dash1Alpha2Language)
		};
		let formattedUrl = format!("https://{}{}", &self.host, relative_root_url);
		let parsed = Url::parse(&formattedUrl);
		let result = parsed.context(format!("either the host '{}' or relative root url '{}' is invalid for the language '{}'", &self.host, relative_root_url, self.iso3166Dash1Alpha2CountryCode()))?;
		Ok(result)
	}
	
	#[inline(always)]
	pub(crate) fn robotsTxtRelativeRootUrls(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Vec<Cow<'static, str>>
	{
		// TODO: Adjust if amp == canonical
		vec!
		[
			self.relative_root_url(iso639Dash1Alpha2Language),
			self.amp_relative_root_url(iso639Dash1Alpha2Language),
		]
	}
	
	#[inline(always)]
	fn relative_root_url(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Cow<'static, str>
	{
		use self::RelativeRootUrl::*;
		match self.relative_root_url
		{
			host => Cow::Borrowed("/"),
			iso => Cow::Owned(format!("/{}/", iso639Dash1Alpha2Language))
		}
	}
	
	#[inline(always)]
	fn amp_relative_root_url(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Cow<'static, str>
	{
		use self::RelativeRootUrl::*;
		match self.relative_root_url
		{
			host => Cow::Borrowed("/amp/"),
			iso => Cow::Owned(format!("/amp/{}/", iso639Dash1Alpha2Language))
		}
	}
	
	#[inline(always)]
	fn host_default() -> String
	{
		"localhost".to_owned()
	}
}
