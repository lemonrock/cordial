// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FeedlyRssChannel
{
	#[serde(default = "FeedlyRssChannel::png_cover_image_default")] png_cover_image: ResourceUrl,
	#[serde(default = "FeedlyRssChannel::svg_icon_default")] svg_icon: ResourceUrl,
	#[serde(default = "FeedlyRssChannel::svg_logo_default")] svg_logo: ResourceUrl,
	#[serde(default)] accent_color: HexadecimalColor,
	#[serde(default = "FeedlyRssChannel::related_default")] related: bool,
	#[serde(default = "FeedlyRssChannel::google_analytics_default")] google_analytics: Option<FeedlyRssChannelGoogleAnalyticsCode>,
}

impl Default for FeedlyRssChannel
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			png_cover_image: Self::png_cover_image_default(),
			svg_icon: Self::svg_icon_default(),
			svg_logo: Self::svg_logo_default(),
			accent_color: Default::default(),
			related: Self::related_default(),
			google_analytics: Self::google_analytics_default(),
		}
	}
}

impl FeedlyRssChannel
{
	pub(crate) const WebfeedsNamespacePrefix: &'static str = "webfeeds";
	
	pub(crate) const WebfeedsNamespaceUrl: &'static str = "http://webfeeds.org/rss/1.0";
	
	#[inline(always)]
	pub(crate) fn writeXml<'a, 'b: 'a, 'c, W: Write>(&'c self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'c>], fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, resources: &'a Resources, parentGoogleAnalyticsCode: Option<&str>) -> Result<(), CordialError>
	{
		let iso639Dash1Alpha2Language = Some(iso639Dash1Alpha2Language);

		{
			let urlData = ResourceReference
			{
				resource: self.png_cover_image.clone(),
				tag: ResourceTag::largest_image,
			}.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			urlData.validateIsPng()?;
			eventWriter.writeEmptyElement(namespace, &[ "image".xml_url_from_UrlData_attribute(&urlData) ], Self::WebfeedsNamespacePrefix.prefixes_xml_name("cover"))?;
		}

		{
			let urlData = ResourceReference
			{
				resource: self.svg_icon.clone(),
				tag: ResourceTag::default,
			}.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			urlData.validateIsSvg()?;
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::WebfeedsNamespacePrefix, "icon", urlData.url_str())?;
		}

		{
			let urlData = ResourceReference
			{
				resource: self.svg_logo.clone(),
				tag: ResourceTag::default,
			}.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			urlData.validateIsSvg()?;
			eventWriter.writePrefixedTextElement(namespace, &emptyAttributes, Self::WebfeedsNamespacePrefix, "logo", urlData.url_str())?;
		}
		
		eventWriter.writePrefixedTextElementString(namespace, &emptyAttributes, Self::WebfeedsNamespacePrefix, "accentColor", self.accent_color.toStringWithoutHashPrefix())?;

		let attributes =
		[
			"layout".xml_str_attribute("card"),
			"target".xml_str_attribute("browser"),
		];
		eventWriter.writeEmptyElement(namespace, &attributes, Self::WebfeedsNamespacePrefix.prefixes_xml_name("analytics"))?;

		if let Some(ref googleAnalytics) = self.google_analytics
		{
			fn writeGoogleAnalyticsCode<W: Write>(eventWriter: &mut EventWriter<W>, namespace: &Namespace, code: &str) -> Result<(), CordialError>
			{
				let attributes =
				[
					"id".xml_str_attribute(code),
					"engine".xml_str_attribute("GoogleAnalytics"),
				];
				eventWriter.writeEmptyElement(namespace, &attributes, FeedlyRssChannel::WebfeedsNamespacePrefix.prefixes_xml_name("analytics"))
			}

			use self::FeedlyRssChannelGoogleAnalyticsCode::*;
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
	fn png_cover_image_default() -> ResourceUrl
	{
		ResourceUrl::string("/cover.png")
	}

	#[inline(always)]
	fn svg_icon_default() -> ResourceUrl
	{
		ResourceUrl::string("/favicon.svg")
	}

	#[inline(always)]
	fn svg_logo_default() -> ResourceUrl
	{
		ResourceUrl::string("/organization-logo.svg")
	}
	
	#[inline(always)]
	fn related_default() -> bool
	{
		true
	}

	#[inline(always)]
	fn google_analytics_default() -> Option<FeedlyRssChannelGoogleAnalyticsCode>
	{
		Some(FeedlyRssChannelGoogleAnalyticsCode::inherit)
	}
}
