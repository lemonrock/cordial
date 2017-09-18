// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct LengthTrackingWriter<'a>
{
	bytes: Vec<u8>,
	bytesWritten: &'a mut usize,
}

impl<'a> Write for LengthTrackingWriter<'a>
{
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>
	{
		let result = self.bytes.write(buf);
		match result
		{
			Err(error) => Err(error),
			Ok(bytesWritten) =>
			{
				*self.bytesWritten += bytesWritten;
				Ok(bytesWritten)
			}
		}
	}
	
	#[inline(always)]
	fn flush(&mut self) -> io::Result<()>
	{
		self.bytes.flush()
	}
}

impl<'a> LengthTrackingWriter<'a>
{
	#[inline(always)]
	pub(crate) fn new(bytesWritten: &'a mut usize) -> Self
	{
		Self
		{
			bytes: Vec::with_capacity(64 * 1024),
			bytesWritten
		}
	}
	
	#[inline(always)]
	pub(crate) fn bytes(self) -> Vec<u8>
	{
		self.bytes
	}
}
