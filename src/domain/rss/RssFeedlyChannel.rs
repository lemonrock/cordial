// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct RssFeedlyChannel
{
	#[serde(default = "RssFeedlyChannel::png_cover_image_default")] png_cover_image: ResourceReference,
	#[serde(default = "RssFeedlyChannel::svg_icon_default")] svg_icon: ResourceReference,
	#[serde(default = "RssFeedlyChannel::svg_logo_default")] svg_logo: ResourceReference,
	#[serde(default = "RssFeedlyChannel::accent_color_default")] accent_color: [u8; 3], // eg 00FF00, R, G, B
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
	pub(crate) fn writeXml<'a, 'b: 'a, 'c, W: Write>(&'c self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'c>], primaryIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, resources: &'a Resources, parentGoogleAnalyticsCode: Option<&str>) -> Result<(), CordialError>
	{
		let iso639Dash1Alpha2Language = Some(iso639Dash1Alpha2Language);
		
		{
			let url = self.png_cover_image.urlMandatory(resources, primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			let attributes =
			[
				XmlAttribute::new(Name::local("image"), url.as_str()),
			];
			eventWriter.writeEmptyElement(namespace, &attributes, Name::prefixed("cover", "webfeeds"))?;
		}
		
		{
			let url = self.svg_icon.urlMandatory(resources, primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, "webfeeds", "icon", url.as_str())?;
		}
		
		{
			let url = self.svg_logo.urlMandatory(resources, primaryIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, "webfeeds", "logo", url.as_str())?;
		}
		
		let accentColor = format!("{:02X}{:02X}{:02X}", self.accent_color[0], self.accent_color[1], self.accent_color[2]);
		
		eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, "webfeeds", "accentColor", &accentColor)?;
		
		let attributes =
		[
			XmlAttribute::new(Name::local("layout"), "card"),
			XmlAttribute::new(Name::local("target"), "browser"),
		];
		eventWriter.writeEmptyElement(namespace, &attributes, Name::prefixed("analytics", "webfeeds"))?;
		
		if let Some(ref googleAnalytics) = self.google_analytics
		{
			fn writeGoogleAnalyticsCode<W: Write>(eventWriter: &mut EventWriter<W>, namespace: &Namespace, code: &str) -> Result<(), CordialError>
			{
				let attributes =
				[
					XmlAttribute::new(Name::local("id"), code),
					XmlAttribute::new(Name::local("engine"), "GoogleAnalytics"),
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
		ResourceReference::new("/cover.png", ResourceTag::largest_image)
	}
	
	#[inline(always)]
	fn svg_icon_default() -> ResourceReference
	{
		ResourceReference::new("/favicon.svg", ResourceTag::default)
	}
	
	#[inline(always)]
	fn svg_logo_default() -> ResourceReference
	{
		ResourceReference::new("/organization-logo.svg", ResourceTag::default)
	}
	
	#[inline(always)]
	fn accent_color_default() -> [u8; 3]
	{
		[0x00, 0xFF, 0x00]
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
