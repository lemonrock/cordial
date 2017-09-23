// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum ResourceReference
{
	absolute(#[serde(with = "url_serde")] Url),
	internal(String, #[serde(default)] Option<UrlTag>),
}

impl ResourceReference
{
	/// NOTE: The URL returned may be a data: or http: URL as well as a https: URL.
	#[inline(always)]
	pub(crate) fn url<'a, 'b: 'a>(&'a self, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, resources: &'a BTreeMap<String, Resource>) -> Option<&'a Url>
	{
		use self::ResourceReference::*;
		match *self
		{
			absolute(ref url) => Some(url),
			internal(ref resourceRelativeUrl, urlTag) =>
			{
				match resources.get(resourceRelativeUrl)
				{
					None => None,
					Some(resource) =>
					{
						let urlTag = match urlTag
						{
							None => UrlTag::default,
							Some(urlTag) => urlTag,
						};
						resource.url(primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code, &urlTag)
					}
				}
			}
		}
	}
	
	/// NOTE: The URL returned may be a data: or http: URL as well as a https: URL.
	#[inline(always)]
	pub(crate) fn urlAndJsonValue<'a, 'b: 'a>(&'a self, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, resources: &'a BTreeMap<String, Resource>) -> Option<(&'a Url, Option<Rc<JsonValue>>)>
	{
		use self::ResourceReference::*;
		match *self
		{
			absolute(ref url) => Some((url, None)),
			internal(ref resourceRelativeUrl, urlTag) =>
			{
				match resources.get(resourceRelativeUrl)
				{
					None => None,
					Some(resource) =>
					{
						let urlTag = match urlTag
						{
							None => UrlTag::default,
							Some(urlTag) => urlTag,
						};
						match resource.urlAndJsonValue(primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code, &urlTag)
						{
							None => None,
							Some((url, jsonValue)) => Some((url, Some(jsonValue)))
						}
					}
				}
			}
		}
	}
	
	/// NOTE: The URL returned may be a data: or http: URL as well as a https: URL.
	#[inline(always)]
	pub(crate) fn urlAndResponse<'a, 'b: 'a>(&'a self, primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: Option<&str>, resources: &'a BTreeMap<String, Resource>, newResources: &'b Resources) -> Option<(&'a Url, Option<&'a RegularAndPjaxStaticResponse>)>
	{
		use self::ResourceReference::*;
		match *self
		{
			absolute(ref url) => Some((url, None)),
			internal(ref resourceRelativeUrl, urlTag) =>
			{
				match resources.get(resourceRelativeUrl)
				{
					None => None,
					Some(resource) =>
					{
						let urlTag = match urlTag
						{
							None => UrlTag::default,
							Some(urlTag) => urlTag,
						};
						match resource.urlAndResource(primary_iso_639_1_alpha_2_language_code, iso_639_1_alpha_2_language_code, &urlTag, newResources)
						{
							None => None,
							Some((url, response)) => Some((url, Some(response)))
						}
					}
				}
			}
		}
	}
}
