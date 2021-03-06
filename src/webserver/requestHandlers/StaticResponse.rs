// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct StaticResponse
{
	#[serde(with = "::serde_with::StatusCodeSerde")] statusCode: StatusCode,
	#[serde(with = "::serde_with::ContentTypeSerde")] contentType: ContentType,
	headers: ResponseHeaders,
	uncompressedBody: ResponseBody,
	gzipAndBrotliCompressedBodies: Option<(BinaryBody, BinaryBody)>,
}

impl StaticResponse
{
	pub(crate) fn new(statusCode: StatusCode, contentType: ContentType, headers: ResponseHeaders, uncompressedBody: ResponseBody, gzipAndBrotliCompressedBodies: Option<(BinaryBody, BinaryBody)>) -> Self
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
	fn respondAssumingResourceIs200Ok(&self, isHead: bool, preferredEncoding: PreferredCompression, entityTag: &str, lastModified: HttpDate, ifMatch: Option<&IfMatch>, ifUnmodifiedSince: Option<&IfUnmodifiedSince>, ifNoneMatch: Option<&IfNoneMatch>, ifModifiedSince: Option<&IfModifiedSince>, ifRange: Option<&IfRange>, range: Option<&Range>) -> Response
	{
		// Order of evaluation: https://tools.ietf.org/html/rfc7232#section-6
		
		if let Some(ifMatch) = ifMatch
		{
			let isTrueCondition = match *ifMatch
			{
				IfMatch::Any => true,
				IfMatch::Items(ref entityTags) =>
				{
					let mut result = false;
					for providedEntityTag in entityTags
					{
						// Must use strong comparison function
						if providedEntityTag.weak
						{
							continue
						}
						if providedEntityTag.tag() == entityTag
						{
							result = true;
							break;
						}
					}
					result
				}
			};
			if !isTrueCondition
			{
				return Response::precondition_failed(isHead, entityTag, lastModified);
			}
		}
		
		if let Some(ifUnmodifiedSince) = ifUnmodifiedSince
		{
			if lastModified > ifUnmodifiedSince.0
			{
				return Response::precondition_failed(isHead, entityTag, lastModified);
			}
		}
		
		if let Some(ifNoneMatch) = ifNoneMatch
		{
			let isTrueCondition = match *ifNoneMatch
			{
				IfNoneMatch::Any => false,
				IfNoneMatch::Items(ref entityTags) =>
				{
					let mut result = true;
					for providedEntityTag in entityTags
					{
						// Must use weak comparison function
						if providedEntityTag.tag() == entityTag
						{
							result = false;
							break;
						}
					}
					result
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
			if lastModified <= ifModifiedSince.0
			{
				return Response::not_modified(entityTag, lastModified, &self.headers);
			}
		}
		
		let isGet = !isHead;
		if isGet
		{
			if let Some(ifRange) = ifRange
			{
				let isTrueCondition = match *ifRange
				{
					// Only strong comparisons are allowed; a weak comparison should result in a Bad Request, but we are lenient
					IfRange::EntityTag(ref providedEntityTag) => if providedEntityTag.weak
					{
						false
					}
					else
					{
						providedEntityTag.tag() == entityTag
					},
					IfRange::Date(date) => date == lastModified,
				};
				
				if isTrueCondition
				{
					// A missing Range header when If-Range is present should result in a Bad Request, but we are lenient
					if let Some(range) = range
					{
						return self.respondToRangeRequest(range, entityTag, lastModified);
					}
				}
			}
			
			if let Some(range) = range
			{
				return self.respondToRangeRequest(range, entityTag, lastModified);
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
	fn rawResponse(&self, isHead: bool) -> Response
	{
		let response = Response::common_headers(self.statusCode.clone(), self.contentType.clone());
		let mut response = response.with_header(ContentLength(self.uncompressedBody.len() as u64));
		
		if isHead
		{
			response.set_body(Body::empty());
		}
		else
		{
			response.set_body(self.uncompressedBody.deref().to_owned());
		}
		
		response
	}
	
	#[inline(always)]
	fn body<'a>(&'a self, preferredEncoding: PreferredCompression, headers: &mut Headers) -> &'a [u8]
	{
		match preferredEncoding
		{
			PreferredCompression::brotli => match self.gzipAndBrotliCompressedBodies
			{
				None => &self.uncompressedBody,
				Some((ref _gzip, ref brotli)) =>
				{
					headers.set(ContentEncoding(vec![Encoding::Brotli]));
					brotli
				},
			}
			PreferredCompression::gzip => match self.gzipAndBrotliCompressedBodies
			{
				None => &self.uncompressedBody,
				Some((ref gzip, ref _brotli)) =>
				{
					headers.set(ContentEncoding(vec![Encoding::Gzip]));
					gzip
				}
			}
			PreferredCompression::uncompressed => &self.uncompressedBody,
		}
	}
	
	#[inline(always)]
	fn respondToRangeRequest(&self, range: &Range, entityTag: &str, lastModified: HttpDate) -> Response
	{
		match *range
		{
			Range::Unregistered(..) => return self.range_not_satisfiable(),
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
							return Response::single_part_partial_content(true, &self.contentType, entityTag, lastModified, &self.headers, self.uncompressedBody.len(), fromInclusive, toExclusive, contentFragment);
						}
					}
				}
				else if byteRanges.len() < 6
				{
					let mut rangeOverlapChecks: BTreeMap<usize, usize> = BTreeMap::new();
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
								if let Some((_previousFromInclusive, previousToInclusive)) = rangeOverlapChecks.range(0 .. fromInclusive).next_back()
								{
									if *previousToInclusive >= toInclusive
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
								headers.set(self.contentType.clone());
								headers.set(ContentRange(ContentRangeSpec::Bytes
								{
									range: Some((fromInclusive as u64, toInclusive as u64)),
									instance_length: Some((fromInclusive - toInclusive) as u64),
								}));
								multipartParts.push(Node::Part(Part
								{
									headers,
									body: self.uncompressedBody[fromInclusive .. toExclusive].to_vec(),
								}));
								
								const CostOfARange: usize = 256;
								approximateCapacityForBody += toExclusive - fromInclusive + CostOfARange;
							}
						}
					}
					
					let mut responseBody = Vec::with_capacity(approximateCapacityForBody);
					let mimeMultipartBoundary = generate_boundary();
					write_multipart(&mut responseBody, &mimeMultipartBoundary, &multipartParts).expect("Should not fail");
					
					return Response::multi_part_partial_content(false, entityTag, lastModified, &self.headers, responseBody, mimeMultipartBoundary);
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
		match *byteRangeSpec
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
				
				Some((fromInclusive as usize, toExclusive as usize))
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
				
				Some((fromInclusive as usize, contentLength as usize))
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
					Some((0usize, contentLength as usize))
				}
				else
				{
					Some(((contentLength - length) as usize, contentLength as usize))
				}
			}
		}
	}
	
	#[inline(always)]
	fn range_not_satisfiable(&self) -> Response
	{
		Response::range_not_satisfiable(self.uncompressedBody.len() as u64)
	}
}
