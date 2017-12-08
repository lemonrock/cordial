// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Responses
{
	resourcesByHostNameAndPathAndQueryString: HashMap<String, Trie<String, StaticResponseVersions>>,
	#[serde(with = "::serde_with::HttpDateSerde")] deploymentDate: HttpDate,
}

impl Responses
{
	#[inline(always)]
	pub(crate) fn empty(deploymentDate: SystemTime) -> Self
	{
		Self::new(deploymentDate, &HashSet::with_capacity(0))
	}
	
	#[inline(always)]
	pub(crate) fn new(deploymentDate: SystemTime, ourHostNames: &HashSet<String>) -> Self
	{
		let mut resourcesByHostNameAndPathAndQueryString = HashMap::with_capacity(ourHostNames.len());
		
		for hostName in ourHostNames.iter()
		{
			resourcesByHostNameAndPathAndQueryString.insert(hostName.to_owned(), Trie::new());
		}
		
		Self
		{
			resourcesByHostNameAndPathAndQueryString,
			deploymentDate: HttpDate::from(deploymentDate),
		}
	}
	
	#[inline(always)]
	pub(crate) fn addResponse(&mut self, url: Url, currentResponse: RegularAndPjaxStaticResponse, oldResponses: Arc<Responses>) -> HttpDate
	{
		let hostName = url.host_str().unwrap();
		let path = url.path().to_owned();
		let currentVersionAsQuery = url.query();
		let (previousLastModifiedAndPreviousResponse, previousVersionAsQuery) = oldResponses.previous(hostName, &path);
		
		use self::StaticResponseVersions::*;
		let staticResponseVersions = match previousLastModifiedAndPreviousResponse
		{
			None => Unversioned
			{
				url: url.clone(),
				currentResponse,
				currentLastModified: self.deploymentDate,
			},
			Some((previousLastModified, previousResponse)) =>
			{
				let currentLastModified = if currentResponse.entityTag() == previousResponse.entityTag()
				{
					previousLastModified
				}
				else
				{
					self.deploymentDate
				};
				
				if let Some(currentVersionAsQuery) = currentVersionAsQuery
				{
					let versionedUrl = url.clone();
					match previousVersionAsQuery
					{
						None => SingleVersion
						{
							versionedUrl,
							currentResponse,
							currentVersionAsQuery: currentVersionAsQuery.to_owned(),
							currentLastModified,
						},
						Some(previousVersionAsQuery) => HasPrevisionVersion
						{
							versionedUrl,
							currentResponse,
							currentVersionAsQuery: currentVersionAsQuery.to_owned(),
							currentLastModified,
							previousResponse: previousResponse.clone(),
							previousVersionAsQuery: previousVersionAsQuery.to_owned(),
							previousLastModified,
						}
					}
				}
				else
				{
					Unversioned
					{
						url: url.clone(),
						currentResponse,
						currentLastModified,
					}
				}
			}
		};
		
		let lastModified = staticResponseVersions.lastModified();
		
		let radixTrie = self.resourcesByHostNameAndPathAndQueryString.get_mut(hostName).unwrap();
		radixTrie.insert(path, staticResponseVersions);
		
		lastModified
	}
	
	#[inline(always)]
	fn isNotOneOfOurHostNames(&self, hostName: &str) -> bool
	{
		!self.resourcesByHostNameAndPathAndQueryString.contains_key(hostName)
	}
	
