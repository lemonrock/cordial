// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
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
	
	audio_player,
	
	video_player,
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
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>, site: &Option<TwitterAtHandle>, articleImage: &Option<(ResourceUrl, Rc<ImageMetaData>)>, articleAudio: Option<&ResourceUrl>, articleVideo: Option<&ResourceUrl>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, languageData: &LanguageData) -> Result<(), CordialError>
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
			
			audio_player => match articleAudio
			{
				None => return Err(CordialError::Configuration("Twitter player cards require at least one associated audio".to_owned())),
				Some(articleAudio) =>
				{
					let articleAudioResource = articleAudio.resourceMandatory(resources)?;
					let audioPipeline = articleAudioResource.audioPipeline()?;
					
					addType(endHeadNodes, "player");
					
					addTwitterHandle(endHeadNodes, site, "site")?;
					
					endHeadNodes.push(meta_with_name_and_content("twitter:player", audioPipeline.iFramePlayerUrl(articleAudio, languageData)?.as_str()));
					
					let (width, height) = audioPipeline.dimensions();
					endHeadNodes.push(meta_with_name_and_content("twitter:player:width", &format!("{}", width)));
					endHeadNodes.push(meta_with_name_and_content("twitter:player:height", &format!("{}", height)));
					
					let placeHolderResource = audioPipeline.metadata.placeholder.resourceMandatory(resources)?;
					let placeHolderUrlData = placeHolderResource.findUrlDataForTwitterCardPlayerPlaceHolderImage(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
					
					endHeadNodes.push(meta_with_property_and_content("twitter:image", placeHolderUrlData.url_str()));
					let placeHolderImageMetaData = placeHolderResource.imageMetaData()?;
					let altText = placeHolderImageMetaData.alt(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
					if altText.chars().count() > 420
					{
						return Err(CordialError::Configuration("Twitter restricts image alt text to 420 characters".to_owned()));
					}
					endHeadNodes.push(meta_with_property_and_content("twitter:image:alt", altText));
					
					let mp4UrlBorrow = audioPipeline.mp4Url.borrow();
					endHeadNodes.push(meta_with_name_and_content("twitter:player:stream", &format!("{}", mp4UrlBorrow.as_ref().unwrap().as_str())));
					
					endHeadNodes.push(meta_with_name_and_content("twitter:player:stream:content_type", audioPipeline.twitterContentType().0.as_ref()));
				}
			}
			
			video_player => match articleVideo
			{
				None => return Err(CordialError::Configuration("Twitter player cards require at least one associated video".to_owned())),
				Some(articleVideo) =>
				{
					let articleVideoResource = articleVideo.resourceMandatory(resources)?;
					let videoPipeline = articleVideoResource.videoPipeline()?;
					
					addType(endHeadNodes, "player");
					
					addTwitterHandle(endHeadNodes, site, "site")?;
					
					endHeadNodes.push(meta_with_name_and_content("twitter:player", videoPipeline.iFramePlayerUrl(articleVideo, languageData)?.as_str()));
					
					let (width, height) = videoPipeline.dimensions();
					endHeadNodes.push(meta_with_name_and_content("twitter:player:width", &format!("{}", width)));
					endHeadNodes.push(meta_with_name_and_content("twitter:player:height", &format!("{}", height)));
					
					let placeHolderResource = videoPipeline.metadata.placeholder.resourceMandatory(resources)?;
					let placeHolderUrlData = placeHolderResource.findUrlDataForTwitterCardPlayerPlaceHolderImage(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
					
					endHeadNodes.push(meta_with_property_and_content("twitter:image", placeHolderUrlData.url_str()));
					let placeHolderImageMetaData = placeHolderResource.imageMetaData()?;
					let altText = placeHolderImageMetaData.alt(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
					if altText.chars().count() > 420
					{
						return Err(CordialError::Configuration("Twitter restricts image alt text to 420 characters".to_owned()));
					}
					endHeadNodes.push(meta_with_property_and_content("twitter:image:alt", altText));
					
					let mp4UrlBorrow = videoPipeline.mp4Url.borrow();
					endHeadNodes.push(meta_with_name_and_content("twitter:player:stream", &format!("{}", mp4UrlBorrow.as_ref().unwrap().as_str())));
					
					endHeadNodes.push(meta_with_name_and_content("twitter:player:stream:content_type", videoPipeline.twitterContentType().0.as_ref()));
				}
			}
		}
		
		Ok(())
	}
}
