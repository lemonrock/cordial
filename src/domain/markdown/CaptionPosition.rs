// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


enum CaptionPosition
{
	top,
	bottom,
	none,
}

impl Default for CaptionPosition
{
	#[inline(always)]
	fn default() -> Self
	{
		CaptionPosition::bottom
	}
}

impl CaptionPosition
{
	#[inline(always)]
	fn parse(value: &str) -> Result<Self, CordialError>
	{
		match value
		{
			"top" => Ok(CaptionPosition::top),
			"bottom" => Ok(CaptionPosition::bottom),
			"none" => Ok(CaptionPosition::none),
			_ => Err(CordialError::Configuration(format!("The caption position '{}' is not valid", value))),
		}
	}
}
