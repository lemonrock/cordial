// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


trait CommonResponses: Sized
{
	#[inline(always)]
	fn common_headers(statusCode: StatusCode, contentType: ContentType) -> Self;
	
	#[inline(always)]
	fn static_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, contentType: ContentType, body: I) -> Self;
	
	#[inline(always)]
	fn static_txt_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, body: I) -> Self;
	
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
	fn old_temporary_redirect(isHead: bool, url: &Url) -> Self;
	
	#[inline(always)]
	fn precondition_failed(isHead: bool, entityTag: &str, lastModified: HttpDate) -> Self;
	
	#[inline(always)]
	fn not_found(isHead: bool) -> Self;
	
	#[inline(always)]
	fn range_not_satisfiable(contentLength: u64) -> Self;
	
	#[inline(always)]
	fn single_part_partial_content(isInResponseToIfRange: bool, contentType: &ContentType, entityTag: &str, lastModified: HttpDate, headers: &[(String, String)], fullBodyLength: usize, fromInclusive: usize, toExclusive: usize, contentFragment: &[u8]) -> Self;
	
	#[inline(always)]
	fn multi_part_partial_content(isInResponseToIfRange: bool, entityTag: &str, lastModified: HttpDate, headers: &[(String, String)], body: Vec<u8>, boundary: Vec<u8>) -> Self;
	
	#[inline(always)]
	fn not_modified(entityTag: &str, lastModified: HttpDate, headers: &[(String, String)]) -> Self;
	
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
}

impl CommonResponses for Response
{
	#[inline(always)]
	fn common_headers(statusCode: StatusCode, contentType: ContentType) -> Self
	{
		Response::new()
		.with_status(statusCode)
		.with_header(contentType)
		.with_header(X_XSS_Protection::Default)
		.with_header(X_Content_Type_Options::Default)
		.with_header(X_Frame_Options::Default)
		.with_header(Date(SystemTime::now().into()))
	}
	
