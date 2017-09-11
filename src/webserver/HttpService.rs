// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
// https://stackoverflow.com/questions/45806359/running-websocket-and-http-server-on-the-same-port-rust-hyper


#[derive(Debug, Clone)]
pub struct HttpService<R: RequestHandler>
{
	ourScheme: &'static str,
	ourDefaultPort: u16,
	requestHandler: Arc<R>,
}

impl<R: RequestHandler> HttpService<R>
{
	fn new(ourScheme: &'static str, ourDefaultPort: u16, requestHandler: Arc<R>) -> Self
	{
		Self
		{
			ourScheme,
			ourDefaultPort,
			requestHandler,
		}
	}
}

impl<R: RequestHandler> Service for HttpService<R>
{
	type Request = Request;
	
	type Response = Response;
	
	type Error = ::hyper::Error;
	
	type Future = Either<FutureResult<Self::Response, Self::Error>, R::AlternativeFuture>;
	
	fn call(&self, request: Request) -> Self::Future
	{
		use ::hyper::Method::*;
		use ::hyper::HttpVersion::*;
		let (method, uri, requestHttpVersion, requestHeaders, requestBody) = request.deconstruct();
		let isHead = method == Head;
		match requestHttpVersion
		{
			Http11 | H2 => (),
			Http09 | Http10 | H2c | _ => return Self::response(Response::unsupported_http_version(isHead)),
		};
		
		let (hostName, hostPort) = match requestHeaders.get::<Host>()
		{
			None => return Self::response(Response::http11_missing_host_header(isHead)),
			Some(host) => (host.hostname().to_owned(), host.port().unwrap_or(self.ourDefaultPort)),
		};
		
		let path = uri.path();
		
		// asterisk-form
		if path == "*"
		{
			if method == Options
			{
				Self::response(Response::options(vec![Options, Head, Get, Post, Delete, Put, Patch]))
			}
			else
			{
				Self::response(Response::asterisk_form_request_uri_is_only_allowed_for_OPTIONS_method(isHead))
			}
		}
		// authority-form
		else if path.is_empty()
		{
			if method == Connect
			{
				Self::response(Response::method_not_allowed(vec![]))
			}
			else
			{
				Self::response(Response::authority_form_request_uri_is_only_allowed_for_CONNECT_method(isHead))
			}
		}
		// invalid
		else if path.starts_with("//")
		{
			Self::response(Response::invalid_request_uri(isHead))
		}
		// either absolute-form or origin-form
		else
		{
			if uri.is_absolute()
			{
				match uri.scheme()
				{
					None => Self::response(Response::invalid_request_uri(isHead)),
					Some(scheme) => if scheme.to_ascii_lowercase() != self.ourScheme
					{
						Self::response(Response::unknown_or_unsupported_scheme_for_absolute_uri(isHead))
					}
					else
					{
						match uri.host()
						{
							None => Self::response(Response::invalid_request_uri(isHead)),
							Some(finalHostName) =>
							{
								let port = uri.port().unwrap_or(hostPort);
								self.safeguardRequest(isHead, method, finalHostName, port, path, uri.query(), requestHeaders, requestBody)
							}
						}
					}
				}
			}
			else
			{
				self.safeguardRequest(isHead, method, &hostName, hostPort, path, uri.query(), requestHeaders, requestBody)
			}
		}
	}
}

impl<R: RequestHandler> HttpService<R>
{
	#[inline(always)]
	fn safeguardRequest(&self, isHead: bool, method: Method, hostName: &str, port: u16, path: &str, query: Option<&str>, requestHeaders: Headers, requestBody: Body) -> <HttpService<R> as Service>::Future
	{
		// We only run on one port
		if port != self.ourDefaultPort
		{
			return Self::response(Response::authority_port_is_not_ours(isHead));
		}
		
		if self.requestHandler.isNotOneOfOurHostNames(hostName)
		{
			return Self::response(Response::authority_server_is_not_one_of_ours(isHead));
		}
		
		let path = match percent_decode(path.as_bytes()).decode_utf8()
		{
			Err(_) => return Self::response(Response::path_is_not_validly_encoded(isHead)),
			Ok(cow) => cow.into_owned(),
		};
		
		let query = if let Some(query) = query
		{
			match percent_decode(query.as_bytes()).decode_utf8()
			{
				Err(_) => return Self::response(Response::query_is_not_validly_encoded(isHead)),
				Ok(cow) => Some(cow.into_owned()),
			}
		}
		else
		{
			None
		};
		
		self.requestHandler.handle(isHead, method, hostName, port, path, query, requestHeaders, requestBody)
	}
	
	#[inline(always)]
	pub fn response(response: Response) -> Either<FutureResult<<HttpService<R> as Service>::Response, <HttpService<R> as Service>::Error>, R::AlternativeFuture>
	{
		Either::A(ok(response))
	}
}

