// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct Resources
{
	resourcesByHostNameAndPathAndQueryString: HashMap<String, Trie<String, StaticResponseVersions>>,
}

impl Resources
{
	#[inline(always)]
	pub(crate) fn empty() -> Self
	{
		Self::new(&HashSet::with_capacity(0))
	}
	
	#[inline(always)]
	pub(crate) fn new(ourHostNames: &HashSet<String>) -> Self
	{
		let mut resourcesByHostNameAndPathAndQueryString = HashMap::with_capacity(ourHostNames.len());
		
		for hostName in ourHostNames.iter()
		{
			resourcesByHostNameAndPathAndQueryString.insert(hostName.to_owned(), Trie::new());
		}
		
		Self
		{
			resourcesByHostNameAndPathAndQueryString,
		}
	}
	
	#[inline(always)]
	pub(crate) fn addResource(&mut self, url: Url, currentResponse: RegularAndPjaxStaticResponse, oldResources: Arc<Resources>)
	{
		use self::StaticResponseVersions::*;
		
		let hostName = url.host_str().unwrap();
		let path = url.path().to_owned();
		let currentVersionAsQuery = url.query();
		
		let staticResponseVersions = if let Some(currentVersionAsQuery) = currentVersionAsQuery
		{
			let versionedUrl = url.clone();
			match oldResources.latestVersionIfVersioned(hostName, &path)
			{
				None => SingleVersion
				{
					versionedUrl,
					currentResponse,
					currentVersionAsQuery: currentVersionAsQuery.to_owned(),
				},
				Some((previousVersionAsQuery, previousResponse)) => HasPrevisionVersion
				{
					versionedUrl,
					currentResponse,
					currentVersionAsQuery: currentVersionAsQuery.to_owned(),
					previousResponse,
					previousVersionAsQuery: previousVersionAsQuery.to_owned(),
				}
			}
		}
		else
		{
			Unversioned
			{
				url: url.clone(),
				currentResponse,
			}
		};
		
		let radixTrie = self.resourcesByHostNameAndPathAndQueryString.get_mut(hostName).unwrap();
		radixTrie.insert(path, staticResponseVersions);
	}
	
	#[inline(always)]
	fn isNotOneOfOurHostNames(&self, hostName: &str) -> bool
	{
		!self.resourcesByHostNameAndPathAndQueryString.contains_key(hostName)
	}
	
	#[inline(always)]
	fn latestVersionIfVersioned(&self, hostName: &str, path: &str) -> Option<(String, RegularAndPjaxStaticResponse)>
	{
		use self::StaticResponseVersions::*;
		
		match self.resourcesByHostNameAndPathAndQueryString.get(hostName)
		{
			None => None,
			Some(trie) => match trie.get(path)
			{
				None => None,
				Some(staticResponseVersions) => match staticResponseVersions
				{
					&Unversioned { .. } => None,
					&SingleVersion { ref currentVersionAsQuery, ref currentResponse, .. } => Some((currentVersionAsQuery.to_owned(), currentResponse.clone())),
					&HasPrevisionVersion { ref currentVersionAsQuery, ref currentResponse, .. } => Some((currentVersionAsQuery.to_owned(), currentResponse.clone())),
				}
			}
		}
	}
	
	#[inline(always)]
	fn response(&self, isHead: bool, hostName: &str, path: String, query: Option<String>, requestHeaders: Headers) -> Response
	{
		match self.resourcesByHostNameAndPathAndQueryString.get(hostName)
		{
			None => Response::not_found(isHead),
			Some(trie) => match trie.get(&path)
			{
				None => Response::not_found(isHead),
				Some(staticResponseVersions) =>
				{
					let isPjax = requestHeaders.get_raw("X-PJAX").is_some();
					let preferredEncoding = PreferredEncoding::preferredEncoding(requestHeaders.get::<AcceptEncoding>());
					
					staticResponseVersions.staticResponse(isHead, isPjax, preferredEncoding, query)
				}
			}
		}
	}
	
	//	#[inline(always)]
	//	fn relativeOutputContentFilePath(&self, language: &language, variant: Variant, version: Option<&str>) -> Result<PathBuf, CordialError>
	//	{
	//		let url = self.url(language, variant)?;
	//		let fileLikeUrl = if let Some(additionalContentFileName) = self.additionalContentFileNameIfAny
	//		{
	//			url.join(additionalContentFileName).unwrap()
	//		}
	//		else
	//		{
	//			url
	//		};
	//
	//		let mut resourceRelativePathString = String::with_capacity(1024);
	//		resourceRelativePathString.push_str(fileLikeUrl.host_str().unwrap());
	//		resourceRelativePathString.push_str(fileLikeUrl.path());
	//		if let Some(version) = version
	//		{
	//			resourceRelativePathString.push_str(variant.fileExtensionWithLeadingPeriod());
	//		}
	//		resourceRelativePathString.push_str(variant.fileExtensionWithLeadingPeriod());
	//		Ok(PathBuf::from(resourceRelativePathString))
	//	}
}
