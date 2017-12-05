// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct GooglePlayRssChannel
{
	// We should delegate to ITunesRssChannel for:-
	// author
	// email
	// image
	// description
	// Like apple itunes, Link is a link to a web page containing the podcast
	// category - like itunes, values are here: https://support.google.com/googleplay/podcasts/answer/6260341?hl=en&ref_topic=6249881
	
}

impl Default for GooglePlayRssChannel
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
		}
	}
}

impl GooglePlayRssChannel
{
	//noinspection SpellCheckingInspection
	pub(crate) const GooglePlayNamespacePrefix: &'static str = "googleplay";
	
	pub(crate) const GooglePlayNamespaceUrl: &'static str = "http://www.google.com/schemas/play-podcasts/1.0";
	
	#[inline(always)]
	pub(crate) fn writeXml<'a, 'b: 'a, 'c, W: Write>(&'c self, eventWriter: &mut EventWriter<W>, namespace: &Namespace, emptyAttributes: &[XmlAttribute<'c>], fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, _resources: &'a Resources) -> Result<(), CordialError>
	{
		Ok(())
	}
}
