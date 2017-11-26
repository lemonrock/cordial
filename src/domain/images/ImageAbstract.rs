// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ImageAbstract
{
	// Used in Image Site Map
	// Is also effectively the tooltip as it is <img title="{}">...
	#[serde(default)] pub(crate) title: Rc<String>,
	#[serde(default)] pub(crate) caption: Rc<String>,
	#[serde(default)] pub(crate) geographic_location: Option<Rc<String>>,
	
	// Used in <img> tag and RSS feed
	// Will also supply Twitter OpenCard twitter:image:alt and Facebook OpenGraph og:image:alt
	#[serde(default)] pub(crate) alt: Rc<String>,
}

impl ImageAbstract
{
	#[inline(always)]
	pub(crate) fn addToImgAttributes(&self, imgAttributes: &mut Vec<Attribute>)
	{
		imgAttributes.push("title".str_attribute(&self.title));
		imgAttributes.push("alt".str_attribute(&self.alt));
	}
}
