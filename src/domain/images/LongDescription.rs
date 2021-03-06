// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct LongDescription
{
	resource: ResourceUrl,
	// Without leading '#'
	#[serde(default)] id: Option<String>,
}

impl LongDescription
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn addToImgAttributes(&self, imgAttributes: &mut Vec<Attribute>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<(), CordialError>
	{
		let url = ResourceReference
		{
			resource: self.resource.clone(),
			tag: ResourceTag::default,
		}.urlMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		let url = url.as_str();
		
		let attribute = if let Some(ref id) = self.id
		{
			"longdesc".string_attribute(format!("{}#{}", url, id))
		}
		else
		{
			"longdesc".str_attribute(url)
		};
		imgAttributes.push(attribute);
		
		Ok(())
	}
}
