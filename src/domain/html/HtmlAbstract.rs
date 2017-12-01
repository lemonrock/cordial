// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct HtmlAbstract
{
	pub(crate) htmlTitle: Rc<String>, // html head title; used by Google, Baidu (35 simplified chinese characters or 70 romanized)
	pub(crate) title: Rc<String>, // Open graph, twitter card title (70 chars)
	pub(crate) safariHtmlTitle: Option<Rc<String>>, // Safari title, overrides htmlTitle
	pub(crate) windowsTilesTitle: Option<Rc<String>>, // Windows tiles title, overrides htmlTitle
	pub(crate) description: Rc<String>, // html meta description (Baidu: 78 chars simplified chinese or 156 romanized), open graph description, RSS description (Feedly: maximum 140 chars), anchor title, twitter card description (maximum 200 chars)
	pub(crate) keywords_for_baidu: HashSet<String>, // https://searchengineland.com/the-b2b-marketers-guide-to-baidu-seo-180658 ; 3 - 5 keywords recommended
	#[serde(default)] pub(crate) shortlink: Option<Rc<UrlSerde>>,
	#[serde(default)] pub(crate) pingback: Option<Rc<UrlSerde>>,
}
