// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// Sourced from https://github.com/w3c/manifest/wiki/Categories
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum WebAppManifestCategory
{
	books,
	business,
	education,
	entertainment,
	finance,
	fitness,
	food,
	games,
	government,
	health,
	kids,
	lifestyle,
	magazines,
	medical,
	music,
	navigation,
	news,
	personalization,
	photo,
	politics,
	productivity,
	security,
	shopping,
	social,
	sports,
	travel,
	utilities,
	weather,
}

impl Default for WebAppManifestCategory
{
	#[inline(always)]
	fn default() -> Self
	{
		WebAppManifestCategory::news
	}
}
