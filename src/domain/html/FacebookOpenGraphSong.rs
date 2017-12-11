// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct FacebookOpenGraphSong
{
	// The album which contains this song. This is the URL of a page with og:type music.song
	url: ResourceUrl,
	#[serde(default = "FacebookOpenGraphSong::disc_default")] disc: u8,
	#[serde(default = "FacebookOpenGraphSong::track_default")] track: u8,
}

impl FacebookOpenGraphSong
{
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		let url = self.url.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::music_song)?;
		endHeadNodes.push(meta_with_property_and_content("music:song:url", url.as_str()));
		
		match self.disc
		{
			0 => return Err(CordialError::Configuration("song disc can not be zero".to_owned())),
			1 => (),
			disc @ _ => endHeadNodes.push(meta_with_property_and_content("music:song:disc", &format!("{}", disc))),
		}
		
		match self.track
		{
			0 => return Err(CordialError::Configuration("song track can not be zero".to_owned())),
			track @ _ => endHeadNodes.push(meta_with_property_and_content("music:song:track", &format!("{}", track))),
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn disc_default() -> u8
	{
		1
	}
	
	#[inline(always)]
	fn track_default() -> u8
	{
		1
	}
}
