// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.



#[serde(deny_unknown_fields)]
#[derive(Deserialize, Default, Debug, Clone)]
pub(crate) struct RssChannelLanguageSpecific
{
	title: String,
	googleplay_summary_description: Option<String>,
	itunes_summary_description: Option<String>,
	summary_description: String,
	copyright: String,
	itunes_subtitle: Option<String>,
	#[serde(default)] googleplay_owner: Option<EMailAddress>, // google defaults to itunes:owner
	#[serde(default)] itunes_owner: Option<EMailAddress>, // not optional if podcasting
	#[serde(default)] managing_editor: EMailAddress,
	#[serde(default)] web_master: EMailAddress,
}

impl RssChannelLanguageSpecific
{
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&'a self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], isUsingFeedly: bool, isForPodcasting: bool) -> Result<(), CordialError>
	{
		{
			let title = self.title.trim();
			
			const ITunesTitleLength: usize = 255;
			if title.chars().count() > ITunesTitleLength
			{
				return Err(CordialError::Configuration(format!("RSS channel title exceeds iTune's maximum of {} characters", ITunesTitleLength)))
			}
			
			eventWriter.writeUnprefixedTextElement(namespace, &emptyAttributes, "title", title)?;
		}
		
		if let Some(ref description) = self.googleplay_summary_description
		{
			let description = description.trim();
			
			const GooglePlayDescriptionLength: usize = 4000;
			if description.chars().count() > GooglePlayDescriptionLength
			{
				return Err(CordialError::Configuration(format!("RSS channel summary_description exceeds Google Play's maximum of {} characters", GooglePlayDescriptionLength)))
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "description", description)?;
		}
		
		if let Some(ref summary) = self.itunes_summary_description
		{
			let summary = summary.trim();
			
			const ITunesSummaryLength: usize = 4000;
			if summary.chars().count() > ITunesSummaryLength
			{
				return Err(CordialError::Configuration(format!("RSS channel summary_description exceeds iTunes's maximum of {} characters", ITunesSummaryLength)))
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "summary", summary)?;
		}
		
		{
			let description = self.summary_description.trim();
			
			if isUsingFeedly
			{
				const FeedlyDescriptionLength: usize = 140;
				if description.chars().count() > FeedlyDescriptionLength
				{
					return Err(CordialError::Configuration(format!("RSS channel summary_description exceeds Feedly's maximum of {} characters", FeedlyDescriptionLength)))
				}
			}
			
			const ITunesDescriptionLength: usize = 4000;
			if description.chars().count() > ITunesDescriptionLength
			{
				return Err(CordialError::Configuration(format!("RSS channel summary_description exceeds iTune's maximum of {} characters", ITunesDescriptionLength)))
			}
			
			eventWriter.writeUnprefixedTextElement(namespace, &emptyAttributes, "description", description)?;
		}
		
		{
			let copyright = self.copyright.trim();
			
			const ITunesCopyrightLength: usize = 255;
			if self.title.chars().count() > ITunesCopyrightLength
			{
				return Err(CordialError::Configuration(format!("RSS channel copyright exceeds iTune's maximum of {} characters", ITunesCopyrightLength)))
			}
			
			eventWriter.writeUnprefixedTextElement(namespace, &emptyAttributes, "copyright", copyright)?;
		}
		
		if let Some(ref subtitle) = self.itunes_subtitle
		{
			let summary = subtitle.trim();
			
			const ITunesSubtitleLength: usize = 255;
			if summary.chars().count() > ITunesSubtitleLength
			{
				return Err(CordialError::Configuration(format!("RSS channel subtitle exceeds iTunes's maximum of {} characters", ITunesSubtitleLength)))
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "summary", summary)?;
		}
		
		if let Some(ref googlePlayOwner) = self.googleplay_owner
		{
			let email = googlePlayOwner.email.trim();
			const GooglePlayEmailLength: usize = 255;
			if email.chars().count() > GooglePlayEmailLength
			{
				return Err(CordialError::Configuration(format!("RSS channel title exceeds Google Play's maximum of {} characters", GooglePlayEmailLength)))
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "email", email)?;
		}
		
		if let Some(ref iTunesOwner) = self.itunes_owner
		{
			eventWriter.writeWithinElement(RssChannel::ITunesNamespacePrefix.prefixes_xml_name("owner"), namespace, emptyAttributes, |eventWriter|
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "email", &iTunesOwner.full_name)?;
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "name", &iTunesOwner.email)
			})?;
		}
		else if isForPodcasting
		{
			return Err(CordialError::Configuration("When a RSS channel is for podcasting, it is not possible to omit the itunes_owner".to_owned()));
		}
		
		eventWriter.writeUnprefixedTextElementEMailAddress(namespace, &emptyAttributes, "managingEditor", &self.managing_editor)?;
		
		eventWriter.writeUnprefixedTextElementEMailAddress(namespace, &emptyAttributes, "webMaster", &self.web_master)
	}
}
