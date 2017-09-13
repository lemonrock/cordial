// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait UrlExt: Sized
{
	#[inline(always)]
	fn fileNameOrIndexNamePercentDecodedUntrusted<'a>(&'a self, indexExtension: &'static str) -> Cow<'a, str>;
	
	#[inline(always)]
	fn fileNamePercentDecodedUntrusted<'a>(&'a self) -> Cow<'a, str>;
	
	#[inline(always)]
	fn pushReplacementFileName(self, replacementFileName: &str) -> Self;
}

impl UrlExt for Url
{
	#[inline(always)]
	fn fileNameOrIndexNamePercentDecodedUntrusted<'a>(&'a self, indexExtension: &'static str) -> Cow<'a, str>
	{
		let mut previous = None;
		let mut fileName = None;
		{
			match self.path_segments()
			{
				None => match self.host_str()
				{
					None => return Cow::Owned(format!("file{}", indexExtension)),
					Some(host) => return Cow::Owned(format!("{}{}", host, indexExtension)),
				},
				Some(pathSegments) => for pathSegment in pathSegments
				{
					previous = fileName;
					fileName = Some(pathSegment);
				}
			}
		}
		
		let percentEncodedFileName = fileName.expect("Bug in url crate");
		if percentEncodedFileName.is_empty()
		{
			match previous
			{
				None => match self.host_str()
				{
					None => return Cow::Owned(format!("file{}", indexExtension)),
					Some(host) => return Cow::Owned(format!("{}{}", host, indexExtension)),
				},
				Some(previous) => if previous.is_empty()
				{
					return Cow::Owned(format!("file{}", indexExtension))
				}
				else
				{
					let withoutFileExtension = percent_decode(previous.as_bytes()).decode_utf8_lossy();
					return Cow::Owned(format!("{}{}", withoutFileExtension, indexExtension))
				}
			}
		}
		else
		{
			percent_decode(percentEncodedFileName.as_bytes()).decode_utf8_lossy()
		}
	}
	
	#[inline(always)]
	fn fileNamePercentDecodedUntrusted<'a>(&'a self) -> Cow<'a, str>
	{
		let mut fileName = None;
		
		match self.path_segments()
		{
			None => return Cow::Borrowed(""),
			Some(pathSegments) => for pathSegment in pathSegments
			{
				fileName = Some(pathSegment);
			}
		}
		
		let percentEncodedFileName = fileName.expect("Bug in url crate");
		percent_decode(percentEncodedFileName.as_bytes()).decode_utf8_lossy()
	}
	
	#[inline(always)]
	fn pushReplacementFileName(mut self, replacementFileName: &str) -> Self
	{
		self.path_segments_mut().unwrap().pop().push(replacementFileName);
		self
	}
	
}
