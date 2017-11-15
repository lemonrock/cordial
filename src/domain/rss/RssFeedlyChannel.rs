// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct RssFeedlyChannel
{
	#[serde(default = "RssFeedlyChannel::png_cover_image_default")] png_cover_image: ResourceReference,
	#[serde(default = "RssFeedlyChannel::svg_icon_default")] svg_icon: ResourceReference,
	#[serde(default = "RssFeedlyChannel::svg_logo_default")] svg_logo: ResourceReference,
	#[serde(default = "RssFeedlyChannel::accent_color_default")] accent_color: String, // eg 00FF00
	#[serde(default = "RssFeedlyChannel::related_default")] related: bool,
	#[serde(default = "RssFeedlyChannel::google_analytics_default")] google_analytics: Option<RssFeedlyChannelGoogleAnalyticsCode>,
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
	pub(crate) fn writeXml<'a, 'b: 'a, 'c, W: Write>(&'c self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[Attribute<'c>], primary_iso_639_1_alpha_2_language_code: &str, iso_639_1_alpha_2_language_code: &str, resources: &'a Resources, parentGoogleAnalyticsCode: Option<&str>) -> XmlWriterResult
	{
		if let Some(Some(url)) = self.png_cover_image.url(primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code), resources).ok()
		{
			let attributes =
			[
				Attribute::new(Name::local("image"), url.as_str()),
			];
			eventWriter.writeEmptyElement(namespace, &attributes, Name::prefixed("cover", "webfeeds"))?;
		}
		
		if let Some(Some(url)) = self.svg_icon.url(primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code), resources).ok()
		{
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, "webfeeds", "icon", url.as_str())?;
		}
		
		if let Some(Some(url)) = self.svg_logo.url(primary_iso_639_1_alpha_2_language_code, Some(iso_639_1_alpha_2_language_code), resources).ok()
		{
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, "webfeeds", "logo", url.as_str())?;
		}
		
		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, "webfeeds", "accentColor", &self.accent_color)?;
		
		let attributes =
		[
			Attribute::new(Name::local("layout"), "card"),
			Attribute::new(Name::local("target"), "browser"),
		];
		eventWriter.writeEmptyElement(namespace, &attributes, Name::prefixed("analytics", "webfeeds"))?;
		
		if let Some(ref googleAnalytics) = self.google_analytics
		{
			fn writeGoogleAnalyticsCode<W: Write>(eventWriter: &mut EventWriter<W>, namespace: &Namespace, code: &str) -> XmlWriterResult
			{
				let attributes =
				[
					Attribute::new(Name::local("id"), code),
					Attribute::new(Name::local("engine"), "GoogleAnalytics"),
				];
				eventWriter.writeEmptyElement(namespace, &attributes, Name::prefixed("analytics", "webfeeds"))
			}
			
			use self::RssFeedlyChannelGoogleAnalyticsCode::*;
			match *googleAnalytics
			{
				specific(ref code) => writeGoogleAnalyticsCode(eventWriter, namespace, code)?,
				inherit => match parentGoogleAnalyticsCode
				{
					Some(code) => writeGoogleAnalyticsCode(eventWriter, namespace, code)?,
					None => (),
				},
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn png_cover_image_default() -> ResourceReference
	{
		ResourceReference::internal("/cover.png".to_owned(), Some(UrlTag::largest_image))
	}
	
	#[inline(always)]
	fn svg_icon_default() -> ResourceReference
	{
		ResourceReference::internal("/favicon.svg".to_owned(), Some(UrlTag::default))
	}
	
	#[inline(always)]
	fn svg_logo_default() -> ResourceReference
	{
		ResourceReference::internal("/organization-logo.svg".to_owned(), Some(UrlTag::default))
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
	fn google_analytics_default() -> Option<RssFeedlyChannelGoogleAnalyticsCode>
	{
		Some(RssFeedlyChannelGoogleAnalyticsCode::inherit)
	}
}
