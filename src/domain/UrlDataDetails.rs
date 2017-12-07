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
		size: u64,
		width: u32,
		height: u32,
	},

	Video
	{
		size: u64,
		durationInSeconds: u64,
		width: u16,
		height: u16,
	},
	
	Audio
	{
		size: u64,
		durationInSeconds: u64,
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
	pub(crate) fn video(body: &[u8], width: u16, height: u16, durationInSeconds: u64) -> Self
	{
		UrlDataDetails::Video
		{
			size: body.len() as u64,
			durationInSeconds,
			width,
			height,
		}
	}
	
	#[inline(always)]
	pub(crate) fn audio(body: &[u8], durationInSeconds: u64) -> Self
	{
		UrlDataDetails::Audio
		{
			size: body.len() as u64,
			durationInSeconds,
		}
	}
	
	#[inline(always)]
	fn dimensions(&self) -> Result<(u32, u32), CordialError>
	{
		match *self
		{
			UrlDataDetails::Image { width, height, .. } => Ok((width, height)),
			UrlDataDetails::Video { width, height, .. } => Ok((width as u32, height as u32)),
			
			_ => Err(CordialError::Configuration("Not an image".to_owned()))
		}
	}
	
	#[inline(always)]
	fn image(&self) -> Result<(u32, u32, u64), CordialError>
	{
		match *self
		{
			UrlDataDetails::Image { width, height, size, .. } => Ok((width, height, size)),
			
			_ => Err(CordialError::Configuration("Not an image".to_owned()))
		}
	}
	
	#[inline(always)]
	fn durationInSeconds(&self) -> Result<u64, CordialError>
	{
		match *self
		{
			UrlDataDetails::Video { durationInSeconds, .. } => Ok(durationInSeconds),
			UrlDataDetails::Audio { durationInSeconds, .. } => Ok(durationInSeconds),
			
			_ => Err(CordialError::Configuration("Not audio or video".to_owned()))
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
			Audio { size, .. } => size,
		}
	}
	
	#[inline(always)]
	pub(crate) fn optionalVideoWidthHeight(&self) -> Option<(u16, u16)>
	{
		match *self
		{
			UrlDataDetails::Video { width, height, .. } => Some((width, height)),
			
			_ => None,
		}
	}
}
