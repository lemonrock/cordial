// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct HttpRedirectToHttpsRequestHandler
{
	portToRedirectTo: u16,
	ourHostNames: HashSet<String>,
	httpKeepAlive: bool,
}

impl RequestHandler for HttpRedirectToHttpsRequestHandler
{
	type AlternativeFuture = Empty<Response, ::hyper::Error>;
	
	#[inline(always)]
	fn isNotOneOfOurHostNames(&self, hostName: &str) -> bool
	{
		!self.ourHostNames.contains(hostName)
	}
	
	#[inline(always)]
	fn httpKeepAlive(&self) -> bool
	{
		self.httpKeepAlive
	}
	
	#[inline(always)]
	fn handle(&self, isHead: bool, method: Method, hostName: &str, port: u16, path: String, query: Option<String>, _requestHeaders: Headers, _requestBody: Body) -> Either<FutureResult<Response, ::hyper::Error>, Self::AlternativeFuture>
	{
		#[inline(always)]
		fn methods() -> Vec<Method>
		{
			vec![Options, Head, Get, Post, Delete, Put, Patch]
		}
		
		use ::hyper::Method::*;
		match method
		{
			Options => HttpService::<Self>::response(Response::options(methods())),
			Head | Get | Post | Delete | Put | Patch =>
			{
				let mut url = match Url::parse(&format!("https://{}:{}{}", hostName, port, path))
				{
					Err(_) => return HttpService::<Self>::response(Response::invalid_request_uri(isHead)),
					Ok(url) => url,
				};
				if let Some(query) = query
				{
					url.set_query(Some(&query));
				}
				HttpService::<Self>::response(Response::old_permanent_redirect(isHead, &url))
			},
			_ => HttpService::<Self>::response(Response::method_not_allowed(methods())),
		}
	}
}

impl HttpRedirectToHttpsRequestHandler
{
	#[inline(always)]
	pub(crate) fn new(portToRedirectTo: u16, ourHostNames: HashSet<String>, httpKeepAlive: bool) -> Self
	{
		Self
		{
			portToRedirectTo,
			ourHostNames,
			httpKeepAlive,
		}
	}
}
