// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct EngiffenSource
{
	#[serde(default = "EngiffenSource::width_default")] pub(crate) width: u16,
	#[serde(default = "EngiffenSource::height_default")] pub(crate) height: u16,
	#[serde(default)] scaling_filter: ImageTransformationFilterType,
	#[serde(default)] default: EngiffenFrame,
	#[serde(default)] perImage: HashMap<usize, EngiffenFrame>,
}

impl Default for EngiffenSource
{
	#[inline(always)]
	fn default() -> Self
	{
		EngiffenSource
		{
			width: Self::width_default(),
			height: Self::height_default(),
			scaling_filter: ImageTransformationFilterType::default(),
			default: EngiffenFrame::default(),
			perImage: HashMap::default(),
		}
	}
}

impl EngiffenSource
{
	#[inline(always)]
	fn transform<'a>(&'a self, sourceSets: &mut SourceSets<'a>, sourceSetIndex: usize, imageIndex: usize, image: &mut ::image::DynamicImage, frameWidthBySourceSet: &mut HashMap<usize, u16>, frameHeightBySourceSet: &mut HashMap<usize, u16>) -> Result<(), CordialError>
	{
		let engiffenFrame = match self.perImage.get(&imageIndex)
		{
			None => &self.default,
			Some(engiffenFrame) => engiffenFrame,
		};
		
		let transformedImage = if let Some(transformedImage) = engiffenFrame.transform(image)?
		{
			transformedImage
		}
		else
		{
			image.clone()
		};
		
		if imageIndex == 0
		{
			let frameWidth = if self.width == 0
			{
				let potentialFrameWidth = transformedImage.width();
				if potentialFrameWidth > (u16::max_value() as u32)
				{
					return Err(CordialError::Configuration("If using defaults for a GIF source set, then a transformed image's width must not exceed 2^16-1, ie 65,535 pixels".to_owned()))
				}
				potentialFrameWidth as u16
			}
			else
			{
				self.width
			};
			frameWidthBySourceSet.insert(sourceSetIndex, frameWidth);
			
			let frameHeight = if self.height == 0
			{
				let potentialFrameHeight = transformedImage.height();
				if potentialFrameHeight > (u16::max_value() as u32)
				{
					return Err(CordialError::Configuration("If using defaults for a GIF source set, then a transformed image's height must not exceed 2^16-1, ie 65,535 pixels".to_owned()))
				}
				potentialFrameHeight as u16
			}
			else
			{
				self.height
			};
			frameHeightBySourceSet.insert(sourceSetIndex, frameHeight);
		}
		
		let resizedImage =
		{
			let ourCorrectedWidth = if self.width == 0
			{
				*frameWidthBySourceSet.get(&sourceSetIndex).unwrap()
			}
			else
			{
				self.width
			};
			
			let ourCorrectedHeight = if self.height == 0
			{
				*frameHeightBySourceSet.get(&sourceSetIndex).unwrap()
			}
			else
			{
				self.height
			};
			
			let (transformedWidth, transformedHeight) = transformedImage.dimensions();
			
			let frameWidth = ourCorrectedWidth as u32;
			let frameHeight = ourCorrectedHeight as u32;
			
			if transformedWidth == frameWidth && transformedHeight == frameHeight
			{
				transformedImage
			}
			else
			{
				transformedImage.resize_exact(frameWidth, frameHeight, self.scaling_filter.to_FilterType())
			}
		};
		
		let engiffenImage = Self::toEngiffenOutputImage(&resizedImage);
		
		let sourceSet = sourceSets.get_mut(sourceSetIndex).unwrap();
		sourceSet.0.push(engiffenImage);
		sourceSet.1.push(engiffenFrame);
		
		Ok(())
	}
	
	#[inline(always)]
	fn toEngiffenOutputImage(image: &EngiffenSourceImage) -> EngiffenOutputImage
	{
		const PixelSize: usize = 4;
		
		let width = image.width();
		let height = image.height();
		
		let mut pixels = Vec::with_capacity((width as usize) * (height as usize) * PixelSize);
		for (_, _, px) in image.pixels()
		{
			pixels.push(px.data);
		}
		
		EngiffenOutputImage
		{
			pixels,
			width,
			height,
		}
	}
	
	#[inline(always)]
	fn width_default() -> u16
	{
		16
	}
	
	#[inline(always)]
	fn height_default() -> u16
	{
		16
	}
}
