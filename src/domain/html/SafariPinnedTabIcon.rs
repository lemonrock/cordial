// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SafariPinnedTabIcon
{
	url: ResourceUrl,
	css_color: Rc<String>,
}

impl SafariPinnedTabIcon
{
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<(), CordialError>
	{
		let urlData = ResourceReference
		{
			resource: self.url.clone(),
			tag: ResourceTag::default,
		}.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		
		urlData.validateIsSvg()?;
		
		endHeadNodes.push("link".with_rel_attribute("mask-icon").with_href_attribute(urlData.url_str()).with_attribute("color".str_attribute(&self.css_color)));
		
		Ok(())
	}
}
