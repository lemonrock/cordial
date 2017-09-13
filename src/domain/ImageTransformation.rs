// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub(crate) enum ImageTransformation
{
	crop
	{
		x: u32,
		y: u32,
		width: u32,
		height: u32,
	},
	grayscale,
	invert,
	resize
	{
		scale: ImageScale,
		filter: TransformFilterType,
	},
	resize_exact
	{
		scale: ImageScale,
		filter: TransformFilterType,
	},
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
	pub(crate) fn applyTransformations(mut image: ::image::DynamicImage, transformations: &[Self]) -> Result<::image::DynamicImage, CordialError>
	{
		for transformation in transformations
		{
			image = transformation.transform(image)?;
		}
		Ok(image)
	}
	
	#[inline(always)]
	pub(crate) fn transform(&self, mut image: ::image::DynamicImage) -> Result<::image::DynamicImage, CordialError>
	{
		use self::ImageTransformation::*;
		let image = match *self
		{
			crop { x, y, width, height } => image.crop(x, y, width, height),
			grayscale => image.grayscale(),
			invert => { image.invert(); image },
			resize { scale, filter } => scale.resize(image, filter.to_FilterType())?,
			resize_exact { scale, filter } => scale.resizeExact(image, filter.to_FilterType())?,
			blur { sigma } => image.blur(sigma),
			unsharpen { sigma, threshold } => image.unsharpen(sigma, threshold),
			filter3x3 { ref kernel } => image.filter3x3(kernel),
			adjust_contrast { contrast } => image.adjust_contrast(contrast),
			brighten { value } => image.brighten(value),
			hue_rotate { value } => image.huerotate(value),
			flip_vertically => image.flipv(),
			flip_horizontally => image.fliph(),
			rotate_90_degrees => image.rotate90(),
			rotate_180_degrees => image.rotate180(),
			rotate_270_degrees => image.rotate270(),
		};
		Ok(image)
	}
}