	#[inline(always)]
	fn previous<'a>(&'a self, hostName: &str, path: &str) -> (Option<(HttpDate, &'a RegularAndPjaxStaticResponse)>, Option<&'a str>)
	{
		use self::StaticResponseVersions::*;
		
		match self.resourcesByHostNameAndPathAndQueryString.get(hostName)
		{
			None => (None, None),
			Some(trie) => match trie.get(path)
			{
				None => (None, None),
				Some(staticResponseVersions) => match staticResponseVersions
				{
					&Unversioned { ref currentLastModified, ref currentResponse, .. } => (Some((*currentLastModified, currentResponse)), None),
					&SingleVersion { ref currentLastModified, ref currentResponse, ref currentVersionAsQuery, .. } => (Some((*currentLastModified, currentResponse)), Some(currentVersionAsQuery)),
					&HasPrevisionVersion { ref currentLastModified, ref currentResponse, ref currentVersionAsQuery, .. } => (Some((*currentLastModified, currentResponse)), Some(currentVersionAsQuery)),
					&Discontinued { .. } => (None, None),
				}
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn addAnythingThatIsDiscontinued(&mut self, oldResponses: &Arc<Self>)
	{
		use self::StaticResponseVersions::*;
		
		for (hostName, trie) in oldResponses.resourcesByHostNameAndPathAndQueryString.iter()
		{
			let ourTrieByPath = self.resourcesByHostNameAndPathAndQueryString.get_mut(hostName).unwrap();
			
			for (path, staticResponseVersion) in trie.iter()
			{
				if ourTrieByPath.get(path).is_none()
				{
					if let Some((previousUrlOrVersionedUrl, previousResponse, previousVersionAsQuery, previousLastModified)) = match staticResponseVersion
					{
						&Unversioned { ref url, currentLastModified, ref currentResponse, .. } => Some((url, currentResponse, None, currentLastModified)),
						&SingleVersion { ref versionedUrl, currentLastModified, ref currentResponse, ref currentVersionAsQuery, .. } => Some((versionedUrl, currentResponse, Some(currentVersionAsQuery), currentLastModified)),
						&HasPrevisionVersion { ref versionedUrl, currentLastModified, ref currentResponse, ref currentVersionAsQuery, .. } => Some((versionedUrl, currentResponse, Some(currentVersionAsQuery), currentLastModified)),
						&Discontinued { .. } => None,
					}
					{
						ourTrieByPath.insert(path.to_owned(), Discontinued
						{
							previousUrlOrVersionedUrl: previousUrlOrVersionedUrl.clone(),
							previousResponse: previousResponse.clone(),
							previousVersionAsQuery: previousVersionAsQuery.cloned(),
							previousLastModified
						});
					}
				}
			}
		}
	}
	
	#[inline(always)]
	fn response<'a>(&self, isHead: bool, hostName: &str, path: Cow<'a, str>, query: Option<Cow<'a, str>>, requestHeaders: Headers) -> Response
	{
		match self.resourcesByHostNameAndPathAndQueryString.get(hostName)
		{
			None => Response::not_found(isHead),
			Some(trie) => match trie.get(path.as_ref())
			{
				None => Response::not_found(isHead),
				Some(staticResponseVersions) =>
				{
					let origin = requestHeaders.get::<Origin>();
					let ourOrigin = if let Some(origin) = origin
					{
						if origin.is_null()
						{
							return Response::illegal_origin(isHead);
						}
						
						if origin.scheme() != Some("https")
						{
							return Response::illegal_origin(isHead);
						}
						
						match origin.host()
						{
							None =>	return Response::illegal_origin(isHead),
							Some(host) =>
							{
								let theirOriginHostName = host.hostname();
								if self.isNotOneOfOurHostNames(theirOriginHostName)
								{
									return Response::illegal_origin(isHead);
								}
								Some(AccessControlAllowOrigin::Value(format!("https://{}", theirOriginHostName)))
							}
						}
					}
					else
					{
						None
					};
					
					let isPjax = requestHeaders.get_raw("X-PJAX").is_some();
					let preferredEncoding = PreferredCompression::preferredEncoding(requestHeaders.get::<AcceptEncoding>());
					
					let response = staticResponseVersions.staticResponse(isHead, isPjax, preferredEncoding, query, requestHeaders.get::<IfMatch>(), requestHeaders.get::<IfUnmodifiedSince>(), requestHeaders.get::<IfNoneMatch>(), requestHeaders.get::<IfModifiedSince>(), requestHeaders.get::<IfRange>(), requestHeaders.get::<Range>());
					
					if let Some(ourOrigin) = ourOrigin
					{
						response.with_header(ourOrigin)
					}
					else
					{
						response
					}
				}
			}
		}
	}
}
