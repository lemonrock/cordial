// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RssItemLanguageSpecific
{
	pub(crate) webPageDescription: Rc<String>, // put into <title>
	pub(crate) webPageUsefulContentHtml: Vec<u8>, // put into <description>
	// <content:encoded> could be supported but isn't
	pub(crate) languageSpecificUrl: Url,
	pub(crate) primaryImage: Option<RssImage>,
}
