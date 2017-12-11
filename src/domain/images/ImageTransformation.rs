// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub(crate) enum ImageTransformation
{
	invert,
	
	resize
	{
		scale: ImageScale,
		#[serde(default)] filter: ImageTransformationFilterType,
	},
	
	resize_exact
	{
		scale: ImageScale,
		#[serde(default)] filter: ImageTransformationFilterType,
	},
	
	crop
	{
		x: u32,
		y: u32,
		width: u32,
		height: u32,
	},
	
	grayscale,
	
	blur
	{
		sigma: f32,
	},
	
	unsharpen
	{
		sigma: f32,
		threshold: i32,
	},
	
	filter3x3
	{
		kernel: [f32; 9],
	},
	
	adjust_contrast
	{
		contrast: f32
	},
	
	brighten
	{
		value: i32
	},
	
	hue_rotate
	{
		value: i32
	},
	
	flip_vertically,
	
	flip_horizontally,
	
	rotate_90_degrees,
	
	rotate_180_degrees,
	
	rotate_270_degrees,
}

impl ImageTransformation
{
	#[inline(always)]
	pub(crate) fn applyTransformations(originalImage: &::image::DynamicImage, transformations: &[Self]) -> Result<Option<::image::DynamicImage>, CordialError>
	{
		let mut previousImage = None;
		for transformation in transformations.iter()
		{
			let mut thisImage = None;
			match previousImage
			{
				None =>
				{
					thisImage = transformation.transform(originalImage)?;
				}
				
				Some(ref mut image) =>
				{
					if let Some(transformed) = transformation.transform(image)?
					{
						thisImage = Some(transformed);
					}
				},
			};
			previousImage = thisImage;
		}
		
		Ok(previousImage)
	}
	
	#[inline(always)]
	pub(crate) fn transform(&self, image: &::image::DynamicImage) -> Result<Option<::image::DynamicImage>, CordialError>
	{
		use self::ImageTransformation::*;
		
		match *self
		{
			resize { scale, filter } => scale.resize(image, filter.to_FilterType()),
			
			resize_exact { scale, filter } => scale.resizeExact(image, filter.to_FilterType()),
			
			invert =>
			{
				let mut newImage = image.clone();
				newImage.invert();
				Ok(Some(newImage))
			},
			
			crop { x, y, width, height } =>
			{
				if x == 0 && y == 0 && width == image.width() && height == image.height()
				{
					Ok(None)
				}
				else
				{
					let mut newImage = image.clone();
					newImage.crop(x, y, width, height);
					Ok(Some(newImage))
				}
			},
			
			grayscale => Ok(Some(image.grayscale())),
			
			blur { sigma } => Ok(Some(image.blur(sigma))),
			
			unsharpen { sigma, threshold } => Ok(Some(image.unsharpen(sigma, threshold))),
			
			filter3x3 { ref kernel } => Ok(Some(image.filter3x3(kernel))),
			
			adjust_contrast { contrast } => Ok(Some(image.adjust_contrast(contrast))),
			
			brighten { value } => Ok(Some(image.brighten(value))),
			
			hue_rotate { value } => Ok(Some(image.huerotate(value))),
			
			flip_vertically => Ok(Some(image.flipv())),
			
			flip_horizontally => Ok(Some(image.fliph())),
			
			rotate_90_degrees => Ok(Some(image.rotate90())),
			
			rotate_180_degrees => Ok(Some(image.rotate180())),
			
			rotate_270_degrees => Ok(Some(image.rotate270())),
		}
	}
}
