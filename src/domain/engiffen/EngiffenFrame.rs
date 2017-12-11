// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct EngiffenFrame
{
	#[serde(default)] transformations: Vec<ImageTransformation>,
	#[serde(default = "EngiffenFrame::frame_delay_in_tens_of_milliseconds_default")] frame_delay_in_tens_of_milliseconds: u16,
	#[serde(default)] needs_user_input: bool,
	#[serde(default)] interlaced: bool,
	#[serde(default)] top_offset: u16,
	#[serde(default)] left_offset: u16,
	#[serde(default)] frame_disposal: EngiffenDisposal,
}

impl Default for EngiffenFrame
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			transformations: Vec::default(),
			frame_delay_in_tens_of_milliseconds: Self::frame_delay_in_tens_of_milliseconds_default(),
			needs_user_input: false,
			interlaced: false,
			top_offset: 0,
			left_offset: 0,
			frame_disposal: EngiffenDisposal::default(),
		}
	}
}

impl EngiffenFrame
{
	#[inline(always)]
	fn transform(&self, image: &mut EngiffenSourceImage) -> Result<Option<EngiffenSourceImage>, CordialError>
	{
		ImageTransformation::applyTransformations(image, &self.transformations[..])
	}
	
	#[inline(always)]
	fn modify(&self, frame: &mut ::gif::Frame)
	{
		frame.delay = self.frame_delay_in_tens_of_milliseconds;
		frame.needs_user_input = self.needs_user_input;
		frame.top = self.top_offset;
		frame.left = self.left_offset;
		frame.dispose = self.frame_disposal.toDisposalMethod();
	}
	
	#[inline(always)]
	fn modifyForPlaceholder(&self, frame: &mut ::gif::Frame)
	{
		frame.top = self.top_offset;
		frame.left = self.left_offset;
	}
	
	#[inline(always)]
	fn frame_delay_in_tens_of_milliseconds_default() -> u16
	{
		const TwentyFiveFramesPerSecond: u16 = 4;
		
		TwentyFiveFramesPerSecond
	}
}
