// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct HttpsStaticRequestHandler
{
}

impl RequestHandler for HttpsStaticRequestHandler
{
	type AlternativeFuture = Empty<Response, ::hyper::Error>;
	
	#[inline(always)]
	fn handle(&self, isHead: bool, method: Method, hostName: &str, port: u16, path: String, query: Option<String>, _requestHeaders: Headers, _requestBody: Body) -> Either<FutureResult<Response, ::hyper::Error>, Self::AlternativeFuture>
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
			Head | Get  => self.static_resource(isHead, hostName, path, query, _requestHeaders),
			_ => HttpService::<Self>::response(Response::method_not_allowed(methods())),
		}
	}
}

impl HttpsStaticRequestHandler
{
	#[inline(always)]
	fn static_resource(&self, isHead: bool, hostName: &str, path: String, query: Option<String>, _requestHeaders: Headers) -> Either<FutureResult<Response, ::hyper::Error>, <HttpsStaticRequestHandler as RequestHandler>::AlternativeFuture>
	{
		HttpService::<Self>::response(Response::SOMETHING_FOR_NOW(isHead))
	}
}
