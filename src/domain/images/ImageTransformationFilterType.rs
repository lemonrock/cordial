// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum ImageTransformationFilterType
{
	nearest_neighbour,
	triangle_linear,
	catmull_rom_cubic,
	gaussian,
	lanczos_with_window_3,
}

impl Default for ImageTransformationFilterType
{
	#[inline(always)]
	fn default() -> Self
	{
		ImageTransformationFilterType::lanczos_with_window_3
	}
}

impl ImageTransformationFilterType
{
	pub(crate) fn to_FilterType(&self) -> ::image::FilterType
	{
		match *self
		{
			ImageTransformationFilterType::nearest_neighbour => ::image::FilterType::Nearest,
			ImageTransformationFilterType::triangle_linear => ::image::FilterType::Triangle,
			ImageTransformationFilterType::catmull_rom_cubic => ::image::FilterType::CatmullRom,
			ImageTransformationFilterType::gaussian => ::image::FilterType::Gaussian,
			ImageTransformationFilterType::lanczos_with_window_3 => ::image::FilterType::Lanczos3,
		}
	}
}
