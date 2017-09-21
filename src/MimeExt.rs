// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait MimeExt: Sized
{
	#[inline(always)]
	fn withoutParameters(&self) -> Self;
	
	#[inline(always)]
	fn characterSet<'a>(&'a self) -> Option<Name<'a>>;
}

impl MimeExt for Mime
{
	#[inline(always)]
	fn withoutParameters(&self) -> Self
	{
		let type_ = self.type_();
		let subtype = self.subtype();
		
		let mimeString = if let Some(suffix) = self.suffix()
		{
			format!("{}/{}+{}", type_, subtype, suffix)
		}
		else
		{
			format!("{}/{}", type_, subtype)
		};
		mimeString.parse().unwrap()
	}
	
	#[inline(always)]
	fn characterSet<'a>(&'a self) -> Option<Name<'a>>
	{
		match self.get_param(mime::CHARSET)
		{
			None => None,
			Some(value) => Some(value)
		}
	}
}
