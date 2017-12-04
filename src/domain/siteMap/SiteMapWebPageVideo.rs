// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct SiteMapWebPageVideo
{
	// TODO: Images must be at least 160x90 pixels and at most 1920x1080 pixels
	// TODO: Check twitter player card rules, too
	pub(crate) placeHolderUrl: Url,
	pub(crate) videoAbstract: Rc<VideoAbstract>, // title + description

	pub(crate) mp4StreamUrl: Url,
	pub(crate) iFrameUrl: Url,
	pub(crate) durationInSeconds: Option<u32>, // omit this for a live stream
	pub(crate) expirationDate: Option<DateTime<Utc>>,
	pub(crate) videoStarRating: VideoStarRating,
	pub(crate) viewCount: Option<u64>,
	pub(crate) publicationDate: Option<DateTime<Utc>>,
	pub(crate) canAppearInSafeSearch: bool,
	pub(crate) countryRestrictions: Rc<VideoCountryRestriction>,
	pub(crate) gallery: Option<ResourceUrl>,
	pub(crate) requires_subscription: bool,
	pub(crate) uploader: Option<Rc<Person>>,
	pub(crate) platformRestrictions: Rc<VideoPlatformRestriction>,
}

impl SiteMapWebPageVideo
{
	#[inline(always)]
	fn writeXml<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		const GoogleMaximumDurationOfEightHoursInSeconds: u32 = 28_800;
		
		eventWriter.writeWithinElement(Name::prefixed("video", "video"), namespace, emptyAttributes, |eventWriter|
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "thumbnail_loc", self.placeHolderUrl.as_str())?;
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "title", &self.videoAbstract.title)?;
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "description", &self.videoAbstract.site_map_description)?;
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "content_loc", self.mp4StreamUrl.as_ref())?;
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "player_loc", self.iFrameUrl.as_ref())?;
			
			let live = if let Some(durationInSeconds) = self.durationInSeconds
			{
				if durationInSeconds > GoogleMaximumDurationOfEightHoursInSeconds
				{
					return Err(CordialError::Configuration("videos in site maps can not exceed eight hours duration".to_owned()));
				}
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "duration", &format!("{}", durationInSeconds))?;
				"no"
			}
			else
			{
				"yes"
			};
			
			if let Some(expirationDate) = self.expirationDate
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "expiration_date", &expirationDate.to_rfc3339())?;
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "rating", &self.videoStarRating.toGoogleSiteMapString())?;
			
			if let Some(viewCount) = self.viewCount
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "view_count", &format!("{}", viewCount))?;
			}
			
			if let Some(publicationDate) = self.publicationDate
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "publication_date", &publicationDate.to_rfc3339())?;
			}
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "family_friendly", if self.canAppearInSafeSearch { "yes" } else { "no" })?;
			
			self.videoAbstract.writeXmlForCanonicalizedTagString(eventWriter, namespace, emptyAttributes)?;
			
			self.videoAbstract.writeXmlForCategory(eventWriter, namespace, emptyAttributes)?;
			
			self.countryRestrictions.writeXmlForRestriction(eventWriter, namespace)?;
			
			if let Some(ref gallery) = self.gallery
			{
				let (url, title) = ResourceReference
				{
					resource: gallery.clone(),
					tag: ResourceTag::default,
				}.urlAndAnchorTitleAttribute(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
				
				let attributes =
				[
					XmlAttribute::new(Name::local("title"), title.as_str())
				];
				
				eventWriter.writePrefixedTextElement(namespace, &attributes, "video", "gallery_loc", url.as_str())?;
			}
			
			// Unimplemented: video:price (can be supported with a BTreeSet of (currency, type, resolution) tuples
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "requires_subscription", if self.requires_subscription { "yes" } else { "no" })?;
			
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
					
					let attributes =
					[
						XmlAttribute::new(Name::local("into"), url.as_str())
					];
					
					eventWriter.writePrefixedTextElement(namespace, &attributes, "video", "uploader", uploaderName)?;
				}
				else
				{
					eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "uploader", uploaderName)?;
				}
			}
			
			self.platformRestrictions.writeXmlForRestriction(eventWriter, namespace)?;
			
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "live", live)
		})
	}
}
