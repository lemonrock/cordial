// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct PodcastRssChannel
{
	#[serde(default, rename = "type")] type_: ITunesChannelType,
	#[serde(default)] googleplay_author: Option<FullName>,
	#[serde(default)] itunes_author: FullName,
	#[serde(default)] itunes_block: bool,
	#[serde(default)] googleplay_category: Option<GooglePlayCategory>,
	#[serde(default)] itunes_category_and_subcategory: ITunesCategoryAndSubCategory,
	#[serde(default)] itunes_complete: bool,
	#[serde(default)] googleplay_explicit: Option<bool>,
	#[serde(default)] itunes_explicit: bool,
	#[serde(default)] itunes_new_feed_url: Option<UrlSerde>,
}

impl Default for PodcastRssChannel
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			type_: Default::default(),
			googleplay_author: None,
			itunes_author: Default::default(),
			itunes_block: false,
			googleplay_category: None,
			itunes_category_and_subcategory: Default::default(),
			itunes_complete: false,
			googleplay_explicit: None,
			itunes_explicit: false,
			itunes_new_feed_url: None,
		}
	}
}

impl PodcastRssChannel
{
	#[inline(always)]
	pub(crate) fn writeXml<'a, 'b: 'a, 'c, W: Write>(&'c self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'c>], fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, resources: &'a Resources, artwork: &ResourceUrl) -> Result<(), CordialError>
	{
		let _iso639Dash1Alpha2Language = Some(iso639Dash1Alpha2Language);
		
		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, RssChannel::ITunesNamespacePrefix, "type", self.type_.to_str())?;
		
		if let Some(ref googleplay_author) = self.googleplay_author
		{
			let googleplay_author = googleplay_author.trim();
			let GooglePlayAuthorLength = 255;
			if googleplay_author.chars().count() > GooglePlayAuthorLength
			{
				return Err(CordialError::Configuration(format!("RSS googleplay_author exceeds Google Play's maximum of {} characters", GooglePlayAuthorLength)));
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "author", googleplay_author)?;
		}
		
		{
			let itunes_author = self.itunes_author.trim();
			let ITunesAuthorLength = 255;
			if itunes_author.chars().count() > ITunesAuthorLength
			{
				return Err(CordialError::Configuration(format!("RSS itunes_author exceeds itunes's maximum of {} characters", ITunesAuthorLength)));
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "author", &self.itunes_author)?;
		}
		
		if self.itunes_block
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "block", ITunesBooleanYes)?;
		}
		
		if let Some(googlePlayCategory) = self.googleplay_category
		{
			eventWriter.writeEmptyElement(namespace, &[ "text".xml_str_attribute(googlePlayCategory.toCategoryStr()) ], RssChannel::GooglePlayNamespacePrefix.prefixes_xml_name("category"))?;
		}
		
		let category = self.itunes_category_and_subcategory.toCategoryStr();
		let categoryAttributes = [ "text".xml_str_attribute(category) ];
		match self.itunes_category_and_subcategory.toSubCategoryStr()
		{
			None => eventWriter.writeEmptyElement(namespace, &categoryAttributes, RssChannel::ITunesNamespacePrefix.prefixes_xml_name("category"))?,
			Some(subCategory) =>
			{
				eventWriter.writeWithinElement(RssChannel::ITunesNamespacePrefix.prefixes_xml_name("category"), namespace, &categoryAttributes, |eventWriter|
				{
					eventWriter.writeEmptyElement(namespace, &[ "text".xml_str_attribute(subCategory) ], RssChannel::ITunesNamespacePrefix.prefixes_xml_name("category"))
				})?;
			}
		}
		
		if self.itunes_complete
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "complete", ITunesBooleanYes)?;
		}
		
		if let Some(googlePlayExplicit) = self.googleplay_explicit
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "explicit", googlePlayExplicitness(googlePlayExplicit))?;
		}
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "explicit", iTunesExplicitness(self.itunes_explicit))?;
		
		{
			let resource = artwork.resourceMandatory(resources)?;
			
			let googlePlayArtworkUrlData = resource.findGooglePlayRssArtwork(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
			let googlePlayArtworkUrl = googlePlayArtworkUrlData.url_str();
			const GooglePlayArtworkUrlLength: usize = 2048;
			if googlePlayArtworkUrl.chars().count() > GooglePlayArtworkUrlLength
			{
				return Err(CordialError::Configuration(format!("RSS googleplay artwork URL exceeds Google Play's maximum of {} characters", GooglePlayArtworkUrlLength)));
			}
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::GooglePlayNamespacePrefix, "image", googlePlayArtworkUrl)?;
			
			let iTunesArtworkUrlData = resource.findITunesRssArtwork(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "image", iTunesArtworkUrlData.url_str())?;
		}
		
		if let Some(ref iTunesNewFeedUrl) = self.itunes_new_feed_url
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, RssChannel::ITunesNamespacePrefix, "new-feed-url", iTunesNewFeedUrl.0.as_ref())?;
		}
		
		Ok(())
	}
}
