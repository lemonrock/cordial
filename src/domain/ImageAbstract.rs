// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ImageAbstract
{
	img_alt: String, // Will also supply Twitter OpenCard twitter:image:alt and Facebook OpenGraph og:image:alt
	img_title: Option<String>, // Is effectively the tooltip
	sitemap_title: Option<String>,
	sitemap_caption: Option<String>,
	sitemap_location: Option<String>, // Default will not output
	#[serde(with = "url_serde")] sitemap_license: Option<Url>, // Will default to the site license
}
