// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct SiteMapWebPageImage
{
	pub(crate) url: ResourceReference,
	pub(crate) licenseUrl: ResourceReference,
	pub(crate) imageAbstract: Rc<ImageAbstract>,
}

impl SiteMapWebPageImage
{
	pub(crate) const ImageNamespacePrefix: &'static str = "image";
	
	pub(crate) const ImageNamespaceUrl: &'static str = "http://www.google.com/schemas/sitemap-image/1.1";
	
	#[inline(always)]
	fn writeXml<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>], resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>) -> Result<(), CordialError>
	{
		eventWriter.writeWithinElement(Self::ImageNamespacePrefix.prefixes_xml_name("image"), namespace, emptyAttributes, |eventWriter|
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ImageNamespacePrefix, "loc", self.url.urlMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?.as_str())?;
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ImageNamespacePrefix, "caption", &self.imageAbstract.caption)?;
			if let Some(geographicLocation) = self.imageAbstract.geographic_location.as_ref()
			{
				eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ImageNamespacePrefix, "geo_location", geographicLocation)?;
			}
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ImageNamespacePrefix, "title", &self.imageAbstract.title)?;
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, Self::ImageNamespacePrefix, "license_url", self.licenseUrl.urlMandatory(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?.as_str())
		})
	}
}