	#[inline(always)]
	fn static_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, contentType: ContentType, body: I) -> Self
	{
		let body = body.into();
		let response = Self::common_headers(statusCode, contentType).with_header(ContentLength(body.len() as u64));
		
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
	fn static_txt_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, body: I) -> Self
	{
		Self::static_response(isHead, statusCode, ContentType::plaintext(), body)
	}
	
	#[inline(always)]
	fn static_html_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, body: I) -> Self
	{
		Self::static_response(isHead, statusCode, ContentType::html(), body)
	}
	
	#[inline(always)]
	fn options(permittedMethods: Vec<Method>) -> Self
	{
		Self::static_txt_response(false, StatusCode::Ok, "")
		.with_header(commonCacheControlHeader(60))
		.with_header(Allow(permittedMethods))
	}
	
	#[inline(always)]
	fn method_not_allowed(permittedMethods: Vec<Method>) -> Self
	{
		Self::static_txt_response(false, StatusCode::MethodNotAllowed, "")
		.with_header(commonCacheControlHeader(60))
		.with_header(Allow(permittedMethods))
	}
	
	#[inline(always)]
	fn misdirected_request(isHead: bool) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::MisdirectedRequest, "")
		.with_header(commonCacheControlHeader(60))
	}
	
	#[inline(always)]
	fn old_permanent_redirect(isHead: bool, url: &Url) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::MovedPermanently, "")
		.with_header(commonCacheControlHeader(31536000))
		.with_header(Location::new(url.as_ref().to_owned()))
	}
	
	#[inline(always)]
	fn old_temporary_redirect(isHead: bool, url: &Url) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::Found, "")
		.with_header(commonCacheControlHeader(60))
		.with_header(Location::new(url.as_ref().to_owned()))
	}
	
	#[inline(always)]
	fn precondition_failed(isHead: bool, entityTag: &str, lastModified: HttpDate) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::PreconditionFailed, "")
		.with_header(ETag(EntityTag::strong(entityTag.to_owned())))
		.with_header(LastModified(lastModified))
	}
	
	#[inline(always)]
	fn not_modified(entityTag: &str, lastModified: HttpDate, headers: &[(String, String)]) -> Self
	{
		let mut response = Response::new()
		.with_status(StatusCode::NotModified)
		.with_header(Date(SystemTime::now().into()))
		.with_header(ETag(EntityTag::strong(entityTag.to_owned())))
		.with_header(LastModified(lastModified));
		
		{
			let responseHeaders = response.headers_mut();
			
			for &(ref headerName, ref headerValue) in headers.iter()
			{
				match headerName.to_ascii_lowercase().as_str()
				{
					"cache-control" | "vary" => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
					"content-location" | "date" | "etag" | "expires" => responseHeaders.set_raw(headerName.to_owned(), headerValue.to_owned()),
					_ => (),
				}
			}
		}
		
		response
	}
	
	#[inline(always)]
	fn not_found(isHead: bool) -> Self
	{
		Self::static_html_response(isHead, StatusCode::NotFound, "<!doctype html><title>Not found</title><p>The document has not found here.".to_owned())
		.with_header(commonCacheControlHeader(60))
	}
	
	#[inline(always)]
	fn range_not_satisfiable(contentLength: u64) -> Self
	{
		Response::new()
		.with_status(StatusCode::RangeNotSatisfiable)
		.with_header(Date(SystemTime::now().into()))
		.with_header(ContentRange(ContentRangeSpec::Bytes
		{
			range: None,
			instance_length: Some(contentLength),
		}))
	}
	
	#[inline(always)]
	fn single_part_partial_content(isInResponseToIfRange: bool, contentType: &ContentType, entityTag: &str, lastModified: HttpDate, headers: &[(String, String)], fullBodyLength: usize, fromInclusive: usize, toExclusive: usize, contentFragment: &[u8]) -> Self
	{
		let mut response = Response::new()
		.with_status(StatusCode::PartialContent)
		.with_header(Date(SystemTime::now().into()))
		.with_header(ETag(EntityTag::strong(entityTag.to_owned())))
		.with_header(LastModified(lastModified))
		.with_header(ContentLength(contentFragment.len() as u64))
		.with_header(contentType.clone())
		.with_header(ContentRange(ContentRangeSpec::Bytes
		{
			range: Some((fromInclusive as u64, (toExclusive - 1) as u64)),
			instance_length: Some(fullBodyLength as u64),
		}));
		
		if isInResponseToIfRange
		{
			let responseHeaders = response.headers_mut();
			
			for &(ref headerName, ref headerValue) in headers.iter()
			{
				match headerName.to_ascii_lowercase().as_str()
				{
					"cache-control" | "vary" => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
					"content-location" | "date" | "etag" | "expires" => responseHeaders.set_raw(headerName.to_owned(), headerValue.to_owned()),
					_ => (),
				}
			}
		}
		else
		{
			let responseHeaders = response.headers_mut();
			
			responseHeaders.set(AcceptRanges(vec![RangeUnit::Bytes]));
			
			for &(ref headerName, ref headerValue) in headers.iter()
			{
				match headerName.to_ascii_lowercase().as_str()
				{
					"cache-control" | "vary" => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
					"content-location" | "date" | "etag" | "expires" => responseHeaders.set_raw(headerName.to_owned(), headerValue.to_owned()),
					_ => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
				}
			}
		}
		
		response.with_body(contentFragment.to_vec())
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn multi_part_partial_content(isInResponseToIfRange: bool, entityTag: &str, lastModified: HttpDate, headers: &[(String, String)], body: Vec<u8>, boundary: Vec<u8>) -> Self
	{
		let mimeType = format!("multipart/byteranges; boundary={}", unsafe { String::from_utf8_unchecked(boundary) }).parse().unwrap();
		
		let mut response = Response::new()
		.with_status(StatusCode::PartialContent)
		.with_header(Date(SystemTime::now().into()))
		.with_header(ETag(EntityTag::strong(entityTag.to_owned())))
		.with_header(LastModified(lastModified))
		.with_header(ContentLength(body.len() as u64))
		.with_header(ContentType(mimeType));
		
		if isInResponseToIfRange
		{
			let responseHeaders = response.headers_mut();
			
			for &(ref headerName, ref headerValue) in headers.iter()
			{
				match headerName.to_ascii_lowercase().as_str()
				{
					"cache-control" | "vary" => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
					"content-location" | "date" | "etag" | "expires" => responseHeaders.set_raw(headerName.to_owned(), headerValue.to_owned()),
					_ => (),
				}
			}
		}
		else
		{
			let responseHeaders = response.headers_mut();
			
			responseHeaders.set(AcceptRanges(vec![RangeUnit::Bytes]));
			
			for &(ref headerName, ref headerValue) in headers.iter()
			{
				match headerName.to_ascii_lowercase().as_str()
				{
					"cache-control" | "vary" => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
					"content-location" | "date" | "etag" | "expires" => responseHeaders.set_raw(headerName.to_owned(), headerValue.to_owned()),
					_ => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
				}
			}
		}
		
		response.with_body(body)
	}
	
	// Bad Request gets displayed to the end user
	#[inline(always)]
	fn bad_request<I: Into<Cow<'static, str>>>(isHead: bool, body: I) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::BadRequest, body)
	}
	
	#[inline(always)]
	fn invalid_request_uri(isHead: bool) -> Self
	{
		Self::bad_request(isHead, "")
	}
	
	#[inline(always)]
	fn http11_missing_host_header(isHead: bool) -> Self
	{
		Self::bad_request(isHead, "")
	}
	
	#[inline(always)]
	fn unsupported_http_version(isHead: bool) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::HttpVersionNotSupported, "Only HTTP/1.1 and H2 over TLS are supported")
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
}
