// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct SiteMapWebPageAudioVideo
{
	pub(crate) placeHolderUrl: ResourceUrl,
	pub(crate) durationInSeconds: Option<u64>,
	pub(crate) mediaUrl: Url,
	pub(crate) iFrameUrl: Url,
	
	pub(crate) audioVideoMetaData: Rc<AudioVideoMetaData>,
}

impl SiteMapWebPageAudioVideo
{
	pub(crate) const VideoNamespacePrefix: &'static str = "video";
	
	pub(crate) const VideoNamespaceUrl: &'static str = "http://www.google.com/schemas/sitemap-video/1.1";
	
	#[inline(always)]
	pub(crate) fn writeXml<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		eventWriter.writeWithinElement(Self::VideoNamespacePrefix.prefixes_xml_name("video"), namespace, emptyAttributes, |eventWriter|
		{
			// TODO: Unify with artwork
			let thumbnailResource = self.placeHolderUrl.resourceMandatory(resources)?;
			let thumbnailUrlData = thumbnailResource.findGoogleVideoSiteMapImageThumbnail(fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::VideoNamespacePrefix, "thumbnail_loc", thumbnailUrlData.url_str())?;
			
			self.audioVideoMetaData.writeSiteMapXml(eventWriter, namespace, emptyAttributes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, &self.mediaUrl, &self.iFrameUrl, self.durationInSeconds)
		})
	}
}
