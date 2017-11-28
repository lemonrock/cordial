// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct TwitterCardImageMatch
{
	pub(crate) ratio: u32,
	pub(crate) minimumWidth: u32,
	pub(crate) minimumHeight: u32,
	pub(crate) maximumWidth: u32,
	pub(crate) maximumHeight: u32,
	pub(crate) maximumSize: u64,
}

impl TwitterCardImageMatch
{
	pub(crate) const Summary: TwitterCardImageMatch = TwitterCardImageMatch
	{
		ratio: 1,
		minimumWidth: 144,
		minimumHeight: 144,
		maximumWidth: 4096,
		maximumHeight: 4096,
		maximumSize: 5 * 1024 * 1024,
	};
	
	pub(crate) const SummaryLargeImage: TwitterCardImageMatch = TwitterCardImageMatch
	{
		ratio: 2,
		minimumWidth: 300,
		minimumHeight: 157,
		maximumWidth: 4096,
		maximumHeight: 4096,
		maximumSize: 5 * 1024 * 1024,
	};
}
