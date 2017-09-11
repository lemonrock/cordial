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
		width: u32,
		height: u32,
		filter: TransformFilterType,
	},
	resize_exact
	{
		width: u32,
		height: u32,
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
	pub(crate) fn applyTransformations(mut image: ::image::DynamicImage, transformations: &[Self]) -> ::image::DynamicImage
	{
		for transformation in transformations
		{
			image = transformation.transform(image);
		}
		image
	}
	
	#[inline(always)]
	pub(crate) fn transform(&self, mut image: ::image::DynamicImage) -> ::image::DynamicImage
	{
		use self::ImageTransformation::*;
		match *self
		{
			crop { x, y, width, height } => image.crop(x, y, width, height),
			grayscale => image.grayscale(),
			invert => { image.invert(); image },
			resize { width, height, filter } => image.resize(width, height, filter.to_FilterType()),
			resize_exact { width, height, filter } => image.resize_exact(width, height, filter.to_FilterType()),
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
		}
	}
}

//noinspection SpellCheckingInspection
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum TransformFilterType
{
	Nearest,
	Triangle,
	CatmullRom,
	Gaussian,
	Lanczos3,
}

impl TransformFilterType
{
	pub(crate) fn to_FilterType(&self) -> ::image::FilterType
	{
		match *self
		{
			TransformFilterType::Nearest => ::image::FilterType::Nearest,
			TransformFilterType::Triangle => ::image::FilterType::Triangle,
			TransformFilterType::CatmullRom => ::image::FilterType::CatmullRom,
			TransformFilterType::Gaussian => ::image::FilterType::Gaussian,
			TransformFilterType::Lanczos3 => ::image::FilterType::Lanczos3,
		}
	}
}
