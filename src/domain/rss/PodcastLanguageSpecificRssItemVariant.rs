// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct PodcastLanguageSpecificRssItemVariant
{
	#[serde(default)] title: String,
	#[serde(default)] itunes_title: String,
	#[serde(default)] itunes_subtitle: String,
	#[serde(default, skip_deserializing)] description: Rc<String>,
	#[serde(default)] googleplay_summary_description: Option<String>, // defaults to itunes_summary_description then <description>
	#[serde(default)] itunes_summary_description: Option<String>, // defaults to <description>

	// Could be that this is the same HTML as would accompany a blog post or transcript surrounding this podcast.
	// Is is limited to 255 characters?
	// If generated, then we need to think about how this is constructed.
	#[serde(default, skip_deserializing)] episode_note_html: Vec<u8>,
}

impl Default for PodcastLanguageSpecificRssItemVariant
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			title: Default::default(),
			itunes_title: Default::default(),
			itunes_subtitle: Default::default(),
			description: Default::default(),
			googleplay_summary_description: None,
			itunes_summary_description: None,
			episode_note_html: Default::default(),
		}
	}
}

impl PodcastLanguageSpecificRssItemVariant
{
	#[inline(always)]
	pub(crate) fn withRssHtml(&mut self, description: Rc<String>, rssHtml: Vec<u8>)
	{
		self.description = description;
		self.episode_note_html = rssHtml;
	}
	
	#[inline(always)]
	pub(crate) fn titleDescriptionAndContentEncoded(&self) -> (&str, &str, Option<&str>)
	{
		(&self.title, &self.description, Some(unsafe { from_utf8_unchecked(&self.episode_note_html) }))
	}
	
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		let subtitle = self.itunes_subtitle.trim();
		if subtitle.chars().count() > 255
		{
			return Err(CordialError::Configuration("iTunes subtitle must be no more than 255 characters when trimmed".to_owned()));
		}
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "subtitle", subtitle)?;
		
		if let Some(ref description) = self.googleplay_summary_description
		{
			let description = description.trim();
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "description", description)?;
		}
		
		if let Some(ref description) = self.itunes_summary_description
		{
			let description = description.trim();
			if description.chars().count() > 4000
			{
				return Err(CordialError::Configuration("iTunes summary description must be no more than 4,000 characters when trimmed".to_owned()));
			}
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "summary", description)?;
		}
		
		let title = self.itunes_title.trim();
		if title.chars().count() > 255
		{
			return Err(CordialError::Configuration("iTunes title must be no more than 255 characters when trimmed".to_owned()));
		}
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "title", title)
	}
}
