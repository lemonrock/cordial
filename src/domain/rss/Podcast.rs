// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Podcast
{
	#[serde(default)] details: HashMap<Iso639Dash1Alpha2Language, RefCell<PodcastLanguageSpecificRssItemVariant>>,
	enclosure: ResourceUrl,
	#[serde(default)] googleplay_author: Option<FullName>,
	#[serde(default)] itunes_author: Option<FullName>,
	#[serde(default)] googleplay_block: Option<bool>,
	#[serde(default)] itunes_block: bool,
	#[serde(default)] season_number: NonZeroNumber,
	#[serde(default)] episode_number: NonZeroNumber,
	#[serde(default)] episode_order: Option<NonZeroNumber>,
	#[serde(default)] episode_type: ITunesEpisodeType,
	#[serde(default)] googleplay_explicit: Option<bool>,
	#[serde(default)] itunes_explicit: bool,
	#[serde(default)] itunes_artwork: Option<ResourceUrl>,
	#[serde(default)] close_captioned: bool,
}

impl Podcast
{
	#[inline(always)]
	pub(crate) fn withRssHtml(&self, description: Rc<String>, rssHtml: Vec<u8>, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		match self.details.get(&iso639Dash1Alpha2Language)
		{
			None => Err(CordialError::Configuration(format!("No podcast details for language '{:?}', iso639Dash1Alpha2Language", iso639Dash1Alpha2Language))),
			Some(refCell) =>
			{
				let mut borrowed = refCell.borrow_mut();
				borrowed.withRssHtml(description, rssHtml);
				Ok(())
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn titleDescriptionAndContentEncoded<R, User: FnMut(&str, &str, Option<&str>) -> Result<R, CordialError>>(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, mut user: User) -> Result<R, CordialError>
	{
		let languageSpecificRssItemVariant = self.details(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		let borrowed = languageSpecificRssItemVariant.borrow();
		let (title, description, contentEncoded) = borrowed.titleDescriptionAndContentEncoded();
		user(title, description, contentEncoded)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		let languageSpecificRssItemVariant = self.details(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
		languageSpecificRssItemVariant.borrow().writeXml(eventWriter, namespace, emptyAttributes)?;
		
		// non-language specific
		{
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
			eventWriter.writeEmptyElement(namespace, &enclosureAttributes, "enclosure".xml_local_name())?;
			
			if let Some(ref googlePlayAuthor) = self.googleplay_author
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "author", googlePlayAuthor)?;
			}
			
			if let Some(ref iTunesAuthor) = self.itunes_author
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "author", iTunesAuthor)?;
			}
			
			if let Some(googlePlayBlock) = self.googleplay_block
			{
				if googlePlayBlock
				{
					eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "block", "yes")?;
				}
			}
			
			if self.itunes_block
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "block", ITunesBooleanYes)?;
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
			eventWriter.writePrefixedTextElementString(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "duration", formattedDuration)?;
			
			eventWriter.writePrefixedTextElementU16(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "episode", self.episode_number.0)?;
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "episodeType", self.episode_type.to_str())?;
			
			if let Some(googlePlayExplicit) = self.googleplay_explicit
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "explicit", googlePlayExplicitness(googlePlayExplicit))?;
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "explicit", iTunesExplicitness(self.itunes_explicit))?;
			
			if let Some(ref iTunesArtwork) = self.itunes_artwork
			{
				let resource = iTunesArtwork.resourceMandatory(resources)?;
				let urlData = resource.findITunesRssArtwork(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "image", urlData.url_str())?;
			}
			
			if self.close_captioned
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "isCloseCaptioned", ITunesBooleanYes)?;
			}
			
			if let Some(episodeOrder) = self.episode_order
			{
				eventWriter.writePrefixedTextElementU16(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "order", episodeOrder.0)?;
			}
			
			eventWriter.writePrefixedTextElementU16(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "season", self.season_number.0)
		}
	}
	
	#[inline(always)]
	fn details(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&RefCell<PodcastLanguageSpecificRssItemVariant>, CordialError>
	{
		if let Some(languageSpecificRssItemVariant) = self.details.get(&iso639Dash1Alpha2Language)
		{
			Ok(languageSpecificRssItemVariant)
		}
		else if let Some(languageSpecificRssItemVariant) = self.details.get(&fallbackIso639Dash1Alpha2Language)
		{
			Ok(languageSpecificRssItemVariant)
		}
		else
		{
			Err(CordialError::Configuration("No PodcastLanguageSpecificRssItemVariant for language or its fallback".to_owned()))
		}
	}
}
