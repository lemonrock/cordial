// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AudioVideoAbstract
{
	pub(crate) title: String,
	pub(crate) site_map_description: String,
}

impl AudioVideoAbstract
{
	#[inline(always)]
	pub(crate) fn writeXmlForSiteMapTitle<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		eventWriter.writeCDataElement(namespace, emptyAttributes, SiteMapWebPageVideo::VideoNamespacePrefix.prefixes_xml_name("title"), &self.title)
	}
	
	#[inline(always)]
	pub(crate) fn writeXmlForSiteMapDescription<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		if self.site_map_description.chars().count() > 2048
		{
			return Err(CordialError::Configuration("Video site map description can not exceed 2048 characters".to_owned()));
		}
		
		eventWriter.writeCDataElement(namespace, emptyAttributes, SiteMapWebPageVideo::VideoNamespacePrefix.prefixes_xml_name("description"), &self.site_map_description)
	}
}
