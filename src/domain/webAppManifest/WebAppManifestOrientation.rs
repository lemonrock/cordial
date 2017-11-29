// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) enum WebAppManifestOrientation
{
	any,
	natural,
	landscape,
	#[serde(rename = "landscape-primary")] landscape_primary,
	#[serde(rename = "landscape-secondary")] landscape_secondary,
	portrait,
	#[serde(rename = "portrait-primary")] portrait_primary,
	#[serde(rename = "portrait-secondary")] portrait_secondary,
}

impl Default for WebAppManifestOrientation
{
	#[inline(always)]
	fn default() -> Self
	{
		WebAppManifestOrientation::any
	}
}
