// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RssImage
{
	pub(crate) width: u32,
	pub(crate) height: u32,
	pub(crate) url: Url,
	pub(crate) fileSize: u64,
	pub(crate) mimeType: Mime,
	pub(crate) alt: String,
	pub(crate) credit: FullName,
	pub(crate) iso_639_1_alpha_2_language_code: Iso639Language,
}
