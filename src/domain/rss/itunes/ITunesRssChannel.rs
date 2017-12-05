// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ITunesRssChannel
{
	#[serde(default, rename = "type")] type_: ITunesRssChannelType,
	#[serde(default)] details: HashMap<Iso639Dash1Alpha2Language, ITunesRssChannelLanguageSpecific>,
	#[serde(default)] author: FullName,
	#[serde(default)] block: bool,
	//#[serde(default)] category: XXX,
	#[serde(default)] complete: bool,
	#[serde(default)] explicit: bool,
	#[serde(default)] artwork: Option<ResourceUrl>,
}

impl Default for ITunesRssChannel
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			type_: Default::default(),
			details: Default::default(),
			author: Default::default(),
			block: false,
			//category: false,
			complete: false,
			explicit: false,
			artwork: None,
		}
	}
}

impl ITunesRssChannel
{
	pub(crate) const ITunesNamespacePrefix: &'static str = "itunes";
	
	pub(crate) const ITunesNamespaceUrl: &'static str = "http://www.itunes.com/dtds/podcast-1.0.dtd";
	
	#[inline(always)]
	pub(crate) fn writeXml<'a, 'b: 'a, 'c, W: Write>(&'c self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'c>], fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, _resources: &'a Resources) -> Result<(), CordialError>
	{
		const ITunesBooleanYes: &'static str = "Yes";
		
		let _iso639Dash1Alpha2Language = Some(iso639Dash1Alpha2Language);
		
		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::ITunesNamespacePrefix, "type", self.type_.to_str())?;
		
		match self.details.get(&iso639Dash1Alpha2Language)
		{
			Some(details) => Self::writeDetailsXml(eventWriter, namespace, emptyAttributes, details)?,
			None => match self.details.get(&fallbackIso639Dash1Alpha2Language)
			{
				Some(details) => Self::writeDetailsXml(eventWriter, namespace, emptyAttributes, details)?,
				None => (),
			}
		}
		
		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::ITunesNamespacePrefix, "author", &self.author)?;
		
		if self.block
		{
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::ITunesNamespacePrefix, "block", ITunesBooleanYes)?;
		}
		
		// category
		
		if self.complete
		{
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::ITunesNamespacePrefix, "complete", ITunesBooleanYes)?;
		}
		
		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::ITunesNamespacePrefix, "explicit", if self.explicit { "explicit" } else { "clean" })?;
		
		// artwork
		
		// <itunes:new-feed-url>
		
		// <itunes:owner> (email, name)
		
		// To think about: <link>http://www.example.com/podcasts/everything/index.html</link> - for podcasts, this is a link to the Podcast home page, not the RSS feed link
		
		Ok(())
	}
	
	#[inline(always)]
	fn writeDetailsXml<'c, W: Write>(eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'c>], details: &ITunesRssChannelLanguageSpecific) -> Result<(), CordialError>
	{
		if let Some(ref summary) = details.summary
		{
			let summary = summary.trim();
			
			if summary.chars().count() > 4000
			{
				return Err(CordialError::Configuration("A podcast summary should not exceed 4,000 characters".to_owned()));
			}
			
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::ITunesNamespacePrefix, "summary", summary)?;
		}
		
		let subtitle = details.subtitle.trim();
		
		if subtitle.chars().count() > 255
		{
			return Err(CordialError::Configuration("A podcast subtitle should not exceed 255 characters".to_owned()));
		}
		
		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::ITunesNamespacePrefix, "subtitle", &subtitle)
	}
}
