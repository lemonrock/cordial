// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct RssFeedlyChannel
{
	#[serde(default = "RssFeedlyChannel::png_cover_image_default")] png_cover_image: ResourceReference,
	#[serde(default = "RssFeedlyChannel::svg_icon_default")] svg_icon: ResourceReference,
	#[serde(default = "RssFeedlyChannel::svg_logo_default")] svg_logo: ResourceReference,
	#[serde(default = "RssFeedlyChannel::accent_color_default")] accent_color: String, // eg 00FF00
	#[serde(default = "RssFeedlyChannel::related_default")] related: bool,
	#[serde(default = "RssFeedlyChannel::google_analytics_default")] google_analytics: bool,
}

impl Default for RssFeedlyChannel
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			png_cover_image: Self::png_cover_image_default(),
			svg_icon: Self::svg_icon_default(),
			svg_logo: Self::svg_logo_default(),
			accent_color: Self::accent_color_default(),
			related: Self::related_default(),
			google_analytics: Self::google_analytics_default(),
		}
	}
}

impl RssFeedlyChannel
{
	#[inline(always)]
	pub(crate) fn writeXml<'a, 'b: 'a, 'c, W: Write>(&'c self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[Attribute<'c>], primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: &str, resources: &'a BTreeMap<String, Resource>, newResources: &'b Resources) -> XmlWriterResult
	{
		if let Some(url, _response) = self.png_cover_image.urlAndResponse(primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code), UrlTag::default, resources, newResources)
		{
			let attributes =
				[
					Attribute::new(Name::local("image"), url),
				];
			eventWriter.writeEmptyElement(namespace, &attributes, Name::prefixed("cover", "webfeeds"))?;
		}
		
		if let Some(url, _response) = self.svg_icon.urlAndResponse(primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code), UrlTag::default, resources, newResources)
		{
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, "webfeeds", "icon", url)?;
		}
		
		if let Some(url, _response) = self.svg_logo.urlAndResponse(primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code), UrlTag::default, resources, newResources)
		{
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, "webfeeds", "logo", url)?;
		}
		
		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, "webfeeds", "accentColor", self.accentColor)?;
		
		let attributes =
			[
				Attribute::new(Name::local("layout"), "card"),
				Attribute::new(Name::local("target"), "browser"),
			];
		eventWriter.writeEmptyElement(namespace, &attributes, Name::prefixed("analytics", "webfeeds"))?;
		
		let attributes =
			[
				Attribute::new(Name::local("id"), GOOGLE_UA_xxx),
				Attribute::new(Name::local("engine"), "GoogleAnalytics"),
			];
		eventWriter.writeEmptyElement(namespace, &attributes, Name::prefixed("analytics", "webfeeds"))
		
		//add <img> with a class of webfeedsFeaturedVisual for feedly OR if first img > 450px OR feedly will try to poll website for open graph or twitter card
	}
	
	#[inline(always)]
	fn png_cover_image_default() -> ResourceReference
	{
		ResourceReference::internal("/cover.png", Some(UrlTag::largest_image))
	}
	
	#[inline(always)]
	fn svg_icon_default() -> ResourceReference
	{
		ResourceReference::internal("/favicon.svg", Some(UrlTag::default))
	}
	
	#[inline(always)]
	fn svg_logo_default() -> ResourceReference
	{
		ResourceReference::internal("/organization-logo.svg", Some(UrlTag::default))
	}
	
	#[inline(always)]
	fn accent_color_default() -> String
	{
		"00FF00".to_owned()
	}
	
	#[inline(always)]
	fn related_default() -> bool
	{
		true
	}
	
	#[inline(always)]
	fn google_analytics_default() -> bool
	{
		true
	}
}
