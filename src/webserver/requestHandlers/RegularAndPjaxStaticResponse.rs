// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct RegularAndPjaxStaticResponse
{
	pub regular: StaticResponse,
	pub pjax: Option<StaticResponse>,
}

impl RegularAndPjaxStaticResponse
{
	#[inline(always)]
	fn staticResponse(&self, isHead: bool, isPjax: bool, preferredEncoding: PreferredEncoding) -> Response
	{
		if isPjax && self.pjax.is_some()
		{
			self.pjax.as_ref().unwrap().staticResponse(isHead, preferredEncoding)
		}
		else
		{
			self.regular.staticResponse(isHead, preferredEncoding)
		}
	}
}
