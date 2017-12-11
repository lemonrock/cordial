// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ImageSourceSetEntry
{
	#[serde(default)] crop: Option<ImageCrop>,
	#[serde(default)] scale: ImageScale,
	#[serde(default)] filter: ImageTransformationFilterType
}

impl ImageSourceSetEntry
{
	#[inline(always)]
	pub(crate) fn cropAndResize(&self, image: &mut ::image::DynamicImage) -> Result<(u32, ::image::DynamicImage), CordialError>
	{
		if let Some(ref imageCrop) = self.crop
		{
			self.resizeExact(&imageCrop.crop(image))
		}
		else
		{
			self.resizeExact(image)
		}
	}
	
	#[inline(always)]
	fn resizeExact(&self, image: &::image::DynamicImage) -> Result<(u32, ::image::DynamicImage), CordialError>
	{
		let oldDimensions = image.dimensions();
		let newDimensions = self.scale.scale(oldDimensions)?;
		let newWidth = newDimensions.0;
		let newHeight = newDimensions.1;
		
		let filter = self.filter.to_FilterType();
		let resizedImage = image.resize_exact(newWidth, newHeight, filter);
		
		Ok((newWidth, resizedImage))
	}
	
	#[inline(always)]
	fn computeCroppedAndResizedImageDimensions(&self, dimensions: (u32, u32)) -> Result<(u32, u32), CordialError>
	{
		let afterCropDimensions = if let Some(ref imageCrop) = self.crop
		{
			imageCrop.dimensionsAfterCrop(dimensions)
		}
		else
		{
			dimensions
		};
		
		self.scale.scale(afterCropDimensions)
	}
}
