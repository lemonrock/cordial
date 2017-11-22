// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum UrlDataDetails
{
	Empty,
	
	Image
	{
		width: u32,
		height: u32,
		size: u64,
	}
}

impl Default for UrlDataDetails
{
	#[inline(always)]
	fn default() -> Self
	{
		UrlDataDetails::Empty
	}
}

impl UrlDataDetails
{
	#[inline(always)]
	fn dimensions(&self) -> Result<(u32, u32), CordialError>
	{
		match *self
		{
			UrlDataDetails::Image { width, height, .. } => Ok((width, height)),
			
			_ => Err(CordialError::Configuration("Not an image".to_owned()))
		}
	}
	
	#[inline(always)]
	fn image(&self) -> Result<(u32, u32, u64), CordialError>
	{
		match *self
		{
			UrlDataDetails::Image { width, height, size } => Ok((width, height, size)),
			
			_ => Err(CordialError::Configuration("Not an image".to_owned()))
		}
	}
}
