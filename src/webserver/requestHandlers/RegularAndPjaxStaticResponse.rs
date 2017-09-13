// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RegularAndPjaxStaticResponse
{
	regular: StaticResponse,
	pjax: Option<StaticResponse>,
	entityTag: String,
}

impl RegularAndPjaxStaticResponse
{
	#[inline(always)]
	pub(crate) fn new(regular: StaticResponse, pjax: Option<StaticResponse>) -> Self
	{
		let entityTag = regular.entityTag();
		
		Self
		{
			regular,
			pjax,
			entityTag,
		}
	}
	
	#[inline(always)]
	pub(crate) fn entityTag<'a>(&'a self) -> &'a str
	{
		&self.entityTag
	}
	
	#[inline(always)]
	fn staticResponse(&self, isHead: bool, isPjax: bool, preferredEncoding: PreferredEncoding, lastModified: HttpDate, ifMatch: Option<&IfMatch>, ifUnmodifiedSince: Option<&IfUnmodifiedSince>, ifNoneMatch: Option<&IfNoneMatch>, ifModifiedSince: Option<&IfModifiedSince>, ifRange: Option<&IfRange>, range: Option<&Range>) -> Response
	{
		if isPjax && self.pjax.is_some()
		{
			self.pjax.as_ref().unwrap().staticResponse(isHead, preferredEncoding, &self.entityTag, lastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range)
		}
		else
		{
			self.regular.staticResponse(isHead, preferredEncoding, &self.entityTag, lastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range)
		}
	}
}
