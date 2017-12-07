// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct SiteMapWebPageVideo
{
	pub(crate) placeHolderUrl: ResourceUrl,
	pub(crate) videoAbstract: Rc<VideoAbstract>,

	pub(crate) mp4Url: Url,
	pub(crate) iFrameUrl: Url,
	
	pub(crate) category: Option<Rc<String>>,
	pub(crate) tags: Rc<ArrayVec<[String; 32]>>,
	pub(crate) durationInSeconds: Option<u64>,
	pub(crate) expirationDate: Option<DateTime<Utc>>,
	pub(crate) videoStarRating: Option<VideoStarRating>,
	pub(crate) viewCount: Option<u64>,
	pub(crate) publicationDate: Option<DateTime<Utc>>,
	pub(crate) explicit: bool,
	pub(crate) countryRestrictions: Rc<VideoCountryRestriction>,
	pub(crate) gallery: Option<ResourceUrl>,
	pub(crate) requiresSubscription: bool,
	pub(crate) uploader: Option<Rc<Person>>,
	pub(crate) platformRestrictions: Rc<VideoPlatformRestriction>,
}

impl SiteMapWebPageVideo
{
	pub(crate) const VideoNamespacePrefix: &'static str = "video";
	
	pub(crate) const VideoNamespaceUrl: &'static str = "http://www.google.com/schemas/sitemap-video/1.1";
	
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		const GoogleMaximumDurationOfEightHoursInSeconds: u64 = 28_800;
		
		let resource = self.placeHolderUrl.resourceMandatory(resources)?;
		let thumbnailUrlData = resource.findGoogleVideoSiteMapImageThumbnail(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
		
		eventWriter.writeWithinElement(Self::VideoNamespacePrefix.prefixes_xml_name("video"), namespace, emptyAttributes, |eventWriter|
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "thumbnail_loc", thumbnailUrlData.url_str())?;
			
			self.videoAbstract.writeXmlForSiteMapTitle(eventWriter, namespace, emptyAttributes)?;
			
			self.videoAbstract.writeXmlForSiteMapDescription(eventWriter, namespace, emptyAttributes)?;
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "content_loc", self.mp4Url.as_ref())?;
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "player_loc", self.iFrameUrl.as_ref())?;
			
			let live = if let Some(durationInSeconds) = self.durationInSeconds
			{
				if durationInSeconds > GoogleMaximumDurationOfEightHoursInSeconds
				{
					return Err(CordialError::Configuration("videos in site maps can not exceed eight hours duration".to_owned()));
				}
				eventWriter.writePrefixedTextElementU64(namespace, emptyAttributes, Self::VideoNamespacePrefix, "duration", durationInSeconds)?;
				Self::BooleanNo
			}
			else
			{
				Self::BooleanYes
			};
			
			if let Some(expirationDate) = self.expirationDate
			{
				eventWriter.writePrefixedTextElementRfc3339(namespace, emptyAttributes, Self::VideoNamespacePrefix, "expiration_date", expirationDate)?;
			}
			
			if let Some(videoStarRating) = self.videoStarRating
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "rating", &videoStarRating.toGoogleSiteMapString())?;
			}
			
			if let Some(viewCount) = self.viewCount
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "view_count", &format!("{}", viewCount))?;
			}
			
			if let Some(publicationDate) = self.publicationDate
			{
				eventWriter.writePrefixedTextElementRfc3339(namespace, emptyAttributes, Self::VideoNamespacePrefix, "publication_date", publicationDate)?;
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "family_friendly", Self::booleanYesOrNo(self.explicit))?;
			
			self.writeXmlForCanonicalizedTagString(eventWriter, namespace, emptyAttributes)?;
			
			self.writeXmlForCategory(eventWriter, namespace, emptyAttributes)?;
			
			self.countryRestrictions.writeXmlForRestriction(eventWriter, namespace)?;
			
			if let Some(ref gallery) = self.gallery
			{
				let (url, title) = ResourceReference
				{
					resource: gallery.clone(),
					tag: ResourceTag::default,
				}.urlAndAnchorTitleAttribute(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
				
				eventWriter.writePrefixedTextElement(namespace, &[ "title".xml_str_attribute(title.as_str()) ], Self::VideoNamespacePrefix, "gallery_loc", url.as_str())?;
			}
			
			// Unimplemented: video:price (can be supported with a BTreeSet of (currency, type, resolution) tuples
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "requires_subscription", Self::booleanYesOrNo(self.requiresSubscription))?;
			
			#[inline(always)]
			fn writeUploader<'a, W: Write>(eventWriter: &mut EventWriter<W>, namespace: &Namespace, uploaderName: &FullName, attributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
			{
				eventWriter.writePrefixedTextElement(namespace, &attributes, SiteMapWebPageVideo::VideoNamespacePrefix, "uploader", uploaderName)
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
			
			self.platformRestrictions.writeXmlForRestriction(eventWriter, namespace)?;
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "live", live)
		})
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
		let mut canonicalizedOrderedTags = OrderMap::with_capacity(self.tags.len());
		for toBeCanonicalizedTag in self.tags.iter()
		{
			let lowerCased = toBeCanonicalizedTag.to_lowercase();
			if !canonicalizedOrderedTags.contains_key(&lowerCased)
			{
				canonicalizedOrderedTags.insert(lowerCased, ());
			}
		}
		for canonicalizedSortedTag in canonicalizedOrderedTags.keys()
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "tag", canonicalizedSortedTag)?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn writeXmlForCategory<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		if let Some(ref category) = self.category
		{
			if category.chars().count() > 256
			{
				return Err(CordialError::Configuration("Video site map category can not exceed 256 characters".to_owned()));
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "category", category)
		}
		else
		{
			Ok(())
		}
	}
}
