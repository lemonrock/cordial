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
	resourceRelativeUrlWithoutFileNameExtension: &'a str,
	languageData: &'a LanguageData<'a>,
}

impl<'a> Engiffen<'a>
{
	#[inline(always)]
	pub(crate) fn new(inputContentFolderPath: &'a Path, engiffenSources: &'a [EngiffenSource], quantizer: &EngiffenQuantizer, loops: EngiffenLoops, inputFormat: Option<ImageInputFormat>, resourceUrl: &'a ResourceUrl, languageData: &'a LanguageData) -> Result<Self, CordialError>
	{
		if engiffenSources.is_empty()
		{
			Err(CordialError::Configuration("No images in GifAnimationPipeline".to_owned()))
		}
		else
		{
			Ok
			(
				Self
				{
					inputContentFolderPath,
					engiffenSources,
					quantizer: quantizer.toQuantizer(),
					loops,
					inputFormat,
					resourceRelativeUrlWithoutFileNameExtension: resourceUrl.withoutFileNameExtension(),
					languageData,
				}
			)
		}
	}
	
	/// Outputs (Url, Width) pairs
	/// Use as say (note commas MATTER): srcset="/url/elva-fairy-320w.gif 320w, /url/elva-fairy-480w.gif 480w, /url/elva-fairy.gif 800w, /url/elva-fairy-1000w.gif 1000w"
	#[inline(always)]
	pub(crate) fn processedImageSourceSet(&self, imageSourceSet: &mut ProcessedImageSourceSet) -> Result<(), CordialError>
	{
		let imageSourceSet = &mut imageSourceSet.2;
		
		imageSourceSet.reserve_exact(self.engiffenSources.len());
		
		let mut isAfterFirst = false;
		for engiffenSource in self.engiffenSources
		{
			let width = engiffenSource.width as u32;
			let url = if isAfterFirst
			{
				ResourceUrl::widthUrl(self.resourceRelativeUrlWithoutFileNameExtension, Self::GifFileExtension, self.languageData, width)?
			}
			else
			{
				isAfterFirst = true;
				ResourceUrl::primaryUrl(self.resourceRelativeUrlWithoutFileNameExtension, Self::GifFileExtension, self.languageData)?
			};
			imageSourceSet.push((url, width));
		}
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn process<HeaderGenerator: for<'r> FnMut(&'r Url) -> Result<Vec<(String, String)>, CordialError>>(&self, mut headerGenerator: HeaderGenerator) -> Result<Vec<(Url, HashMap<ResourceTag, Rc<UrlDataDetails>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		let sourceImages = self.sourceImages()?;
		
		let (mut sourceSets, defaultEngiffenSource) = self.sourceSetsAndDefaultEngiffenSource(&sourceImages);
		
		let length = sourceSets.len();
		let mut frameWidthBySourceSet = HashMap::with_capacity(length);
		let mut frameHeightBySourceSet = HashMap::with_capacity(length);
		
		// Transform images. Can not be easily abstracted to a method due to lifetimes.
		{
			let mutableBorrowOfSourceSets = &mut sourceSets;
			
			let mut imageIndex = 0;
			for (_, mut image) in sourceImages
			{
				match defaultEngiffenSource
				{
					Some(ref engiffenSource) =>
					{
						engiffenSource.transform(mutableBorrowOfSourceSets, 0, imageIndex, &mut image, &mut frameWidthBySourceSet, &mut frameHeightBySourceSet)?;
					}
					None =>
					{
						let mut sourceSetIndex = 0;
						for engiffenSource in self.engiffenSources.iter()
						{
							engiffenSource.transform(mutableBorrowOfSourceSets, sourceSetIndex, imageIndex, &mut image, &mut frameWidthBySourceSet, &mut frameHeightBySourceSet)?;
							sourceSetIndex += 1;
						}
					}
				}
				imageIndex += 1;
			}
		}
		
		let mut result = Vec::with_capacity(length);
		for sourceSetIndex in 0 .. length
		{
			result.push(self.createGifResource(&sourceSets, &frameWidthBySourceSet, &frameHeightBySourceSet, &mut headerGenerator, sourceSetIndex)?);
		}
		Ok(result)
	}
	
