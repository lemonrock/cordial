// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct SafariStyling
{
	#[serde(default)] touch_icon: Option<FavIcon>,
	#[serde(default)] pinned_tab: Option<SafariPinnedTabIcon>,
	#[serde(default)] startup_images: BTreeMap<SafariTouchStartUpImage, ResourceUrl>,
	#[serde(default)] fullscreen: bool,
	#[serde(default)] status_bar_appearance: SafariStatusBarAppearance,
	#[serde(default)] itunes_app: Option<SafariITunesApp>,
	
}

impl Default for SafariStyling
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			touch_icon: None,
			pinned_tab: None,
			startup_images: Default::default(),
			fullscreen: false,
			status_bar_appearance: Default::default(),
			itunes_app: None,
		}
	}
}

impl SafariStyling
{
	#[inline(always)]
	pub(crate) fn addToStartHeadNodes(&self, startHeadNodes: &mut Vec<UnattachedNode>, htmlAbstract: &HtmlAbstract)
	{
		if self.fullscreen
		{
			startHeadNodes.push(meta_with_name_and_content("apple-mobile-web-app-capable", "yes"));
		}
		
		self.status_bar_appearance.addTo(startHeadNodes);
		
		if let Some(ref safariTitle) = htmlAbstract.safari_web_app_title
		{
			startHeadNodes.push(meta_with_name_and_content("apple-mobile-web-app-title", safariTitle.as_str()));
		}
	}
	
	#[inline(always)]
	pub(crate) fn addToEndHeadNodes(&self, endHeadNodes: &mut Vec<UnattachedNode>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>, resourceUrl: &ResourceUrl) -> Result<(), CordialError>
	{
		if let Some(ref touchIcon) = self.touch_icon
		{
			touchIcon.addLinkNodes(endHeadNodes, "apple-touch-icon", resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		}
		
		if let Some(ref pinnedTab) = self.pinned_tab
		{
			pinnedTab.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		}
		
		for (startup_image, url) in self.startup_images.iter()
		{
			startup_image.addLinkNode(endHeadNodes, url, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		}
		
		if let Some(ref itunes_app) = self.itunes_app
		{
			itunes_app.addToEndHeadNodes(endHeadNodes, resourceUrl);
		}
		
		Ok(())
	}
}
