// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug)]
	pub enum WoffError
	{
		OutOfMemory
		{
			description("Out of memory because malloc() or realloc() failed")
			display("Out of memory")
		}
		
		InvalidInput
		{
			description("Invalid input (eg bad offset in data)")
			display("Invalid input")
		}
		
		CompressionFailure
		{
			description("Compression failure in zopfli call")
			display("Compression failure")
		}
		
		UnrecognizedFileSignature
		{
			description("Unrecognized file signature")
			display("Unrecognized file signature")
		}
		
		ProvidedBufferTooSmall
		{
			description("Provided buffer too small")
			display("Provided buffer too small")
		}
		
		InvalidParameter
		{
			description("Invalid parameter (eg null source parameter)")
			display("Invalid parameter")
		}
		
		ImproperlyOrderedChunksInWoffData
		{
			description("Improperly ordered chunks in WOFF data")
			display("Improperly ordered chunks in WOFF data")
		}
	}
}

impl WoffError
{
	#[inline(always)]
	fn process(status: u32) -> Self
	{
		use self::WoffError::*;
		match status
		{
			0 => panic!("Status should not be zero (0), OK"),
			1 => OutOfMemory,
			2 => InvalidInput,
			3 => CompressionFailure,
			4 => UnrecognizedFileSignature,
			5 => ProvidedBufferTooSmall,
			6 => InvalidParameter,
			7 => ImproperlyOrderedChunksInWoffData,
			_ => panic!("Unexpected status '{}'", status)
		}
	}
}
