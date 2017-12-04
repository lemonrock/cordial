// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum TwitterCardType
{
	summary,
	summary_large_image
	{
		// AKA author
		#[serde(default)] creator: Option<TwitterAtHandle>,
	},
	app
	{
		#[serde(default)] iphone: Option<TwitterCardAppReference>,
		#[serde(default)] ipad: Option<TwitterCardAppReference>,
		#[serde(default)] googleplay: Option<TwitterCardAppReference>,
	},
	
	// iframe rules:-
	// HTML page must be responsive, filling the height and width of display area
	// Default to ‘sound off’ for videos that automatically play content
	// Content greater than 10 seconds in length must not automatically play
	// Include stop, pause and play controls
//	player
//	{
//		iframe: ResourceUrl, // twitter:player
//		iframe_width: u32, // twitter:player:width
//		iframe_height: u32, // twitter:player:height
//		placeholder: ResourceUrl, // twitter:image, twitter:image:alt  placeholder image for audioVideo or animation. Images with fewer than 68,600 pixels (a 262x262 square image, or a 350x196 16:9 image) will cause the player card not to render. Images must be less than 5MB in size. JPG, PNG, WEBP and GIF formats are supported.
//		mp4_stream: ResourceUrl, // twitter:player:stream  Codecs supported:  Video: H.264, Baseline Profile (BP), Level 3.0, up to 640 x 480 at 30 fps.  Audio: AAC, Low Complexity Profile (LC)
//		mp4_stream_content_type: String, // twitter:player:stream:content_type  One of application/mp4 audioVideo/mp4 audio/mp4 BUT with additional codec information: https://tools.ietf.org/html/rfc6381#section-3.6
//
//		/*
//
//		Example of codecs:
//			<meta name="twitter:player:stream:content_type" content="audioVideo/mp4; codecs=&quot;avc1.42E01E1, mp4a.40.2&quot;" />
//			<meta name="twitter:player:stream:content_type" content="audioVideo/mp4; codecs=&quot;avc1.42E01E1, mp4a.40.2&quot;">
//
//		JWPlayer IFrame sample: https://www.jwplayer.com/blog/supporting-twitter-cards/
//
//		Basic HTML5 audioVideo:-
//			<audioVideo src="audioVideo.webm" controls></audioVideo>
//
//		Fallback HTML5 audioVideo:-
//			<audioVideo controls>
//  				<source src="devstories.webm" type='audioVideo/webm;codecs="vp8, vorbis"'/>
//  				<source src="devstories.mp4" type='audioVideo/mp4;codecs="avc1.42E01E, mp4a.40.2"'/>
//			</audioVideo>
//
//		Start time and end time in seconds (Accept-Ranges required):-
//			<source src="devstories.webm#t=10,20" type='audioVideo/webm;codecs="vp8, vorbis"' />
//
//		In hours, minutes and seconds, play only one minute: #t=,00:01:00
//
//		With a track element to caption in WebVTT format and a poster (plaecholder) image
//		<audioVideo controls style="width:640px;height:360px;" poster="poster.png">
//  <source src="devstories.webm"
//          type='audioVideo/webm;codecs="vp8, vorbis"' />
//  <source src="devstories.mp4"
//          type='audioVideo/mp4;codecs="avc1.42E01E, mp4a.40.2"' />
//  <track src="devstories-en.vtt" label="English subtitles"
//         kind="subtitles" srclang="en" default></track>
//</audioVideo>
//
//
//
//
//		IFrame sample:-
//
//<!DOCTYPE html>
//<html>
//	<head>
//
//<style type="text/css">
//audioVideo
//{
//   width:100%;
//   max-width:600px;
//   height:auto;
//}
//</style>
//	</head>
//	<body>
//		<audioVideo width="100%" controls>
//  			<source src="example.mp4" type="audioVideo/mp4">
//			Your browser does not support audioVideo
//		</audioVideo>
//	</body>
//</html>
//
//		*/
//	},
}

impl Default for TwitterCardType
{
	#[inline(always)]
	fn default() -> Self
	{
		TwitterCardType::summary
	}
}

