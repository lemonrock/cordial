// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


trait CommonResponses: Sized
{
	#[inline(always)]
	fn static_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, contentType: ContentType, body: I) -> Self;
	
	#[inline(always)]
	fn static_html_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, body: I) -> Self;
	
	#[inline(always)]
	fn options(permittedMethods: Vec<Method>) -> Self;
	
	#[inline(always)]
	fn method_not_allowed(permittedMethods: Vec<Method>) -> Self;
	
	#[inline(always)]
	fn misdirected_request(isHead: bool) -> Self;
	
	#[inline(always)]
	fn old_permanent_redirect(isHead: bool, url: &Url) -> Self;
	
	#[inline(always)]
	fn bad_request<I: Into<Cow<'static, str>>>(isHead: bool, body: I) -> Self;
	
	#[inline(always)]
	fn invalid_request_uri(isHead: bool) -> Self;
	
	#[inline(always)]
	fn path_is_not_validly_encoded(isHead: bool) -> Self;
	
	#[inline(always)]
	fn query_is_not_validly_encoded(isHead: bool) -> Self;
	
	#[inline(always)]
	fn http11_missing_host_header(isHead: bool) -> Self;
	
	#[inline(always)]
	fn unsupported_http_version(isHead: bool) -> Self;
	
	#[inline(always)]
	fn asterisk_form_request_uri_is_only_allowed_for_OPTIONS_method(isHead: bool) -> Self;
	
	#[inline(always)]
	fn authority_form_request_uri_is_only_allowed_for_CONNECT_method(isHead: bool) -> Self;
	
	#[inline(always)]
	fn unknown_or_unsupported_scheme_for_absolute_uri(isHead: bool) -> Self;
	
	#[inline(always)]
	fn authority_port_is_not_ours(isHead: bool) -> Self;
	
	#[inline(always)]
	fn authority_server_is_not_one_of_ours(isHead: bool) -> Self;
	
	fn SOMETHING_FOR_NOW(isHead: bool) -> Self;
}

impl CommonResponses for Response
{
	#[inline(always)]
	fn static_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, contentType: ContentType, body: I) -> Self
	{
		let body = body.into();
		let response = Response::new()
		.with_status(statusCode)
		.with_header(contentType)
		.with_header(ContentLength(body.len() as u64))
		.with_header(X_XSS_Protection::Default)
		.with_header(X_Content_Type_Options::Default)
		.with_header(X_Frame_Options::Default)
		.with_header(Date(SystemTime::now().into()));
		
		if isHead
		{
			response.with_body(Body::empty())
		}
		else
		{
			response.with_body(body)
		}
	}
	
	#[inline(always)]
	fn static_html_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, body: I) -> Self
	{
		Self::static_response(isHead, statusCode, ContentType::html(), body)
	}
	
	#[inline(always)]
	fn options(permittedMethods: Vec<Method>) -> Self
	{
		Self::static_html_response(false, StatusCode::Ok, "<!doctype html><title>Options</title><p>Options.")
		.with_header(commonCacheControlHeader(60))
		.with_header(Allow(permittedMethods))
	}
	
	#[inline(always)]
	fn method_not_allowed(permittedMethods: Vec<Method>) -> Self
	{
		Self::static_html_response(false, StatusCode::MethodNotAllowed, "<!doctype html><title>Method not allowed</title><p>Method not allowed.")
		.with_header(commonCacheControlHeader(60))
		.with_header(Allow(permittedMethods))
	}
	
	#[inline(always)]
	fn misdirected_request(isHead: bool) -> Self
	{
		Self::static_html_response(isHead, StatusCode::MisdirectedRequest, "<!doctype html><title>Misdirected request</title><p>Misdirected request.")
		.with_header(commonCacheControlHeader(60))
	}
	
	#[inline(always)]
	fn old_permanent_redirect(isHead: bool, url: &Url) -> Self
	{
		Self::static_html_response(isHead, StatusCode::MovedPermanently, format!("<!doctype html><title>Moved permanently</title><p>The document has moved <a href='{}'>here</a>.", url))
		.with_header(commonCacheControlHeader(31536000))
		.with_header(Location::new(url.as_ref().to_owned()))
	}
	
	#[inline(always)]
	fn bad_request<I: Into<Cow<'static, str>>>(isHead: bool, body: I) -> Self
	{
		Self::static_html_response(isHead, StatusCode::BadRequest, body)
	}
	
	#[inline(always)]
	fn invalid_request_uri(isHead: bool) -> Self
	{
		Self::bad_request(isHead, "<!doctype html><title>Bad Request</title><p>Invalid Request-URI.")
	}
	
	#[inline(always)]
	fn http11_missing_host_header(isHead: bool) -> Self
	{
		Self::bad_request(isHead, "<!doctype html><title>Bad Request</title><p>Missing HTTP/1.1 or later Host header.")
	}
	
	#[inline(always)]
	fn unsupported_http_version(isHead: bool) -> Self
	{
		Self::static_html_response(isHead, StatusCode::HttpVersionNotSupported, "<!doctype html><title>HTTP Version Not Supported</title><p>Only HTTP/1.1 and H2 over TLS are supported.")
	}
	
	#[inline(always)]
	fn asterisk_form_request_uri_is_only_allowed_for_OPTIONS_method(isHead: bool) -> Self
	{
		Self::invalid_request_uri(isHead)
	}
	
	#[inline(always)]
	fn authority_form_request_uri_is_only_allowed_for_CONNECT_method(isHead: bool) -> Self
	{
		Self::invalid_request_uri(isHead)
	}
	
	#[inline(always)]
	fn path_is_not_validly_encoded(isHead: bool) -> Self
	{
		Self::invalid_request_uri(isHead)
	}
	
	#[inline(always)]
	fn query_is_not_validly_encoded(isHead: bool) -> Self
	{
		Self::invalid_request_uri(isHead)
	}
	
	#[inline(always)]
	fn unknown_or_unsupported_scheme_for_absolute_uri(isHead: bool) -> Self
	{
		Self::misdirected_request(isHead)
	}
	
	#[inline(always)]
	fn authority_port_is_not_ours(isHead: bool) -> Self
	{
		Self::misdirected_request(isHead)
	}
	
	#[inline(always)]
	fn authority_server_is_not_one_of_ours(isHead: bool) -> Self
	{
		Self::misdirected_request(isHead)
	}
	
	fn SOMETHING_FOR_NOW(isHead: bool) -> Self
	{
		Self::static_html_response(isHead, StatusCode::Ok, "<!doctype html><html><head><title>Options</title></head><body><p>Hello World</p></body></html>")
	}
}
