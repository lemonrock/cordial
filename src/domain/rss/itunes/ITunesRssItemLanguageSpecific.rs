// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// Data required by iTunes: https://help.apple.com/itc/podcasts_connect/?lang=en#/itcb54353390
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ITunesRssItemLanguageSpecific
{
	#[serde(default)] title: String,
	#[serde(default)] subtitle: String,
	#[serde(default)] summary: String,
	#[serde(default)] enclosure: ResourceUrl,

	#[serde(default)] author: FullName,
	#[serde(default)] block: bool,
	#[serde(default)] season_number: NonZeroNumber,
	#[serde(default)] episode_number: NonZeroNumber,
	#[serde(default)] episode_order: Option<NonZeroNumber>,
	#[serde(default)] episode_type: ITunesRssItemEpisodeType,
	#[serde(default)] explicit: bool,
	#[serde(default)] artwork: Option<ResourceUrl>,
	#[serde(default)] close_captioned: bool,
}

impl ITunesRssItemLanguageSpecific
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &'a Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "subtitle", &self.subtitle.trim())?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "summary", &self.summary.trim())?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "title", &self.title.trim())?;
		
		let mp3UrlData = ResourceReference
		{
			resource: self.enclosure.clone(),
			tag: ResourceTag::audio_mp3
		}.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
		mp3UrlData.validateIsMp3()?;
		
		let length = "length".xml_u64_attribute(mp3UrlData.size());
		let enclosureAttributes =
		[
			"url".xml_str_attribute(mp3UrlData.url_str()),
			length.borrow(),
			"type".xml_str_attribute("audio/mpeg"),
		];
		eventWriter.writeUnprefixedTextElement(namespace, &enclosureAttributes, "enclosure", &self.title.trim())?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "author", &self.author)?;
		
		if self.block
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "block", ITunesRssChannel::ITunesBooleanYes)?;
		}
		
		let durationInSeconds = mp3UrlData.durationInSeconds()?;
		const SecondsInAMinute: u64 = 60;
		const MinutesInAnHour: u64 = 60;
		const SecondsInAnHour: u64 = SecondsInAMinute * MinutesInAnHour;
		let hours = durationInSeconds / SecondsInAnHour;
		let remainingSeconds = durationInSeconds % SecondsInAnHour;
		let minutes = remainingSeconds / SecondsInAMinute;
		let seconds = remainingSeconds % SecondsInAMinute;
		let formattedDuration = if hours > 0
		{
			format!("{}:{:02}:{:02}", hours, minutes, seconds)
		}
		else if minutes > 9
		{
			format!("{:02}:{:02}", minutes, seconds)
		}
		else if minutes > 0
		{
			format!("{}:{:02}", minutes, seconds)
		}
		else
		{
			format!("{}", seconds)
		};
		eventWriter.writePrefixedTextElementString(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "duration", formattedDuration)?;
		
		eventWriter.writePrefixedTextElementU16(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "episode", self.episode_number.0)?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "episodeType", self.episode_type.to_str())?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "explicit", ITunesRssChannel::explicitness(self.explicit))?;
		
		if let Some(ref artwork) = self.artwork
		{
			let resource = artwork.resourceMandatory(resources)?;
			let urlData = resource.findITunesRssArtwork(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "artwork", urlData.url_str())?;
		}
		
		if self.close_captioned
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "isCloseCaptioned", ITunesRssChannel::ITunesBooleanYes)?;
		}
		
		if let Some(episodeOrder) = self.episode_order
		{
			eventWriter.writePrefixedTextElementU16(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "order", episodeOrder.0)?;
		}
		
		eventWriter.writePrefixedTextElementU16(namespace, emptyAttributes, ITunesRssChannel::ITunesNamespacePrefix, "season", self.season_number.0)
	}
}