impl TwitterCardType
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>, site: &Option<TwitterAtHandle>, articleImage: &Option<(ResourceUrl, Rc<ImageMetaData>)>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, languageData: &LanguageData) -> Result<(), CordialError>
	{
		fn validateTwitterAtHandle<'a>(twitterAtHandle: &'a String, name: &str) -> Result<&'a str, CordialError>
		{
			if !twitterAtHandle.starts_with('@') || twitterAtHandle.len() < 2
			{
				return Err(CordialError::Configuration(format!("twitter card '{}' does not start @ and is not at least 2 characters long", name)));
			}
			Ok(twitterAtHandle.as_str())
		}
		
		fn addType(endHeadNodes: &mut Vec<UnattachedNode>, type_: &str)
		{
			endHeadNodes.push(meta_with_property_and_content("twitter:card", type_));
		}
		
		fn addTwitterHandle(endHeadNodes: &mut Vec<UnattachedNode>, handle: &Option<TwitterAtHandle>, name: &str) -> Result<(), CordialError>
		{
			if let &Some(ref site) = handle
			{
				endHeadNodes.push(meta_with_name_and_content(&format!("twitter:{}", name), validateTwitterAtHandle(site, name)?))
			}
			Ok(())
		}
		
		fn addImage(endHeadNodes: &mut Vec<UnattachedNode>, articleImage: &Option<(ResourceUrl, Rc<ImageMetaData>)>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, twitterCardImageMatch: &TwitterCardImageMatch) -> Result<(), CordialError>
		{
			if let &Some((ref articleImageResourceUrl, ref articleImageMetaData)) = articleImage
			{
				let resource = articleImageResourceUrl.resourceMandatory(resources)?;
				let urlData = resource.findUrlForTwitterCardImage(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language), twitterCardImageMatch)?;
				endHeadNodes.push(meta_with_property_and_content("twitter:image", urlData.url_str()));
				
				let altText = articleImageMetaData.alt(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
				// twitter is not clear by 'characters' whether they mean bytes or unicode code points or graphemes; we use the lowest possible value.
				if altText.chars().count() > 420
				{
					return Err(CordialError::Configuration("Twitter restricts image alt text to 420 characters".to_owned()));
				}
				endHeadNodes.push(meta_with_property_and_content("twitter:image:alt", altText));
			}
			
			Ok(())
		}
		
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		use self::TwitterCardType::*;
		
		match *self
		{
			summary =>
			{
				addType(endHeadNodes, "summary");
				
				addTwitterHandle(endHeadNodes, site, "site")?;
				
				addImage(endHeadNodes, articleImage, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &TwitterCardImageMatch::Summary)?;
			}
			
			summary_large_image { ref creator } =>
			{
				addType(endHeadNodes, "summary_large_image");
				
				addTwitterHandle(endHeadNodes, site, "site")?;
				
				addTwitterHandle(endHeadNodes, creator, "creator")?;
				
				addImage(endHeadNodes, articleImage, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &TwitterCardImageMatch::SummaryLargeImage)?;
			}
			
			app { ref iphone, ref ipad, ref googleplay } =>
			{
				addType(endHeadNodes, "app");
				
				addTwitterHandle(endHeadNodes, site, "site")?;
				
				let appStoreCountryCode = languageData.language.iso3166Dash1Alpha2CountryCode();
				if appStoreCountryCode != Iso3166Dash1Alpha2CountryCode::default()
				{
					endHeadNodes.push(meta_with_name_and_content("twitter:app:country", appStoreCountryCode.to_iso_3166_1_alpha_2_language_code()));
				}
				
				TwitterCardAppReference::addToIf(iphone, "iphone", endHeadNodes);
				if TwitterCardAppReference::addToIf(ipad, "ipad", endHeadNodes)
				{
					// Add the iphone app as the ipad one if no specialized ipad add
					TwitterCardAppReference::addToIf(iphone, "ipad", endHeadNodes);
				}
				TwitterCardAppReference::addToIf(googleplay, "googleplay", endHeadNodes);
			}
			
//			player { }
//			{
//			}
		}
		
		Ok(())
	}
}
