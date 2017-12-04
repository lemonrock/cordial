// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct VideoCountryRestriction
{
	#[serde(default)] pub(crate) restriction: CountryRestrictionInclusion,
	#[serde(default)] pub(crate) countries: BTreeSet<Iso3166Dash1Alpha2CountryCode>,
}

impl Default for VideoCountryRestriction
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			restriction: Default::default(),
			countries: Default::default(),
		}
	}
}

impl VideoCountryRestriction
{
	#[inline(always)]
	pub(crate) fn writeXmlForRestriction<'a, W: Write>(&self, eventWriter: &mut EventWriter<W>, namespace: &Namespace) -> Result<(), CordialError>
	{
		if self.countries.is_empty()
		{
			return Ok(());
		}
		
		let mut afterFirst = false;
		let mut countries = String::new();
		for country in self.countries.iter()
		{
			if afterFirst
			{
				countries.push(' ');
			}
			else
			{
				afterFirst = true;
			}
			countries.push_str(country.to_iso_3166_1_alpha_2_language_code());
		}
		
		use self::CountryRestrictionInclusion::*;
		let attributes =
		[
			XmlAttribute::new(Name::local("restriction"), match self.restriction
			{
				whitelist => "allow",
				blacklist => "deny",
			}),
		];
		eventWriter.writePrefixedTextElement(namespace, &attributes, "video", "restriction", &countries)
	}
}
