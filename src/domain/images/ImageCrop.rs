// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ImageCrop
{
	x: u32,
	y: u32,
	width: u32,
	height: u32,
}

impl ImageCrop
{
	#[inline(always)]
	fn crop(&self, image: &mut ::image::DynamicImage) -> ::image::DynamicImage
	{
		image.crop(self.x, self.y, self.width, self.height)
	}
	
	#[inline(always)]
	fn dimensionsAfterCrop(&self, dimensions: (u32, u32)) -> (u32, u32)
	{
		let (beforeWidth, beforeHeight) = dimensions;
		
		let afterWidth = if self.x >= beforeWidth
		{
			0
		}
		else
		{
			let maximumAfterWidth = beforeWidth - self.x;
			if self.width >= maximumAfterWidth
			{
				maximumAfterWidth
			}
			else
			{
				self.width
			}
		};
		
		let afterHeight = if self.y >= beforeHeight
		{
			0
		}
		else
		{
			let maximumAfterHeight = beforeHeight - self.x;
			if self.height >= maximumAfterHeight
			{
				maximumAfterHeight
			}
			else
			{
				self.height
			}
		};
		
		(afterWidth, afterHeight)
	}
}
