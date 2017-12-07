// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AudioVideoPlatformRestriction
{
	#[serde(default)] pub(crate) restriction: CountryRestrictionInclusion,
	#[serde(default)] pub(crate) platforms: BTreeSet<AudioVideoPlatform>,
}

impl Default for AudioVideoPlatformRestriction
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			restriction: Default::default(),
			platforms: Default::default(),
		}
	}
}

impl AudioVideoPlatformRestriction
{
	#[inline(always)]
	pub(crate) fn writeXmlForRestriction<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace) -> Result<(), CordialError>
	{
		if self.platforms.is_empty()
		{
			return Ok(());
		}
		
		let mut afterFirst = false;
		let mut platforms = String::new();
		for platform in self.platforms.iter()
		{
			if afterFirst
			{
				platforms.push(' ');
			}
			else
			{
				afterFirst = true;
			}
			platforms.push_str(platform.to_str());
		}
		
		use self::CountryRestrictionInclusion::*;
		eventWriter.writePrefixedTextElement
		(
			namespace,
			&[
				"restriction".xml_str_attribute(match self.restriction
				{
					whitelist => "allow",
					blacklist => "deny",
				})
			],
			"video",
			"platform",
			&platforms
		)
	}
}
