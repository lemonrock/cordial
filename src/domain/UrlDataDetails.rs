// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum UrlDataDetails
{
	Generic
	{
		size: u64,
	},
	
	Image
	{
		width: u32,
		height: u32,
		size: u64,
	},

	Video
	{
		width: u32,
		height: u32,
		size: u64,
	}
}

impl UrlDataDetails
{
	#[inline(always)]
	pub(crate) fn generic(body: &[u8]) -> Self
	{
		UrlDataDetails::Generic
		{
			size: body.len() as u64,
		}
	}
	
	#[inline(always)]
	pub(crate) fn video(body: &[u8], width: u32, height: u32) -> Self
	{
		UrlDataDetails::Video
		{
			width,
			height,
			size: body.len() as u64,
		}
	}
	
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
	
	#[inline(always)]
	fn size(&self) -> u64
	{
		use self::UrlDataDetails::*;
		
		match *self
		{
			Generic { size } => size,
			Image { size, .. } => size,
			Video { size, .. } => size,
		}
	}
}
