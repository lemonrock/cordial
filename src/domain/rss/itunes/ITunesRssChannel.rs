// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ITunesRssChannel
{
	#[serde(default, rename = "type")] type_: ITunesRssChannelType,
	#[serde(default)] author: FullName,
	#[serde(default)] block: bool,
	#[serde(default)] categorisation: ITunesRssChannelCategorisation,
	#[serde(default)] complete: bool,
	#[serde(default)] explicit: bool,
	#[serde(default)] artwork: Option<ResourceUrl>,
	#[serde(default)] new_feed_url: Option<UrlSerde>,
	#[serde(default)] owner: EMailAddress,
}

impl Default for ITunesRssChannel
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			type_: Default::default(),
			author: Default::default(),
			block: false,
			categorisation: Default::default(),
			complete: false,
			explicit: false,
			artwork: None,
			new_feed_url: None,
			owner: Default::default(),
		}
	}
}

impl ITunesRssChannel
{
	pub(crate) const ITunesNamespacePrefix: &'static str = "itunes";
	
	pub(crate) const ITunesNamespaceUrl: &'static str = "http://www.itunes.com/dtds/podcast-1.0.dtd";
	
	pub(crate) const ITunesBooleanYes: &'static str = "Yes";
	
	#[inline(always)]
	pub(crate) fn writeXml<'a, 'b: 'a, 'c, W: Write>(&'c self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'c>], fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, resources: &'a Resources) -> Result<(), CordialError>
	{
		let _iso639Dash1Alpha2Language = Some(iso639Dash1Alpha2Language);
		
		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::ITunesNamespacePrefix, "type", self.type_.to_str())?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ITunesNamespacePrefix, "author", &self.author)?;
		
		if self.block
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ITunesNamespacePrefix, "block", Self::ITunesBooleanYes)?;
		}
		
		let category = self.categorisation.toCategoryStr();
		let categoryAttributes = [ "text".xml_str_attribute(category) ];
		match self.categorisation.toSubCategoryStr()
		{
			None => eventWriter.writeEmptyElement(namespace, &categoryAttributes, Self::ITunesNamespacePrefix.prefixes_xml_name("category"))?,
			Some(subCategory) =>
			{
				eventWriter.writeWithinElement(Self::ITunesNamespacePrefix.prefixes_xml_name("category"), namespace, &categoryAttributes, |eventWriter|
				{
					eventWriter.writeEmptyElement(namespace, &[ "text".xml_str_attribute(subCategory) ], Self::ITunesNamespacePrefix.prefixes_xml_name("category"))
				})?;
			}
		}
		
		if self.complete
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ITunesNamespacePrefix, "complete", Self::ITunesBooleanYes)?;
		}
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ITunesNamespacePrefix, "explicit", Self::explicitness(self.explicit))?;
		
		if let Some(ref artwork) = self.artwork
		{
			let resource = artwork.resourceMandatory(resources)?;
			let urlData = resource.findITunesRssArtwork(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ITunesNamespacePrefix, "artwork", urlData.url_str())?;
		}
		
		if let Some(ref new_feed_url) = self.new_feed_url
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ITunesNamespacePrefix, "new-feed-url", new_feed_url.0.as_ref())?;
		}
		
		eventWriter.writeWithinElement(Self::ITunesNamespacePrefix.prefixes_xml_name("owner"), namespace, emptyAttributes, |eventWriter|
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ITunesNamespacePrefix, "email", &self.owner.full_name)?;
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ITunesNamespacePrefix, "name", &self.owner.email)
		})?;
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn explicitness(isExplicit: bool) -> &'static str
	{
		if isExplicit { "explicit" } else { "clean" }
	}
}
