// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) enum StaticResponseVersions
{
	Unversioned
	{
		url: Url,
		currentResponse: RegularAndPjaxStaticResponse,
		currentLastModified: HttpDate,
	},
	
	SingleVersion
	{
		versionedUrl: Url,
		currentResponse: RegularAndPjaxStaticResponse,
		currentVersionAsQuery: String,
		currentLastModified: HttpDate,
	},
	
	HasPrevisionVersion
	{
		versionedUrl: Url,
		currentResponse: RegularAndPjaxStaticResponse,
		currentVersionAsQuery: String,
		currentLastModified: HttpDate,
		previousResponse: RegularAndPjaxStaticResponse,
		previousVersionAsQuery: String,
		previousLastModified: HttpDate,
	},
}

impl StaticResponseVersions
{
	#[inline(always)]
	fn staticResponse<'a>(&self, isHead: bool, isPjax: bool, preferredEncoding: PreferredEncoding, query: Option<Cow<'a, str>>, ifMatch: Option<&IfMatch>, ifUnmodifiedSince: Option<&IfUnmodifiedSince>, ifNoneMatch: Option<&IfNoneMatch>, ifModifiedSince: Option<&IfModifiedSince>, ifRange: Option<&IfRange>, range: Option<&Range>) -> Response
	{
		use self::StaticResponseVersions::*;
		
		match *self
		{
			Unversioned { ref url, ref currentResponse, currentLastModified } =>
			{
				if query.is_none()
				{
					currentResponse.staticResponse(isHead, isPjax, preferredEncoding, currentLastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range)
				}
				else
				{
					Response::old_temporary_redirect(isHead, &url)
				}
			},
			
			SingleVersion { ref versionedUrl, ref currentResponse, ref currentVersionAsQuery, currentLastModified } =>
			{
				if query.is_none()
				{
					return Response::old_temporary_redirect(isHead, &versionedUrl);
				}
				
				let unwrapped = query.unwrap();
				if unwrapped.as_ref() == currentVersionAsQuery.as_str()
				{
					currentResponse.staticResponse(isHead, isPjax, preferredEncoding, currentLastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range)
				}
				else
				{
					Response::old_temporary_redirect(isHead, &versionedUrl)
				}
			}
			
			HasPrevisionVersion { ref versionedUrl, ref currentResponse, ref currentVersionAsQuery, ref previousVersionAsQuery, ref previousResponse, currentLastModified, previousLastModified } =>
			{
				let unwrapped = query.unwrap();
				if unwrapped.as_ref() == currentVersionAsQuery.as_str()
				{
					currentResponse.staticResponse(isHead, isPjax, preferredEncoding, currentLastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range)
				}
				else if unwrapped.as_ref() == previousVersionAsQuery.as_str()
				{
					previousResponse.staticResponse(isHead, isPjax, preferredEncoding, previousLastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range)
				}
				else
				{
					Response::old_temporary_redirect(isHead, &versionedUrl)
				}
			}
		}
	}
}
