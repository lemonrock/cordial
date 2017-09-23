// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct HttpsStaticRequestHandler
{
	resources: Arc<Resources>,
	httpKeepAlive: bool,
}

impl RequestHandler for HttpsStaticRequestHandler
{
	type AlternativeFuture = Empty<Response, ::hyper::Error>;
	
	#[inline(always)]
	fn isNotOneOfOurHostNames(&self, hostName: &str) -> bool
	{
		self.resources.isNotOneOfOurHostNames(hostName)
	}
	
	#[inline(always)]
	fn httpKeepAlive(&self) -> bool
	{
		self.httpKeepAlive
	}
	
	#[inline(always)]
	fn handle<'a>(&self, isHead: bool, method: Method, hostName: &str, _port: u16, path: Cow<'a, str>, query: Option<Cow<'a, str>>, requestHeaders: Headers, _requestBody: Body) -> Either<FutureResult<Response, ::hyper::Error>, Self::AlternativeFuture>
	{
		#[inline(always)]
		fn methods() -> Vec<Method>
		{
			vec![Options, Head, Get]
		}
		
		#[inline(always)]
		fn deny<H, R>(value: Option<H>) -> Option<Vec<R>>
		{
			if value.is_some()
			{
				Some(vec![])
			}
			else
			{
				None
			}
		}
		
		use ::hyper::Method::*;
		match method
		{
			Options =>
			{
				let origin = requestHeaders.get::<Origin>();
				
				if let Some(origin) = origin
				{
					let accessControlRequestMethods = requestHeaders.get::<AccessControlRequestMethod>();
					let accessControlRequestHeaders = requestHeaders.get::<AccessControlRequestHeaders>();
					
					let ourOrigin = AccessControlAllowOrigin::Value(hostName.to_string());
					
					if !origin.is_null()
					{
						if origin.scheme() != Some("http")
						{
							return HttpService::<Self>::response(Response::options(methods(), Some((ourOrigin, deny(accessControlRequestMethods), deny(accessControlRequestHeaders)))))
						}
						
						if let Some(host) = origin.host()
						{
							if self.isNotOneOfOurHostNames(host.hostname())
							{
								return HttpService::<Self>::response(Response::options(methods(), Some((ourOrigin, deny(accessControlRequestMethods), deny(accessControlRequestHeaders)))));
							}
						}
						else
						{
							return HttpService::<Self>::response(Response::options(methods(), Some((ourOrigin, deny(accessControlRequestMethods), deny(accessControlRequestHeaders)))));
						}
					};
					
					let allowMethods = if let Some(accessControlRequestMethods) = accessControlRequestMethods
					{
						Some(methods())
					}
					else
					{
						None
					};
					
					let allowHeaders = if let Some(accessControlRequestHeaders) = accessControlRequestHeaders
					{
						Some(accessControlRequestHeaders.0.clone())
					}
					else
					{
						None
					};
					
					HttpService::<Self>::response(Response::options(methods(), Some((ourOrigin, allowMethods, allowHeaders))))
				}
				else
				{
					HttpService::<Self>::response(Response::options(methods(), None))
				}
			},
			Head | Get  => self.response(isHead, hostName, path, query, requestHeaders),
			_ => HttpService::<Self>::response(Response::method_not_allowed(methods())),
		}
	}
}

impl HttpsStaticRequestHandler
{
	#[inline(always)]
	pub(crate) fn new(resources: Resources, httpKeepAlive: bool) -> Self
	{
		Self
		{
			resources: Arc::new(resources),
			httpKeepAlive,
		}
	}
	
	#[inline(always)]
	pub(crate) fn resources(&self) -> Arc<Resources>
	{
		self.resources.clone()
	}
	
	#[inline(always)]
	fn response<'a>(&self, isHead: bool, hostName: &str,path: Cow<'a, str>, query: Option<Cow<'a, str>>, requestHeaders: Headers) -> Either<FutureResult<Response, ::hyper::Error>, <HttpsStaticRequestHandler as RequestHandler>::AlternativeFuture>
	{
		HttpService::<Self>::response(self.resources.response(isHead, hostName, path, query, requestHeaders))
	}
}
