// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub(crate) enum ImageScale
{
	Identity,
	FixedWidth
	{
		width: u32,
	},
	FixedHeight
	{
		height: u32,
	},
	FixedSize
	{
		width: u32,
		height: u32,
	},
	Ratio
	{
		upper: u32,
		lower: u32,
	},
	Fraction
	{
		ratio: f64,
	},
}

impl Default for ImageScale
{
	#[inline(always)]
	fn default() -> Self
	{
		ImageScale::Identity
	}
}

impl ImageScale
{
	#[inline(always)]
	pub(crate) fn resize(&self, image: &::image::DynamicImage, filter: ::image::FilterType) -> Result<Option<::image::DynamicImage>, CordialError>
	{
		let (width, height) = self.scale(image.dimensions())?;
		if width == image.width() && height == image.height()
		{
			Ok(None)
		}
		else
		{
			Ok(Some(image.resize(width, height, filter)))
		}
	}
	
	#[inline(always)]
	pub(crate) fn resizeExact(&self, image: &::image::DynamicImage, filter: ::image::FilterType) -> Result<Option<::image::DynamicImage>, CordialError>
	{
		let (width, height) = self.scale(image.dimensions())?;
		if width == image.width() && height == image.height()
		{
			Ok(None)
		}
		else
		{
			Ok(Some(image.resize_exact(width, height, filter)))
		}
	}
	
	#[inline(always)]
	pub(crate) fn scale(&self, dimensions: (u32, u32)) -> Result<(u32, u32), CordialError>
	{
		use self::ImageScale::*;
		
		match *self
		{
			Identity => Ok(dimensions),
			FixedWidth { width } =>
			{
				if width == 0
				{
					Err(CordialError::Configuration("width can not be zero".to_owned()))
				}
				else if width == dimensions.0
				{
					Ok(dimensions)
				}
				else
				{
					Ok((width, (dimensions.1 * width) / dimensions.0))
				}
			}
			FixedHeight { height } =>
			{
				if height == 0
				{
					Err(CordialError::Configuration("height can not be zero".to_owned()))
				}
				else if height == dimensions.0
				{
					Ok(dimensions)
				}
				else
				{
					Ok(((dimensions.0 * height) / dimensions.1, height))
				}
			}
			FixedSize { width, height } =>
			{
				if width == 0
				{
					Err(CordialError::Configuration("width can not be zero".to_owned()))
				}
				else if height == 0
				{
					Err(CordialError::Configuration("height can not be zero".to_owned()))
				}
				else
				{
					Ok((width, height))
				}
			},
			Ratio { upper, lower } =>
			{
				if upper == 0
				{
					Err(CordialError::Configuration("upper can not be zero".to_owned()))
				}
				else if lower == 0
				{
					Err(CordialError::Configuration("lower can not be zero".to_owned()))
				}
				else
				{
					let (width, height) = dimensions;
					Ok(((width * upper) / lower, (height * upper) / lower))
				}
			}
			Fraction { ratio } =>
			{
				if !ratio.is_normal()
				{
					Err(CordialError::Configuration(format!("ratio must be normal, not '{}'", ratio)))
				}
				else if ratio.is_sign_negative()
				{
					Err(CordialError::Configuration(format!("ratio must be positive, not '{}'", ratio)))
				}
				else
				{
					let (width, height) = dimensions;
					let newWidth = ((width as f64) * ratio) as u32;
					let newHeight = ((height as f64) * ratio) as u32;
					if newWidth == 0
					{
						Err(CordialError::Configuration("ratio must produce non-zero width".to_owned()))
					}
					else if newHeight == 0
					{
						Err(CordialError::Configuration("ratio must produce non-zero height".to_owned()))
					}
					else
					{
						Ok((newWidth, newHeight))
					}
				}
			}
		}
	}
}
