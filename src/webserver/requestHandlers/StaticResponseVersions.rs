// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) enum StaticResponseVersions
{
	Unversioned
	{
		#[serde(with = "url_serde")] url: Url,
		currentResponse: RegularAndPjaxStaticResponse,
		#[serde(with = "::serde_with::HttpDateSerde")] currentLastModified: HttpDate,
	},
	
	SingleVersion
	{
		#[serde(with = "url_serde")] versionedUrl: Url,
		currentResponse: RegularAndPjaxStaticResponse,
		currentVersionAsQuery: String,
		#[serde(with = "::serde_with::HttpDateSerde")] currentLastModified: HttpDate,
	},
	
	HasPrevisionVersion
	{
		#[serde(with = "url_serde")] versionedUrl: Url,
		currentResponse: RegularAndPjaxStaticResponse,
		currentVersionAsQuery: String,
		#[serde(with = "::serde_with::HttpDateSerde")] currentLastModified: HttpDate,
		previousResponse: RegularAndPjaxStaticResponse,
		previousVersionAsQuery: String,
		#[serde(with = "::serde_with::HttpDateSerde")] previousLastModified: HttpDate,
	},
	
	Discontinued
	{
		#[serde(with = "url_serde")] previousUrlOrVersionedUrl: Url,
		previousResponse: RegularAndPjaxStaticResponse,
		previousVersionAsQuery: Option<String>,
		#[serde(with = "::serde_with::HttpDateSerde")] previousLastModified: HttpDate,
	}
}

impl StaticResponseVersions
{
	#[inline(always)]
	pub(crate) fn lastModified(&self) -> HttpDate
	{
		use self::StaticResponseVersions::*;
		
		match *self
		{
			Unversioned { currentLastModified, .. } =>
			{
				currentLastModified
			}
			
			SingleVersion { currentLastModified, .. } =>
			{
				currentLastModified
			}
			
			HasPrevisionVersion { currentLastModified, .. } =>
			{
				currentLastModified
			}
			
			Discontinued { previousLastModified, .. } =>
			{
				previousLastModified
			}
		}
	}
	
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
			}
			
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
			
			Discontinued { ref previousUrlOrVersionedUrl, ref previousResponse, ref previousVersionAsQuery, previousLastModified } =>
			{
				if previousVersionAsQuery.is_none()
				{
					if query.is_none()
					{
						previousResponse.staticResponse(isHead, isPjax, preferredEncoding, previousLastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range)
					}
					else
					{
						Response::old_temporary_redirect(isHead, &previousUrlOrVersionedUrl)
					}
				}
				else
				{
					if query.is_none()
					{
						return Response::old_temporary_redirect(isHead, &previousUrlOrVersionedUrl);
					}
					
					let unwrapped = query.unwrap();
					if unwrapped.as_ref() == previousVersionAsQuery.as_ref().unwrap()
					{
						previousResponse.staticResponse(isHead, isPjax, preferredEncoding, previousLastModified, ifMatch, ifUnmodifiedSince, ifNoneMatch, ifModifiedSince, ifRange, range)
					}
					else
					{
						Response::old_temporary_redirect(isHead, &previousUrlOrVersionedUrl)
					}
				}
			}
		}
	}
}
