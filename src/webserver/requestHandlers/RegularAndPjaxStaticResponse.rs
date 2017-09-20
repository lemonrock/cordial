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
	pub(crate) fn regular(regular: StaticResponse) -> Self
	{
		Self::both(regular, None)
	}
	
	#[inline(always)]
	pub(crate) fn both(regular: StaticResponse, pjax: Option<StaticResponse>) -> Self
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
	pub(crate) fn contentMimeType<'a>(&'a self) -> &'a Mime
	{
		&self.regular.contentType.0
	}
	
	#[inline(always)]
	pub(crate) fn toDataUri(&self) -> Url
	{
		let dataUriString = format!("data:{};base64,{}", resource.contentMimeType(), base64Encode(&regular.uncompressedBody, STANDARD));
		Url.parse(dataUriString).unwrap()
	}
	
	#[inline(always)]
	pub(crate) fn contentMimeTypeWithoutParameters<'a>(&'a self) -> Mime
	{
		let mimeTypeWithParameters = self.contentMimeType();
		
		let type_ = mimeTypeWithParameters.type_();
		let subtype = mimeTypeWithParameters.subtype();
		
		let mimeString = if let Some(suffix) = mimeTypeWithParameters.suffix()
		{
			format!("{}/{}+{}", type_, subtype, suffix)
		}
		else
		{
			format!("{}/{}", type_, subtype)
		};
		mimeString.parse().unwrap()
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
