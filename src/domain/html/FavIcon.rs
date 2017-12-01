// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub struct FavIcon
{
	#[serde(default = "FavIcon::url_default")] url: ResourceUrl,
	#[serde(default = "FavIcon::sizes_default")] sizes: BTreeSet<u32>,
}

impl Default for FavIcon
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			url: Self::url_default(),
			sizes: Self::sizes_default(),
		}
	}
}

impl FavIcon
{
	#[inline(always)]
	fn addLinkNodes(&self, endHeadNodes: &mut Vec<UnattachedNode>, linkRelation: &str, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<(), CordialError>
	{
		for iconSize in self.sizes.iter()
		{
			let width = *iconSize;
			let height = width;
			
			let urlData = ResourceReference
			{
				resource: self.url.clone(),
				tag: ResourceTag::width_height_image(width, height),
			}.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			
			urlData.validateIsPng()?;
			
			endHeadNodes.push("link".with_rel_attribute(linkRelation).with_attribute("type".str_attribute(urlData.mimeType().as_ref())).with_href_attribute(urlData.url_str()).with_attribute("sizes".string_attribute(format!("{}x{}", width, height))));
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn url_default() -> ResourceUrl
	{
		ResourceUrl::string("/favicon.png")
	}
	
	#[inline(always)]
	fn sizes_default() -> BTreeSet<u32>
	{
		btreeset!
		{
			16,
			
			32,
			
			// 48, 64 are not used by anything
			
			// 96 is only used by Google TV which is dead
			
			// 128 is only used by Chrome web store which does not use favicons
			
			// 160 is for defunct versions of Opera 12 Speed Dial
			
			// 192 was for Android but newer versions seem to use the Web App Manifest
			192,
			
			194,
			
			// ? 196
			
			// 228 Opera Coast
			
			// Firefox uses last declared picture
		}
	}
}
