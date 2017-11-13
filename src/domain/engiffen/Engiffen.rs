// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct Engiffen<'a>
{
	inputContentFolderPath: &'a Path,
	engiffenSources: &'a [EngiffenSource],
	quantizer: Quantizer,
	loops: EngiffenLoops,
	inputFormat: Option<ImageInputFormat>,
}

impl<'a> Engiffen<'a>
{
	#[inline(always)]
	pub(crate) fn new(inputContentFolderPath: &'a Path, engiffenSources: &'a [EngiffenSource], quantizer: &EngiffenQuantizer, loops: EngiffenLoops, inputFormat: Option<ImageInputFormat>) -> Self
	{
		Self
		{
			inputContentFolderPath,
			engiffenSources,
			quantizer: quantizer.toQuantizer(),
			loops,
			inputFormat
		}
	}
	
	#[inline(always)]
	pub(crate) fn process<F: for<'r> FnMut(&'r Url) -> Result<Vec<(String, String)>, CordialError>>(&self, resourceRelativeUrlWithoutFileNameExtension: &str, languageData: &'a LanguageData, mut headerGenerator: F) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		let images = self.inputContentFolderPath.fileContentsInFolder(|filePath|
		{
			match ImageInputFormat::load(self.inputFormat, filePath)
			{
				None => None,
				Some(Err(error)) => Some(Err(error)),
				Some(Ok(image)) =>
				{
					let result = if image.width() > (u16::max_value() as u32)
					{
						Err(CordialError::Configuration("GIF images may not have frames with a width greater than 2^16 - 1, ie 65,535 pixels".to_owned()))
					}
					else if image.height() > (u16::max_value() as u32)
					{
						Err(CordialError::Configuration("GIF images may not have frames with a height greater than 2^16 - 1, ie 65,535 pixels".to_owned()))
					}
					else
					{
						Ok(image)
					};
					Some(result)
				}
			}
		})?;
		
		if images.is_empty()
		{
			return Err(CordialError::Configuration(format!("No matching images for engiffen for '{:?}'", self.inputContentFolderPath)));
		}
		
		let (mut sourceSets, defaultEngiffenSource) =
		{
			let (numberOfSourceSets, defaultEngiffenSource) = if self.engiffenSources.is_empty()
			{
				(1, Some(EngiffenSource::default()))
			}
			else
			{
				(self.engiffenSources.len(), None)
			};
			let mut sourceSets = Vec::with_capacity(numberOfSourceSets);
			while sourceSets.len() < numberOfSourceSets
			{
				sourceSets.push((Vec::with_capacity(images.len()), Vec::with_capacity(images.len())));
			}
			
			(sourceSets, defaultEngiffenSource)
		};
		
		let mut frameWidthBySourceSet = HashMap::with_capacity(sourceSets.len());
		let mut frameHeightBySourceSet = HashMap::with_capacity(sourceSets.len());
		
		{
			let mut imageIndex = 0;
			for (_, mut image) in images
			{
				match defaultEngiffenSource
				{
					Some(ref engiffenSource) =>
					{
						engiffenSource.transform(&mut sourceSets, 0, imageIndex, &mut image, &mut frameWidthBySourceSet, &mut frameHeightBySourceSet)?;
					}
					None =>
					{
						let mut sourceSetIndex = 0;
						for engiffenSource in self.engiffenSources.iter()
						{
							engiffenSource.transform(&mut sourceSets, sourceSetIndex, imageIndex, &mut image, &mut frameWidthBySourceSet, &mut frameHeightBySourceSet)?;
							sourceSetIndex += 1;
						}
					}
				}
				imageIndex += 1;
			}
		}
		
		let mut result = Vec::with_capacity(sourceSets.len());
		let mut sourceSetIndex = 0;
		for (engiffenImages, engiffenFrames) in sourceSets
		{
			const IgnoreFramesPerSecond: usize = 1;
			
			let gifAnimation = ::engiffen::engiffen(&engiffenImages[..], IgnoreFramesPerSecond, self.quantizer).expect("Should not occur");
			let mut body = Vec::with_capacity(128 * 1024);
			self.writeGifAnimation(&gifAnimation, &mut body, &engiffenFrames[..])?;
			body.shrink_to_fit();
			
			let width = *frameWidthBySourceSet.get(&sourceSetIndex).unwrap() as u32;
			let height = *frameHeightBySourceSet.get(&sourceSetIndex).unwrap() as u32;
			
			let jsonValue = Rc::new
			(
				json!
				({
					"width": width,
					"height": height,
				})
			);
			
			let mut urlTags = hashmap!
			{
				width_image(width) => jsonValue.clone(),
				height_image(height) => jsonValue.clone(),
				width_height_image(width, height) => jsonValue.clone(),
			};
			
			use self::UrlTag::*;
			let url = if sourceSetIndex == 0
			{
				urlTags.insert(default, jsonValue.clone());
				urlTags.insert(primary_image, jsonValue.clone());
				ImageSourceSet::primaryUrl(resourceRelativeUrlWithoutFileNameExtension, ".gif", languageData)?
			}
			else
			{
				ImageSourceSet::widthUrl(resourceRelativeUrlWithoutFileNameExtension, ".gif", languageData, width)?
			};
			
			let headers = headerGenerator(&url)?;
			
			result.push((url, urlTags, StatusCode::Ok, ContentType(mime::IMAGE_GIF), headers, body, None, false));
			
			sourceSetIndex += 1;
		}
		Ok(result)
	}
	
	#[inline(always)]
	fn writeGifAnimation<W: Write>(&self, gifAnimation: &Gif, writer: &mut W, engiffenFrames: &[&EngiffenFrame]) -> Result<(), CordialError>
	{
		let mut encoder = ::gif::Encoder::new(writer, gifAnimation.width, gifAnimation.height, &gifAnimation.palette).context(self.inputContentFolderPath)?;
		encoder.set(self.loops.toRepeat()).context(self.inputContentFolderPath)?;
		
		let mut frameIndex = 0;
		for imageFrame in &gifAnimation.images
		{
			let mut frame = ::gif::Frame::default();
			frame.width = gifAnimation.width;
			frame.height = gifAnimation.height;
			frame.buffer = Cow::Borrowed(&*imageFrame);
			frame.transparent = gifAnimation.transparency;
			
			encoder.write_frame(&frame).context(self.inputContentFolderPath)?;
			
			engiffenFrames.get(frameIndex).unwrap().modify(&mut frame);
			
			frameIndex += 1;
		}
		Ok(())
	}
}
