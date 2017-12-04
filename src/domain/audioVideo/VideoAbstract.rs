// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct VideoAbstract
{
	pub(crate) title: String,
	pub(crate) site_map_description: String,
	pub(crate) site_map_category: String,
	pub(crate) site_map_tags: ArrayVec<[String; 32]>,
}

impl VideoAbstract
{
	#[inline(always)]
	pub(crate) fn writeXmlForCanonicalizedTagString<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		// tags; canonicalized and sorted
		let mut canonicalizedSortedTags = BTreeSet::new();
		for toBeCanonicalizedTag in self.site_map_tags.iter()
		{
			let lowerCased = toBeCanonicalizedTag.to_lowercase();
			canonicalizedSortedTags.insert(lowerCased);
		}
		for canonicalizedSortedTag in canonicalizedSortedTags.iter()
		{
			eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "tag", canonicalizedSortedTag)?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn writeXmlForCategory<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'a>]) -> Result<(), CordialError>
	{
		if self.site_map_category.len() > 256
		{
			return Err(CordialError::Configuration("Video site map category can not exceed 256 characters".to_owned()));
		}
		
		eventWriter.writePrefixedTextElement(namespace, emptyAttributes, "video", "category", &self.site_map_category)
	}
}
