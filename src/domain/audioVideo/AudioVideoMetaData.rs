// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Default, Debug, Clone)]
pub(crate) struct AudioVideoMetaData
{
	// Used by amp-video, Google Video Site Map, ?twitter player card? (if we decide to)
	#[serde(default)] pub(crate) abstracts: HashMap<Iso639Dash1Alpha2Language, Rc<AudioVideoAbstract>>,
	
	// Used by amp-video, amp-audio
	#[serde(default)] pub(crate) artist: Option<String>,
	#[serde(default)] pub(crate) album: Option<String>,
	#[serde(default)] pub(crate) artwork: Option<ResourceUrl>,
	
	// Used by site map (and some by mRSS: https://developers.google.com/webmasters/videosearch/markups)
	#[serde(default)] pub(crate) site_map_category: Option<String>,
	#[serde(default)] pub(crate) site_map_tags: ArrayVec<[String; 32]>,
	#[serde(default)] pub(crate) expiration_date: Option<DateTime<Utc>>,
	#[serde(default)] pub(crate) publication_date: Option<DateTime<Utc>>,
	#[serde(default)] pub(crate) rating: Option<AudioVideoStarRating>,
	#[serde(default)] pub(crate) views: Option<u64>,
	#[serde(default)] pub(crate) site_map_explicit: bool,
	#[serde(default)] pub(crate) country_restrictions: AudioVideoCountryRestriction,
	#[serde(default)] pub(crate) platform_restrictions: AudioVideoPlatformRestriction,
	#[serde(default)] pub(crate) gallery: Option<ResourceUrl>,
	#[serde(default)] pub(crate) requires_subscription: bool,
	#[serde(default)] pub(crate) uploader: Option<Person>,
}

impl AudioVideoMetaData
{
	#[inline(always)]
	pub(crate) fn audioVideoAbstract(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&AudioVideoAbstract, CordialError>
	{
		if let Some(audioVideoAbstract) = self.abstracts.get(&iso639Dash1Alpha2Language)
		{
			Ok(audioVideoAbstract)
		}
		else if let Some(audioVideoAbstract) = self.abstracts.get(&fallbackIso639Dash1Alpha2Language)
		{
			Ok(audioVideoAbstract)
		}
		else
		{
			return Err(CordialError::Configuration("no AudioVideoAbstract for primary or fallback language".to_owned()))
		}
	}
	
	#[inline(always)]
	pub(crate) fn writeSiteMapXml<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, mediaUrl: &Url, iFrameUrl: &Url, durationInSeconds: Option<u64>) -> Result<(), CordialError>
	{
		self.audioVideoAbstract(fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?.writeSiteMapXml(eventWriter, namespace, emptyAttributes)?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "content_loc", mediaUrl.as_ref())?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "player_loc", iFrameUrl.as_ref())?;
		
		let live = if let Some(durationInSeconds) = durationInSeconds
		{
			const GoogleMaximumDurationOfEightHoursInSeconds: u64 = 28_800;
			if durationInSeconds > GoogleMaximumDurationOfEightHoursInSeconds
			{
				return Err(CordialError::Configuration("Audio or video in site maps can not exceed eight hours duration".to_owned()));
			}
			eventWriter.writePrefixedTextElementU64(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "duration", durationInSeconds)?;
			Self::BooleanNo
		}
		else
		{
			Self::BooleanYes
		};
		
		if let Some(expirationDate) = self.expiration_date
		{
			eventWriter.writePrefixedTextElementRfc3339(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "expiration_date", expirationDate)?;
		}
		
		if let Some(videoStarRating) = self.rating
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "rating", &videoStarRating.toGoogleSiteMapString())?;
		}
		
		if let Some(viewCount) = self.views
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "view_count", &format!("{}", viewCount))?;
		}
		
		if let Some(publicationDate) = self.publication_date
		{
			eventWriter.writePrefixedTextElementRfc3339(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "publication_date", publicationDate)?;
		}
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "family_friendly", Self::booleanYesOrNo(self.site_map_explicit))?;
		
		self.writeXmlForCanonicalizedTagString(eventWriter, namespace, emptyAttributes)?;
		
		self.writeXmlForCategory(eventWriter, namespace, emptyAttributes)?;
		
		self.country_restrictions.writeXmlForRestriction(eventWriter, namespace)?;
		
		if let Some(ref gallery) = self.gallery
		{
			let (url, title) = ResourceReference
			{
				resource: gallery.clone(),
				tag: ResourceTag::default,
			}.urlAndAnchorTitleAttribute(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
			
			eventWriter.writePrefixedTextElement(namespace, &[ "title".xml_str_attribute(title.as_str()) ], SiteMapWebPageAudioVideo::VideoNamespacePrefix, "gallery_loc", url.as_str())?;
		}
		
		// Unimplemented: video:price (can be supported with a BTreeSet of (currency, type, resolution) tuples
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "requires_subscription", Self::booleanYesOrNo(self.requires_subscription))?;
		
		#[inline(always)]
		fn writeUploader<'a, W: Write>(eventWriter: &mut EventWriter<W>, namespace: &Namespace, uploaderName: &FullName, attributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
		{
			eventWriter.writePrefixedTextElement(namespace, &attributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "uploader", uploaderName)
		}
		
		if let Some(ref uploader) = self.uploader
		{
			let uploaderName = &uploader.full_name;
			
			if let Some(ref resourceUrl) = uploader.url
			{
				let url = ResourceReference
				{
					resource: resourceUrl.clone(),
					tag: ResourceTag::default,
				}.urlMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
				
				writeUploader(eventWriter, namespace, uploaderName, &["info".xml_url_attribute(&url)])?;
			}
			else
			{
				writeUploader(eventWriter, namespace, uploaderName, &emptyAttributes)?;
			}
		}
		
		self.platform_restrictions.writeXmlForRestriction(eventWriter, namespace)?;
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "live", live)
	}
	
	const BooleanYes: &'static str = "yes";
	
	const BooleanNo: &'static str = "no";
	
	#[inline(always)]
	fn booleanYesOrNo(boolean: bool) -> &'static str
	{
		if boolean
		{
			Self::BooleanYes
		}
		else
		{
			Self::BooleanNo
		}
	}
	
	#[inline(always)]
	fn writeXmlForCanonicalizedTagString<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		let mut canonicalizedOrderedTags = OrderMap::with_capacity(self.site_map_tags.len());
		for toBeCanonicalizedTag in self.site_map_tags.iter()
		{
			let lowerCased = toBeCanonicalizedTag.to_lowercase();
			if !canonicalizedOrderedTags.contains_key(&lowerCased)
			{
				canonicalizedOrderedTags.insert(lowerCased, ());
			}
		}
		for canonicalizedSortedTag in canonicalizedOrderedTags.keys()
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "tag", canonicalizedSortedTag)?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn writeXmlForCategory<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		if let Some(ref category) = self.site_map_category
		{
			if category.chars().count() > 256
			{
				return Err(CordialError::Configuration("Audio / Video site map category can not exceed 256 characters".to_owned()));
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, SiteMapWebPageAudioVideo::VideoNamespacePrefix, "category", category)
		}
		else
		{
			Ok(())
		}
	}
}
