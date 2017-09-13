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
		
		let base85EncodedDigest = bytes.to_z85().unwrap();
		let mut entityTag = String::with_capacity(base85EncodedDigest.len());
		entityTag.push_str(&base85EncodedDigest);
		entityTag
	}
	
	#[inline(always)]
	fn staticResponse(&self, isHead: bool, preferredEncoding: PreferredEncoding, entityTag: &str, lastModified: HttpDate, ifMatch: Option<IfMatch>, ifUnmodifiedSince: Option<IfUnmodifiedSince>, ifNoneMatch: Option<IfNoneMatch>, ifModifiedSince: Option<IfModifiedSince>, ifRange: Option<IfRange>, range: Option<Range>) -> Response
	{
		// Order of evaluation: https://tools.ietf.org/html/rfc7232#section-6
		
		if let Some(ifMatch) = ifMatch
		{
			use self::IfMatch::*;
			let isTrueCondition = match ifMatch
			{
				Any => true,
				Items(ref entityTags) =>
				{
					for providedEntityTag in entityTags
					{
						// Must use strong comparison function
						if providedEntityTag.weak
						{
							continue
						}
						if providedEntityTag.tag() == entityTag
						{
							break true;
						}
					}
					false
				}
			};
			if !isTrueCondition
			{
				return Response::precondition_failed(isHead, entityTag, lastModified);
			}
		}
		
		if let Some(ifUnmodifiedSince) = ifUnmodifiedSince
		{
			if lastModified > ifUnmodifiedSince
			{
				return Response::precondition_failed(isHead, entityTag, lastModified);
			}
		}
		
		if let Some(ifNoneMatch) = ifMatch
		{
			use self::IfMatch::*;
			let isTrueCondition = match ifNoneMatch
			{
				Any => false,
				Items(ref entityTags) =>
				{
					for providedEntityTag in entityTags
					{
						// Must use weak comparison function
						if providedEntityTag.tag() == entityTag
						{
							break false;
						}
					}
					true
				}
			};
			if !isTrueCondition
			{
				// Response::precondition_failed for methods other than HEAD & GET
				return Response::not_modified(entityTag, lastModified, &self.headers);
			}
		}
		
		// Only relevant for HEAD & GET
		if let Some(ifModifiedSince) = ifModifiedSince
		{
			if lastModified <= ifModifiedSince
			{
				return Response::not_modified(entityTag, lastModified, &self.headers);
			}
		}
		
		let isGet = !isHead;
		if isGet
		{
			if let Some(ifRange) = ifRange
			{
				use self::IfRange::*;
				let isTrueCondition = match ifRange
				{
					// Only strong comparisons are allowed; a weak comparison should result in a Bad Request, but we are lenient
					IfRange::EntityTag(ref providedEntityTag) => if entityTag.weak
					{
						false
					}
					else
					{
						providedEntityTag == entityTag
					},
					IfRange::Date(date) => date == lastModified,
				};
				
				if isTrueCondition
				{
					// A missing Range header when If-Range is present should result in a Bad Request, but we are lenient
					if let Some(range) = range
					{
						return self.respondToRangeRequest(range);
					}
				}
			}
			
			if let Some(range) = range
			{
				return self.respondToRangeRequest(range);
			}
		}
				
		let mut response = Response::common_headers(self.statusCode.clone(), self.contentType.clone());
		
		let body =
		{
			let headers = response.headers_mut();
			
			headers.set(ETag(EntityTag::strong(entityTag.to_owned())));
			headers.set(LastModified(lastModified));
			headers.set(AcceptRanges(vec![RangeUnit::Bytes]));
			
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
	
	#[inline(always)]
	fn respondToRangeRequest(&self, range: Range) -> Response
	{
		match range
		{
			Unregistered(..) => return self.range_not_satisfiable(),
			Range::Bytes(ref byteRanges) =>
			{
				if byteRanges.is_empty()
				{
					return self.range_not_satisfiable();
				}
				else if byteRanges.len() == 1
				{
					let byteRangeSpec = byteRanges.get(0).unwrap();
					match self.bodyByteRange(byteRangeSpec)
					{
						None => return self.range_not_satisfiable(),
						Some((fromInclusive, toExclusive)) =>
						{
							let contentFragment = &self.uncompressedBody[fromInclusive .. toExclusive];
							return Response::single_part_partial_content(true, &self.contentType, entityTag, lastModified, &self.headers, self.body.len(), fromInclusive, toExclusive, contentFragment);
						}
					}
				}
				else if byteRanges.len() < 6
				{
					let mut rangeOverlapChecks = BTreeMap::new();
					let mut multipartParts = Vec::with_capacity(byteRanges.len());
					let mut approximateCapacityForBody = 0;
					
					for byteRangeSpec in byteRanges.iter()
					{
						match self.bodyByteRange(byteRangeSpec)
						{
							None => return self.range_not_satisfiable(),
							Some((fromInclusive, toExclusive)) =>
							{
								let toInclusive = toExclusive - 1;
								
								// Do we overlap with lower range?
								if let Some((previousFromInclusive, previousToInclusive)) = rangeOverlapChecks.range(0 .. fromInclusive).next_back()
								{
									if previousToInclusive >= toInclusive
									{
										return self.range_not_satisfiable();
									}
								}
								
								// Do we overlap with higher range?
								if rangeOverlapChecks.range(fromInclusive .. toExclusive).next().is_some()
								{
									return self.range_not_satisfiable();
								}
								
								// OK
								rangeOverlapChecks.insert(fromInclusive, toInclusive);
								
								let mut headers = Headers::with_capacity(2);
								headers.set(&self.contentType.clone());
								headers.set(ContentRange(ContentRangeSpec::Bytes
								{
									range: Some(fromInclusive as u64, toInclusive as u64),
									instance_length: Some(fromInclusive - toInclusive),
								}));
								multipartParts.push(Node::Part
								{
									headers,
									body: self.uncompressedBody[fromInclusive .. toExclusive].clone(),
								});
								
								const CostOfARange: usize = 256;
								approximateCapacityForBody += (toExclusive - fromInclusive + CostOfARange);
							}
						}
					}
					
					let mut responseBody = Vec::with_capacity(approximateCapacityForBody);
					let mimeMultipartBoundary = generate_boundary();
					write_multipart(&mut responseBody, &mimeMultipartBoundary, &multipartParts).expect("Should not fail");
					
					return Response::multi_part_partial_content(false, entityTag, lastModified, &self.headers, responseBody, boundary);
				}
				else
				{
					return self.range_not_satisfiable();
				}
			}
		}
	}
	
	// It is valid to return this without compression UNLESS the client explicitly set Accept-Encoding with identity;q=0 - which is daft...
	// None => Bad range
	// Resultant ranges can be empty - is that invalid?
	#[inline(always)]
	fn bodyByteRange<'a>(&'a self, byteRangeSpec: &ByteRangeSpec) -> Option<(usize, usize)>
	{
		let contentLength = self.uncompressedBody.len() as u64;
		let range = match *byteRangeSpec
		{
			ByteRangeSpec::FromTo(fromInclusive, toInclusive) =>
			{
				if fromInclusive > toInclusive
				{
					return None;
				}
				
				if fromInclusive >= contentLength
				{
					return None;
				}
				
				let toExclusive = if toInclusive >= contentLength
				{
					contentLength
				}
				else
				{
					if toInclusive == u64::max_value()
					{
						return None;
					}
					toInclusive + 1
				};
				
				#[cfg(target_pointer_width = "32")]
				{
					if fromInclusive > usize::max_value() as u64 || toInclusive > usize::max_value() as u64
					{
						return None;
					}
				}
				
				(fromInclusive as usize, toExclusive as usize)
			},
			ByteRangeSpec::AllFrom(fromInclusive) =>
			{
				if fromInclusive >= contentLength
				{
					return None;
				}
				
				#[cfg(target_pointer_width = "32")]
				{
					if fromInclusive > usize::max_value() as u64
					{
						return None;
					}
				}
				
				(fromInclusive as usize, contentLength as usize)
			},
			ByteRangeSpec::Last(length) =>
			{
				if length == 0
				{
					return None;
				}
				
				#[cfg(target_pointer_width = "32")]
				{
					if length > usize::max_value() as u64
					{
						return None;
					}
				}
				
				if length >= contentLength
				{
					(0, contentLength)
					&self.uncompressedBody
				}
				else
				{
					((contentLength - length) as usize, contentLength as usize)
				}
			}
		};
		Some(range)
	}
	
	#[inline(always)]
	fn range_not_satisfiable(&self) -> Response
	{
		Response::range_not_satisfiable(self.uncompressedBody.len())
	}
}
