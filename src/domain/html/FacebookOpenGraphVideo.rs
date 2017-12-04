// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FacebookOpenGraphVideo
{
	#[serde(default)] pub(crate) actors: Vec<FacebookOpenGraphVideoActor>,
	#[serde(default)] pub(crate) directors: Vec<ResourceUrl>,
	#[serde(default)] pub(crate) writers: Vec<ResourceUrl>,
	#[serde(default)] pub(crate) duration_in_seconds: u64,
	pub(crate) release_date: DateTime<Utc>,
	#[serde(default)] pub(crate) tags: HashSet<String>,
}

impl FacebookOpenGraphVideo
{
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, languageData: &LanguageData) -> Result<(), CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;

		for actor in self.actors.iter()
		{
			actor.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, languageData)?;
		}

		for director in self.directors.iter()
		{
			let url = director.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::profile)?;
			endHeadNodes.push(meta_with_property_and_content("audioVideo:director", url.as_str()));
		}

		for writer in self.writers.iter()
		{
			let url = writer.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::profile)?;
			endHeadNodes.push(meta_with_property_and_content("audioVideo:writer", url.as_str()));
		}

		if self.duration_in_seconds != 0
		{
			endHeadNodes.push(meta_with_property_and_content("audioVideo:duration", &format!("{}", self.duration_in_seconds)));
		}

		endHeadNodes.push(meta_with_property_and_content("audioVideo:release_date", &self.release_date.to_rfc3339()));

		for tag in self.tags.iter()
		{
			let translatedTag = languageData.facebookOpenGraphVideoTagTranslation(tag);
			endHeadNodes.push(meta_with_property_and_content("audioVideo:tag", translatedTag));
		}

		Ok(())
	}
}
