// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct HttpsStaticRequestHandler
{
	pub(crate) responses: Arc<Responses>,
	pub(crate) httpKeepAlive: bool,
	pub(crate) hstsPreloadingEnabledForProduction: bool,
	pub(crate) allowSearchEngineIndexingForProduction: bool,
}

impl RequestHandler for HttpsStaticRequestHandler
{
	type AlternativeFuture = Empty<Response, ::hyper::Error>;
	
	#[inline(always)]
	fn isNotOneOfOurHostNames(&self, hostName: &str) -> bool
	{
		self.responses.isNotOneOfOurHostNames(hostName)
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
		let response = match method
		{
			Options =>
			{
				let origin = requestHeaders.get::<Origin>();
				
				if let Some(origin) = origin
				{
					let accessControlRequestMethods = requestHeaders.get::<AccessControlRequestMethod>();
					let accessControlRequestHeaders = requestHeaders.get::<AccessControlRequestHeaders>();
					
					let ourOrigin = if origin.is_null()
					{
						return HttpService::<Self>::response(Response::options(methods(), Some((AccessControlAllowOrigin::Null, deny(accessControlRequestMethods), deny(accessControlRequestHeaders)))))
					}
					else
					{
						if origin.scheme() != Some("http")
						{
							return HttpService::<Self>::response(Response::options(methods(), Some((AccessControlAllowOrigin::Null, deny(accessControlRequestMethods), deny(accessControlRequestHeaders)))))
						}
						
						let ourOrigin = if let Some(host) = origin.host()
						{
							let theirOriginHostName = host.hostname();
							if self.isNotOneOfOurHostNames(theirOriginHostName)
							{
								return HttpService::<Self>::response(Response::options(methods(), Some((AccessControlAllowOrigin::Null, deny(accessControlRequestMethods), deny(accessControlRequestHeaders)))));
							}
							
							AccessControlAllowOrigin::Value(format!("https://{}", theirOriginHostName))
						}
						else
						{
							return HttpService::<Self>::response(Response::options(methods(), Some((AccessControlAllowOrigin::Null, deny(accessControlRequestMethods), deny(accessControlRequestHeaders)))));
						};
						
						ourOrigin
					};
					
					let allowMethods = if accessControlRequestMethods.is_some()
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
					
					Response::options(methods(), Some((ourOrigin, allowMethods, allowHeaders)))
				}
				else
				{
					Response::options(methods(), None)
				}
			},
			Head | Get  => self.responses.response(isHead, hostName, path, query, requestHeaders),
			_ => Response::method_not_allowed(methods()),
		};
		
		let response = if self.hstsPreloadingEnabledForProduction
		{
			response.with_header(Strict_Transport_Security::Default)
		}
		else
		{
			response
		};
		
		let response = if self.allowSearchEngineIndexingForProduction
		{
			response
		}
		else
		{
			response.with_header(X_Robots_Tag::Default)
		};
		
		HttpService::<Self>::response(response)
	}
}

impl HttpsStaticRequestHandler
{
	#[inline(always)]
	pub(crate) fn responses(&self) -> Arc<Responses>
	{
		self.responses.clone()
	}
}
