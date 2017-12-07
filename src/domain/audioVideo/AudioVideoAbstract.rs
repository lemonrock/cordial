// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AudioVideoAbstract
{
	pub(crate) title: String,
	pub(crate) site_map_description: String,
	
	// For Podcast RSS
	#[serde(default)] pub(crate) itunes_title: String,
	#[serde(default)] pub(crate) itunes_subtitle: String,
	#[serde(default)] pub(crate) googleplay_summary_description: Option<String>, // defaults to itunes_summary_description then <description>
	#[serde(default)] pub(crate) itunes_summary_description: Option<String>, // defaults to <description>
}

impl AudioVideoAbstract
{
	#[inline(always)]
	pub(crate) fn writeSiteMapXml<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		self.writeXmlForSiteMapTitle(eventWriter, namespace, emptyAttributes)?;
		
		self.writeXmlForSiteMapDescription(eventWriter, namespace, emptyAttributes)
	}
	
	#[inline(always)]
	fn writeXmlForSiteMapTitle<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		eventWriter.writeCDataElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix.prefixes_xml_name("title"), &self.title)
	}
	
	#[inline(always)]
	fn writeXmlForSiteMapDescription<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		if self.site_map_description.chars().count() > 2048
		{
			return Err(CordialError::Configuration("Video site map description can not exceed 2048 characters".to_owned()));
		}
		
		eventWriter.writeCDataElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix.prefixes_xml_name("description"), &self.site_map_description)
	}
	
	#[inline(always)]
	pub(crate) fn writePodcastRssXml<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
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
