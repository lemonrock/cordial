// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) enum RegularAndPjaxStaticResponse
{
	Regular
	{
		response: StaticResponse,
		entityTag: String,
	},
	
	WithPjax
	{
		response: StaticResponse,
		entityTag: String,
		pjax: StaticResponse,
	},
	
	Unadorned
	{
		response: StaticResponse,
		entityTag: String,
	}
}

impl RegularAndPjaxStaticResponse
{
	#[inline(always)]
	pub(crate) fn regular(response: StaticResponse) -> Self
	{
		RegularAndPjaxStaticResponse::Regular
		{
			entityTag: response.entityTag(),
			response,
		}
	}
	
	#[inline(always)]
	pub(crate) fn both(response: StaticResponse, pjax: Option<StaticResponse>) -> Self
	{
		let entityTag = response.entityTag();
		
		if let Some(pjax) = pjax
		{
			RegularAndPjaxStaticResponse::WithPjax
			{
				response,
				entityTag,
				pjax
			}
		}
		else
		{
			RegularAndPjaxStaticResponse::Regular
			{
				response,
				entityTag,
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn unadorned(response: StaticResponse) -> Self
	{
		RegularAndPjaxStaticResponse::Unadorned
		{
			entityTag: response.entityTag(),
			response,
		}
	}
	
	#[inline(always)]
	pub(crate) fn entityTag<'a>(&'a self) -> &'a str
	{
		use self::RegularAndPjaxStaticResponse::*;
		
		match *self
		{
			Regular { ref entityTag, .. } => entityTag,
			WithPjax { ref entityTag, .. } => entityTag,
			Unadorned { ref entityTag, .. } => entityTag,
		}
	}
	
//	#[inline(always)]
//	fn response(&self) -> &StaticResponse
//	{
//		use self::RegularAndPjaxStaticResponse::*;
//
//		match *self
//		{
//			Regular { ref response, .. } => response,
//			WithPjax { ref response, .. } => response,
//			Unadorned { ref response, .. } => response,
//		}
//	}
//
//	#[inline(always)]
//	fn contentMimeType<'a>(&'a self) -> &'a Mime
//	{
//		&self.response().contentType.0
//	}
//
//	#[inline(always)]
//	pub(crate) fn toDataUri(&self) -> Url
//	{
//		use ::base64::encode_config as base64Encode;
//		use ::base64::STANDARD;
//		let dataUriString = format!("data:{};base64,{}", self.contentMimeType(), base64Encode(&self.response().uncompressedBody, STANDARD));
//		Url::parse(&dataUriString).unwrap()
//	}
	
	#[inline(always)]
	fn staticResponse(&self, isHead: bool, isPjax: bool, preferredEncoding: PreferredEncoding, lastModified: HttpDate, ifMatch: Option<&IfMatch>, ifUnmodifiedSince: Option<&IfUnmodifiedSince>, ifNoneMatch: Option<&IfNoneMatch>, ifModifiedSince: Option<&IfModifiedSince>, ifRange: Option<&IfRange>, range: Option<&Range>) -> Response
	{
		use self::RegularAndPjaxStaticResponse::*;
		
		match *self
		{
			Regular { ref response, ref entityTag } => response.respondAssumingResourceIs200Ok(isHead, preferredEncoding, entityTag, lastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range),
			
			WithPjax { ref response, ref entityTag, ref pjax } => if isPjax
			{
				pjax.respondAssumingResourceIs200Ok(isHead, preferredEncoding, entityTag, lastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range)
			}
			else
			{
				response.respondAssumingResourceIs200Ok(isHead, preferredEncoding, entityTag, lastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range)
			},
			
			Unadorned { ref response, .. } => response.rawResponse(isHead),
		}
	}
}
