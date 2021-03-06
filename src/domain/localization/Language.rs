// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Language
{
	iso_3166_1_alpha_2_country_code: Iso3166Dash1Alpha2CountryCode,
	pub(crate) facebook_open_graph_locale: FacebookOpenGraphLocale,
	language_tool_long_code: String,
	#[serde(default = "Language::host_default")] pub(crate) host: String,
	#[serde(default)] relative_root_url: RelativeRootUrl,
	#[serde(default)] pub(crate) assume_right_to_left_script: bool,
	native_name: String, // Native name for language, with correct Unicode accents, etc. See https://dribbble.com/shots/1202316-Language-menus-with-flags for an example of common Language descriptions
	required_translations: HashMap<RequiredTranslation, Rc<String>>,
	facebook_open_graph_video_actor_role_translations: HashMap<String, String>,
	facebook_open_graph_video_tag_translations: HashMap<String, String>,
	facebook_open_graph_article_tag_translations: HashMap<String, String>,
	facebook_open_graph_article_section_translations: HashMap<String, String>,
	facebook_open_graph_book_tag_translations: HashMap<String, String>,
}

impl Default for Language
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			iso_3166_1_alpha_2_country_code: Default::default(),
			facebook_open_graph_locale: Default::default(),
			language_tool_long_code: "en-US".to_owned(),
			host: Self::host_default(),
			relative_root_url: RelativeRootUrl::default(),
			assume_right_to_left_script: false,
			native_name: "English".to_owned(),
			required_translations: RequiredTranslation::englishTranslations(),
			facebook_open_graph_video_actor_role_translations: Default::default(),
			facebook_open_graph_video_tag_translations: Default::default(),
			facebook_open_graph_article_tag_translations: Default::default(),
			facebook_open_graph_article_section_translations: Default::default(),
			facebook_open_graph_book_tag_translations: Default::default(),
		}
	}
}

impl Language
{
	#[inline(always)]
	pub(crate) fn facebookOpenGraphLocaleStr(&self) -> &str
	{
		self.facebook_open_graph_locale.to_str()
	}
	
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
	pub(crate) fn dir(&self) -> Dir
	{
		if self.assume_right_to_left_script
		{
			Dir::RightToLeft
		}
		else
		{
			Dir::LeftToRight
		}
	}
	
	#[inline(always)]
	pub(crate) fn requiredTranslation(&self, requiredTranslation: RequiredTranslation) -> Result<&Rc<String>, CordialError>
	{
		match self.required_translations.get(&requiredTranslation)
		{
			None => Err(CordialError::Configuration(format!("Missing translation for '{:?}'", requiredTranslation))),
			Some(translation) => Ok(translation)
		}
	}
	
	#[inline(always)]
	pub(crate) fn translation<'a: 'b, 'b>(translations: &'a HashMap<String, String>, key: &'b str) -> &'b str
	{
		translations.get(key).map(|translation| translation.as_str()).unwrap_or(key)
	}
	
	#[inline(always)]
	pub(crate) fn facebookOpenGraphVideoActorRoleTranslation<'a: 'b, 'b>(&'a self, role: &'b str) -> &'b str
	{
		Self::translation(&self.facebook_open_graph_video_actor_role_translations, role)
	}
	
	#[inline(always)]
	pub(crate) fn facebookOpenGraphVideoTagTranslation<'a: 'b, 'b>(&'a self, tag: &'b str) -> &'b str
	{
		Self::translation(&self.facebook_open_graph_video_tag_translations, tag)
	}
	
	#[inline(always)]
	pub(crate) fn facebookOpenGraphArticleTagTranslation<'a: 'b, 'b>(&'a self, tag: &'b str) -> &'b str
	{
		Self::translation(&self.facebook_open_graph_article_tag_translations, tag)
	}
	
	#[inline(always)]
	pub(crate) fn facebookOpenGraphArticleSectionTranslation<'a: 'b, 'b>(&'a self, section: &'b str) -> &'b str
	{
		Self::translation(&self.facebook_open_graph_article_section_translations, section)
	}
	
	#[inline(always)]
	pub(crate) fn facebookOpenGraphBookTagTranslation<'a: 'b, 'b>(&'a self, tag: &'b str) -> &'b str
	{
		Self::translation(&self.facebook_open_graph_book_tag_translations, tag)
	}
	
	#[inline(always)]
	fn baseUrl(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, isForAmp: bool) -> Result<Url, CordialError>
	{
		let relativeRootUrl = if isForAmp
		{
			self.ampRelativeRootUrl(iso639Dash1Alpha2Language)
		}
		else
		{
			self.relativeRootUrl(iso639Dash1Alpha2Language)
		};
		let formattedUrl = format!("https://{}{}", &self.host, relativeRootUrl);
		let parsed = Url::parse(&formattedUrl);
		let result = parsed.context(format!("either the host '{}' or relative root url '{}' is invalid for the language '{}'", &self.host, relativeRootUrl, self.iso3166Dash1Alpha2CountryCode()))?;
		Ok(result)
	}
	
	#[inline(always)]
	pub(crate) fn robotsTxtRelativeRootUrls(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Vec<Cow<'static, str>>
	{
		// TODO: Adjust if amp == canonical
		vec!
		[
			self.relativeRootUrl(iso639Dash1Alpha2Language),
			self.ampRelativeRootUrl(iso639Dash1Alpha2Language),
		]
	}
	
	#[inline(always)]
	fn relativeRootUrl(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Cow<'static, str>
	{
		use self::RelativeRootUrl::*;
		match self.relative_root_url
		{
			host => Cow::Borrowed("/"),
			iso => Cow::Owned(format!("/{}/", iso639Dash1Alpha2Language))
		}
	}
	
	#[inline(always)]
	fn ampRelativeRootUrl(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Cow<'static, str>
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
