// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct StaticResponse
{
	statusCode: StatusCode,
	contentType: ContentType,
	headers: Vec<(String, String)>,
	uncompressedBody: Vec<u8>,
	gzipAndBrotliCompressedBodies: Option<(Vec<u8>, Vec<u8>)>,
}

impl StaticResponse
{
	pub(crate) fn new(statusCode: StatusCode, contentType: ContentType, headers: Vec<(String, String)>, uncompressedBody: Vec<u8>, gzipAndBrotliCompressedBodies: Option<(Vec<u8>, Vec<u8>)>) -> Self
	{
		Self
		{
			statusCode,
			contentType,
			headers,
			uncompressedBody,
			gzipAndBrotliCompressedBodies
		}
	}
	
	pub(crate) fn entityTag(&self) -> String
	{
		let mut responseHeaders = Headers::with_capacity(1 + self.headers.len());
		
		responseHeaders.set(self.contentType.clone());
		
		for &(ref name, ref value) in self.headers.iter()
		{
			responseHeaders.set_raw(name.to_owned(), value.to_owned())
		}
		
		let mut context = Context::new(&SHA256);
		context.update(format!("{}", responseHeaders).as_bytes());
		context.update(&self.uncompressedBody);
		let digest = context.finish();
		let bytes = digest.as_ref();
		
		// Unfortunately there is a slight possibility of generating a leading 'W/' with the ZeroMQ 85 (a variant of Base85 or Ascii85) encoding, which makes the ETag 'weak'
		let base85EncodedDigest = bytes.to_z85().unwrap();
		let mut entityTagSafe = String::with_capacity(1 + base85EncodedDigest.len());
		entityTagSafe.push('x');
		entityTagSafe.push_str(&base85EncodedDigest);
		entityTagSafe
	}
	
	#[inline(always)]
	fn staticResponse(&self, isHead: bool, preferredEncoding: PreferredEncoding, entityTag: &str) -> Response
	{
		let mut response = Response::common_headers(self.statusCode.clone(), self.contentType.clone());
		
		let body =
		{
			let headers = response.headers_mut();
			
			headers.set(ETag(EntityTag::new(false, entityTag.to_owned())));
			
			for &(ref name, ref value) in self.headers.iter()
			{
				headers.set_raw(name.to_owned(), value.to_owned())
			}
			
			self.body(preferredEncoding, headers)
		};
		
		let mut response = response.with_header(ContentLength(body.len() as u64));
		
		if isHead
		{
			response.set_body(Body::empty());
		}
		else
		{
			response.set_body(body.to_owned());
		}
		
		response
	}
	
	#[inline(always)]
	fn body<'a>(&'a self, preferredEncoding: PreferredEncoding, headers: &mut Headers) -> &'a [u8]
	{
		match preferredEncoding
		{
			PreferredEncoding::brotli => match self.gzipAndBrotliCompressedBodies
			{
				None => &self.uncompressedBody,
				Some((ref _gzip, ref brotli)) =>
				{
					headers.set(ContentEncoding(vec![Encoding::Brotli]));
					brotli
				},
			}
			PreferredEncoding::gzip => match self.gzipAndBrotliCompressedBodies
			{
				None => &self.uncompressedBody,
				Some((ref gzip, ref _brotli)) =>
				{
					headers.set(ContentEncoding(vec![Encoding::Gzip]));
					gzip
				}
			}
			PreferredEncoding::uncompressed => &self.uncompressedBody,
		}
	}
}
