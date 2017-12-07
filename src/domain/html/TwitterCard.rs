// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TwitterCard
{
	#[serde(default)] pub(crate) site: Option<TwitterAtHandle>,
	#[serde(default)] pub(crate) card: TwitterCardType,
}

impl Default for TwitterCard
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			site: None,
			//creator_or_author: None,
			card: Default::default(),
		}
	}
}

impl TwitterCard
{
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>, articleImage: &Option<(ResourceUrl, Rc<ImageMetaData>)>, articleAudio: Option<&ResourceUrl>, articleVideo: Option<&ResourceUrl>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, languageData: &LanguageData) -> Result<(), CordialError>
	{
		self.card.addTo(endHeadNodes, &self.site, articleImage, articleAudio, articleVideo, resources, fallbackIso639Dash1Alpha2Language, languageData)?;
		
		Ok(())
	}
}