	#[inline(always)]
	fn createGifResource<'b, HeaderGenerator: for<'r> FnMut(&'r Url) -> Result<Vec<(String, String)>, CordialError>>(&self, sourceSets: &SourceSets<'b>, frameWidthBySourceSet: &HashMap<usize, u16>, frameHeightBySourceSet: &HashMap<usize, u16>, headerGenerator: &mut HeaderGenerator, sourceSetIndex: usize) -> Result<(Url, HashMap<ResourceTag, Rc<UrlDataDetails>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool), CordialError>
	{
		#[inline(always)]
		fn frameDimension(frameDimensions: &HashMap<usize, u16>, sourceSetIndex: usize) -> u32
		{
			*frameDimensions.get(&sourceSetIndex).unwrap() as u32
		}
		
		use self::ResourceTag::*;
		
		let &(ref engiffenImages, ref engiffenFrames) = sourceSets.get(sourceSetIndex).unwrap();
		
		let body = self.toGifBytes(engiffenImages, engiffenFrames)?;
		
		let width = frameDimension(&frameWidthBySourceSet, sourceSetIndex);
		let height = frameDimension(&frameHeightBySourceSet, sourceSetIndex);
		
		let urlDataDetails = Rc::new
		(
			UrlDataDetails::Image
			{
				width,
				height,
				mime: Self::GifMimeType,
				size: body.len() as u64,
			}
		);
		
		let mut resourceTags = hashmap!
		{
			width_image(width) => urlDataDetails.clone(),
			height_image(height) => urlDataDetails.clone(),
			width_height_image(width, height) => urlDataDetails.clone(),
		};
		
		let url = if sourceSetIndex == 0
		{
			resourceTags.insert(default, urlDataDetails.clone());
			resourceTags.insert(primary_image, urlDataDetails.clone());
			ResourceUrl::primaryUrl(self.resourceRelativeUrlWithoutFileNameExtension, Self::GifFileExtension, self.languageData)?
		}
		else
		{
			ResourceUrl::widthUrl(self.resourceRelativeUrlWithoutFileNameExtension, Self::GifFileExtension, self.languageData, width)?
		};
		
		let headers = headerGenerator(&url)?;
		Ok((url, resourceTags, StatusCode::Ok, ContentType(Self::GifMimeType), headers, body, None, false))
	}
	
	const GifMimeType: Mime = mime::IMAGE_GIF;
	
	const GifFileExtension: &'static str = ".gif";
	
	#[inline(always)]
	fn sourceImages(&self) -> Result<EngiffenSourceImages, CordialError>
	{
		let sourceImages = self.inputContentFolderPath.fileContentsInFolder(|filePath|
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
		
		if sourceImages.is_empty()
		{
			Err(CordialError::Configuration(format!("No matching images for engiffen for '{:?}'", self.inputContentFolderPath)))
		}
		else
		{
			Ok(sourceImages)
		}
	}
	
	#[inline(always)]
	fn sourceSetsAndDefaultEngiffenSource<'b>(&self, sourceImages: &EngiffenSourceImages) -> (SourceSets<'b>, Option<EngiffenSource>)
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
			sourceSets.push((Vec::with_capacity(sourceImages.len()), Vec::with_capacity(sourceImages.len())));
		}
		
		(sourceSets, defaultEngiffenSource)
	}
	
	#[inline(always)]
	fn toGifBytes(&self, engiffenImages: &Vec<EngiffenOutputImage>, engiffenFrames: &Vec<&'a EngiffenFrame>) -> Result<Vec<u8>, CordialError>
	{
		const IgnoreFramesPerSecond: usize = 1;
		
		let gifAnimation = ::engiffen::engiffen(&engiffenImages[..], IgnoreFramesPerSecond, self.quantizer).expect("Should not occur");
		let mut body = Vec::with_capacity(128 * 1024);
		self.writeGifAnimation(&gifAnimation, &mut body, &engiffenFrames[..])?;
		body.shrink_to_fit();
		
		Ok(body)
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
