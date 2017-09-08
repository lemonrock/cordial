// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub trait RequestHandler: Debug + Clone
{
	type AlternativeFuture: Future<Item=Response, Error=::hyper::Error>;
	
	#[inline(always)]
	fn handle(&self, isHead: bool, method: Method, hostName: &str, port: u16, path: String, query: Option<String>, requestHeaders: Headers, requestBody: Body) -> Either<FutureResult<Response, ::hyper::Error>, Self::AlternativeFuture>;
}
