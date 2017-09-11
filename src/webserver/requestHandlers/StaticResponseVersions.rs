// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) enum StaticResponseVersions
{
	Unversioned
	{
		url: Url,
		currentResponse: RegularAndPjaxStaticResponse
	},
	
	SingleVersion
	{
		versionedUrl: Url,
		currentResponse: RegularAndPjaxStaticResponse,
		currentVersionAsQuery: String,
	},
	
	HasPrevisionVersion
	{
		versionedUrl: Url,
		currentResponse: RegularAndPjaxStaticResponse,
		currentVersionAsQuery: String,
		previousResponse: RegularAndPjaxStaticResponse,
		previousVersionAsQuery: String,
	},
}

impl StaticResponseVersions
{
	#[inline(always)]
	fn staticResponse(&self, isHead: bool, isPjax: bool, preferredEncoding: PreferredEncoding, query: Option<String>) -> Response
	{
		use self::StaticResponseVersions::*;
		
		match *self
		{
			Unversioned { ref url, ref currentResponse } =>
			{
				if query.is_none()
				{
					currentResponse.staticResponse(isHead, isPjax, preferredEncoding)
				}
				else
				{
					Response::old_temporary_redirect(isHead, &url)
				}
			},
			
			SingleVersion { ref versionedUrl, ref currentResponse, ref currentVersionAsQuery } =>
			{
				if query.is_none()
				{
					return Response::old_temporary_redirect(isHead, &versionedUrl);
				}
				
				let unwrapped = &query.unwrap();
				if unwrapped == currentVersionAsQuery
				{
					currentResponse.staticResponse(isHead, isPjax, preferredEncoding)
				}
				else if unwrapped == ""
				{
					Response::old_temporary_redirect(isHead, &versionedUrl)
				}
				else
				{
					Response::not_found(isHead)
				}
			}
			
			HasPrevisionVersion { ref versionedUrl, ref currentResponse, ref currentVersionAsQuery, ref previousVersionAsQuery, ref previousResponse } =>
			{
				let unwrapped = &query.unwrap();
				if unwrapped == currentVersionAsQuery
				{
					currentResponse.staticResponse(isHead, isPjax, preferredEncoding)
				}
				else if unwrapped == previousVersionAsQuery
				{
					previousResponse.staticResponse(isHead, isPjax, preferredEncoding)
				}
				else if unwrapped == ""
				{
					Response::old_temporary_redirect(isHead, &versionedUrl)
				}
				else
				{
					Response::not_found(isHead)
				}
			}
		}
	}
}
