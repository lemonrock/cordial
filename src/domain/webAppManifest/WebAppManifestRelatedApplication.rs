// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct WebAppManifestRelatedApplication
{
	#[serde(default)] platform: WebAppManifestPlatform,
	#[serde(default, skip_serializing_if = "WebAppManifestRelatedApplication::url_skip_serializing_if")] url: Option<UrlSerde>,
	#[serde(default, skip_serializing_if = "WebAppManifestRelatedApplication::id_skip_serializing_if")] id: Option<String>,
	#[serde(default, skip_serializing_if = "WebAppManifestRelatedApplication::min_version_skip_serializing_if")] min_version: Option<String>,
	#[serde(default)] fingerprints: Vec<WebAppManifestRelatedApplicationFingerprint>,
}

impl WebAppManifestRelatedApplication
{
	#[inline(always)]
	fn url_skip_serializing_if(url: &Option<UrlSerde>) -> bool
	{
		url.is_none()
	}
	
	#[inline(always)]
	fn id_skip_serializing_if(id: &Option<String>) -> bool
	{
		id.is_none()
	}
	
	#[inline(always)]
	fn min_version_skip_serializing_if(min_version: &Option<String>) -> bool
	{
		min_version.is_none()
	}
}
