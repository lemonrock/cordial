// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ITunesRssChannel
{
	#[serde(default)] podcast_type: ITunesRssChannelPodcastType,
	#[serde(default)] podcast_summary: HashMap<Iso639Dash1Alpha2Language, String>,
}

impl Default for ITunesRssChannel
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			podcast_type: Default::default(),
			podcast_summary: Default::default(),
		}
	}
}

impl ITunesRssChannel
{
	pub(crate) const ITunesNamespacePrefix: &'static str = "itunes";
	
	pub(crate) const ITunesNamespaceUrl: &'static str = "http://www.itunes.com/dtds/podcast-1.0.dtd";
	
	#[inline(always)]
	pub(crate) fn writeXml<'a, 'b: 'a, 'c, W: Write>(&'c self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'c>], _fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, _resources: &'a Resources) -> Result<(), CordialError>
	{
		let _iso639Dash1Alpha2Language = Some(iso639Dash1Alpha2Language);
		
		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::ITunesNamespacePrefix, "type", self.podcast_type.to_str())?;
		
//		if self.podcast_summary.chars().count() > 4000
//		{
//			return Err(CordialError::Configuration("A podcast summary should not exceed 4,000 characters".to_owned());
//		}
//		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::ITunesNamespacePrefix, "type", &self.podcast_summary)?;
		
		Ok(())
	}
}

/*
For text encoding, use plain text UTF-8 encoding for your feed (no markup or HTML). Tag values are limited to 255 characters, except for <description> and <itunes:summary>, which can be up to 4000 characters. Don’t add leading or trailing spaces to your values. Enclose all portions of your XML that contain embedded links in a CDATA section to prevent formatting issues, and to ensure proper link functionality. For example:
*/

