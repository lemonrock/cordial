// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum SafariTouchStartUpImage
{
	#[serde(rename = "320 x 460")] _320x460,
	#[serde(rename = "640 x 920")] _640x920,
	#[serde(rename = "640 x 1096")] _640x1096,
	#[serde(rename = "750 x 1294")] _750x1294,
	#[serde(rename = "1182 x 2208")] _1182x2208,
	#[serde(rename = "1242 x 2148")] _1242x2148,
	#[serde(rename = "748 x 1024")] _748x1024,
	#[serde(rename = "768 x 1004")] _768x1004,
	#[serde(rename = "1496 x 2048")] _1496x2048,
	#[serde(rename = "1536 x 2008")] _1536x2008,
}

impl SafariTouchStartUpImage
{
	#[inline(always)]
	fn addLinkNode(&self, endHeadNodes: &mut Vec<UnattachedNode>, url: &ResourceUrl, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<(), CordialError>
	{
		use self::SafariTouchStartUpImage::*;
		
		let (width, height, mediaQuery) = match *self
		{
			_320x460 => (320, 460, "(device-width: 320px) and (device-height: 480px) and (-webkit-device-pixel-ratio: 1)"),
			_640x920 => (640, 920, "(device-width: 320px) and (device-height: 480px) and (-webkit-device-pixel-ratio: 2)"),
			_640x1096 => (640, 1096, "(device-width: 320px) and (device-height: 568px) and (-webkit-device-pixel-ratio: 2)"),
			_750x1294 => (750, 1294, "(device-width: 375px) and (device-height: 667px) and (-webkit-device-pixel-ratio: 2)"),
			_1182x2208 => (1182, 2208, "(device-width: 414px) and (device-height: 736px) and (orientation: landscape) and (-webkit-device-pixel-ratio: 3)"),
			_1242x2148 => (1242, 2148, "(device-width: 414px) and (device-height: 736px) and (orientation: portrait) and (-webkit-device-pixel-ratio: 3)"),
			_748x1024 => (748, 1024, "(device-width: 768px) and (device-height: 1024px) and (orientation: landscape) and (-webkit-device-pixel-ratio: 1)"),
			_768x1004 => (768, 1004, "(device-width: 768px) and (device-height: 1024px) and (orientation: portrait) and (-webkit-device-pixel-ratio: 1)"),
			_1496x2048 => (1496, 2048, "(device-width: 768px) and (device-height: 1024px) and (orientation: landscape) and (-webkit-device-pixel-ratio: 2)"),
			_1536x2008 => (1536, 2008, "(device-width: 768px) and (device-height: 1024px) and (orientation: portrait) and (-webkit-device-pixel-ratio: 2)"),
		};
		
		let urlData = ResourceReference
		{
			resource: url.clone(),
			tag: ResourceTag::width_height_image(width, height),
		}.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		
		urlData.validateIsPng()?;
		
		endHeadNodes.push("link".with_rel_attribute("apple-touch-startup-image").with_attribute("media".str_attribute(mediaQuery)).with_href_attribute(urlData.url_str()));
		
		Ok(())
	}
}
