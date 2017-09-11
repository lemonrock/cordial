// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct HttpsStaticRequestHandler
{
	resourcesByHostNameAndPathAndVariant: HashMap<String, Trie<String, RegularAndPjaxStaticResponse>>,
	httpKeepAlive: bool,
}

impl RequestHandler for HttpsStaticRequestHandler
{
	type AlternativeFuture = Empty<Response, ::hyper::Error>;
	
	#[inline(always)]
	fn isNotOneOfOurHostNames(&self, hostName: &str) -> bool
	{
		!self.resourcesByHostNameAndPathAndVariant.contains_key(hostName)
	}
	
	#[inline(always)]
	fn httpKeepAlive(&self) -> bool
	{
		self.httpKeepAlive
	}
	
	#[inline(always)]
	fn handle(&self, isHead: bool, method: Method, hostName: &str, _port: u16, path: String, query: Option<String>, requestHeaders: Headers, _requestBody: Body) -> Either<FutureResult<Response, ::hyper::Error>, Self::AlternativeFuture>
	{
		#[inline(always)]
		fn methods() -> Vec<Method>
		{
			vec![Options, Head, Get]
		}
		
		use ::hyper::Method::*;
		match method
		{
			Options => HttpService::<Self>::response(Response::options(methods())),
			Head | Get  => self.static_resource(isHead, hostName, path, query, requestHeaders),
			_ => HttpService::<Self>::response(Response::method_not_allowed(methods())),
		}
	}
}

impl HttpsStaticRequestHandler
{
	#[inline(always)]
	pub(crate) fn new(ourHostNames: &HashSet<String>, httpKeepAlive: bool) -> Self
	{
		let mut this = Self
		{
			resourcesByHostNameAndPathAndVariant: HashMap::with_capacity(ourHostNames.len()),
			httpKeepAlive,
		};
		for hostName in ourHostNames.iter()
		{
			this.resourcesByHostNameAndPathAndVariant.insert(hostName.to_owned(), Trie::new());
		}
		this
	}
	
	#[inline(always)]
	pub(crate) fn addResource(&mut self, url: Url, response: RegularAndPjaxStaticResponse)
	{
		let radixTrie = self.resourcesByHostNameAndPathAndVariant.get_mut(url.host_str().unwrap()).unwrap();
		radixTrie.insert(url.path().to_owned(), response);
		
	}
	
	#[inline(always)]
	fn static_resource(&self, isHead: bool, hostName: &str, path: String, _query: Option<String>, requestHeaders: Headers) -> Either<FutureResult<Response, ::hyper::Error>, <HttpsStaticRequestHandler as RequestHandler>::AlternativeFuture>
	{
		match self.resourcesByHostNameAndPathAndVariant.get(hostName)
		{
			None => HttpService::<Self>::response(Response::not_found(isHead)),
			Some(trie) => match trie.get(&path)
			{
				None => HttpService::<Self>::response(Response::not_found(isHead)),
				Some(regularAndPjaxStaticResponse) =>
				{
					let isPjax = requestHeaders.get_raw("X-PJAX").is_some();
					let preferredEncoding = PreferredEncoding::preferredEncoding(requestHeaders.get::<AcceptEncoding>());
					
					HttpService::<Self>::response(regularAndPjaxStaticResponse.staticResponse(isHead, isPjax, preferredEncoding))
				}
			}
		}
	}
}
