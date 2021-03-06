// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct ImageSourceSet<'a>
{
	inputContentFilePath: &'a Path,
	imageInputFormat: Option<ImageInputFormat>,
	resourceRelativeUrlWithoutFileNameExtension: &'a str,
	jpegQuality: Option<u8>,
	jpegSpeedOverCompression: bool,
	primaryImageWidth: u32,
	primaryImageHeight: u32,
	imagesInOrder: BTreeMap<u32, ::image::DynamicImage>,
	languageData: &'a LanguageData<'a>
}

impl<'a> ImageSourceSet<'a>
{
	#[inline(always)]
	pub(crate) fn new(inputContentFilePath: &'a Path, imageInputFormat: Option<ImageInputFormat>, resourceUrl: &'a ResourceUrl, jpegQuality: Option<u8>, jpegSpeedOverCompression: bool, primaryImage: ::image::DynamicImage, languageData: &'a LanguageData) -> Self
	{
		let resourceRelativeUrlWithoutFileNameExtension = resourceUrl.withoutFileNameExtension();
		
		let primaryImageWidth = primaryImage.width();
		let primaryImageHeight = primaryImage.height();
		
		let mut imagesInOrder = BTreeMap::new();
		imagesInOrder.insert(primaryImageWidth, primaryImage);
		
		Self
		{
			inputContentFilePath,
			imageInputFormat,
			resourceRelativeUrlWithoutFileNameExtension,
			jpegQuality,
			jpegSpeedOverCompression,
			primaryImageWidth,
			primaryImageHeight,
			imagesInOrder,
			languageData,
		}
	}
	
	#[inline(always)]
	fn overrideFilePath(&self, imageSourceSetEntry: &ImageSourceSetEntry, primaryImageDimensions: (u32, u32)) -> Result<Option<::image::DynamicImage>, CordialError>
	{
		let (width, height) = imageSourceSetEntry.computeCroppedAndResizedImageDimensions(primaryImageDimensions)?;
		let fileStem = self.inputContentFilePath.file_stem().unwrap();
		let extension = self.inputContentFilePath.extension().unwrap();
		
		let mut fileName = fileStem.to_os_string();
		fileName.push(&format!("-{}w-{}h.", width, height));
		fileName.push(extension);
		let overrideFilePath = self.inputContentFilePath.with_file_name(fileName);
		
		if overrideFilePath.is_file()
		{
			match ImageInputFormat::load(self.imageInputFormat, &overrideFilePath)
			{
				None => Ok(None),
				Some(result) =>
				{
					let image = result?;
					Ok(Some(image))
				}
			}
		}
		else
		{
			Ok(None)
		}
	}
	
	#[inline(always)]
	pub(crate) fn add(&mut self, imageSourceSetEntries: &[ImageSourceSetEntry]) -> Result<(), CordialError>
	{
		let primaryImageDimensions = self.primaryImage().dimensions();
		
		for imageSourceSetEntry in imageSourceSetEntries.iter()
		{
			let (width, image) = match self.overrideFilePath(imageSourceSetEntry, primaryImageDimensions)?
			{
				Some(overrideImage) => (overrideImage.width(), overrideImage),
				None => imageSourceSetEntry.cropAndResize(self.primaryImage())?,
			};
			
			self.imagesInOrder.insert(width, image);
		}
		Ok(())
	}
	
	#[inline(always)]
	fn contentTypeAndFileExtension(&self) -> (ContentType, &'static str)
	{
		if self.jpegQuality.is_some()
		{
			(content_type_image_jpeg(), ".jpg")
		}
		else
		{
			(content_type_image_png(), ".png")
		}
	}
	
	#[inline(always)]
	fn primaryImage(&mut self) -> &mut ::image::DynamicImage
	{
		self.imagesInOrder.get_mut(&self.primaryImageWidth).unwrap()
	}
	
	#[inline(always)]
	pub(crate) fn primaryImageDimensions(&self) -> (u32, u32)
	{
		(self.primaryImageWidth, self.primaryImageHeight)
	}
	
	/// Outputs (Url, Width) pairs
	/// Use as say (note commas MATTER): srcset="/url/elva-fairy-320w.jpg 320w, /url/elva-fairy-480w.jpg 480w, /url/elva-fairy.jpg 800w, /url/elva-fairy-1000w.jpg 1000w,"
	#[inline(always)]
	pub(crate) fn processedImageSourceSet(&self, imageSourceSet: &mut Vec<(Url, u32)>) -> Result<(), CordialError>
	{
		let (_, fileExtension) = self.contentTypeAndFileExtension();
		
		imageSourceSet.reserve_exact(self.imagesInOrder.len());
		
		for width in self.imagesInOrder.keys()
		{
			let width = *width;
			let url = if width == self.primaryImageWidth
			{
				ResourceUrl::primaryUrl(&self.resourceRelativeUrlWithoutFileNameExtension, fileExtension, self.languageData)?
			}
			else
			{
				ResourceUrl::widthUrl(&self.resourceRelativeUrlWithoutFileNameExtension, fileExtension, self.languageData, width)?
			};
			
			imageSourceSet.push((url, width))
		}
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn urls<F: FnMut(&Url) -> Result<Vec<(String, String)>, CordialError>>(&self, mut headerGenerator: F) -> Result<Vec<PipelineResponse>, CordialError>
	{
		let (contentType, fileExtension) = self.contentTypeAndFileExtension();
		
		let mut urls = Vec::with_capacity(self.imagesInOrder.len());
		let mut index = 0;
		let finalIndex = self.imagesInOrder.len() - 1;
		for (width, image) in self.imagesInOrder.iter()
		{
			let width = *width;
			let (url, isPrimary) = if width == self.primaryImageWidth
			{
				(ResourceUrl::primaryUrl(self.resourceRelativeUrlWithoutFileNameExtension, fileExtension, self.languageData)?, true)
			}
			else
			{
				(ResourceUrl::widthUrl(self.resourceRelativeUrlWithoutFileNameExtension, fileExtension, self.languageData, width)?, false)
			};
			
			let body = self.optimize(image)?;
			let headers = headerGenerator(&url)?;
			
			let height = image.height();
			let urlDataDetails = Rc::new
			(
				UrlDataDetails::Image
				{
					width,
					height,
					size: body.len() as u64,
				}
			);
			
			use self::ResourceTag::*;
			let mut resourceTags = hashmap!
			{
				width_image(width) => urlDataDetails.clone(),
				height_image(height) => urlDataDetails.clone(),
				width_height_image(width, height) => urlDataDetails.clone()
			};
			
			if isPrimary
			{
				resourceTags.insert(default, urlDataDetails.clone());
				resourceTags.insert(primary_image, urlDataDetails.clone());
			}
			if index == 0
			{
				resourceTags.insert(smallest_image, urlDataDetails.clone());
			}
			if index == finalIndex
			{
				resourceTags.insert(largest_image, urlDataDetails.clone());
			}
			urls.push((url, resourceTags, StatusCode::Ok, contentType.clone(), headers, ResponseBody::binary(body), None, false));
			index += 1;
		}
		Ok(urls)
	}
	
	fn optimize(&self, image: &::image::DynamicImage) -> Result<Vec<u8>, CordialError>
	{
		fn encodeJpeg(image: &::image::DynamicImage, quality: u8, path: &Path) -> Result<Vec<u8>, CordialError>
		{
			let bytes = image.raw_pixels();
			let mut jpegBytes = Vec::with_capacity(bytes.len() * 2);
			{
				let mut writer = BufWriter::with_capacity(jpegBytes.len(), &mut jpegBytes);
				
				let mut jpegEncoder = JPEGEncoder::new_with_quality(&mut writer, quality);
				
				let (width, height) = image.dimensions();
				let color = image.color();
				jpegEncoder.encode(&bytes, width, height, color).context(path)?;
			}
			Ok(jpegBytes)
		}
		
		let bytes = if let Some(jpegQuality) = self.jpegQuality
		{
			if self.jpegSpeedOverCompression
			{
				let quality = match jpegQuality
				{
					quality @ 0 ... 100 => quality,
					_ => 100
				};
				encodeJpeg(image, quality, self.inputContentFilePath)?
			}
			else
			{
				let inputJpegBytes = encodeJpeg(image, 100, self.inputContentFilePath)?;
				
				let quality = match jpegQuality
				{
					quality if quality < 84 => 84,
					quality @ 84 ... 100 => quality,
					_ => 100
				};
				
				::guetzli_sys::guetzli(&inputJpegBytes, quality, None)?
			}
		}
		else
		{
			Self::optimizePngImage(&image, self.inputContentFilePath)?
		};
		Ok(bytes)
	}
	
	fn optimizePngImage(image: &::image::DynamicImage, context: &Path) -> Result<Vec<u8>, CordialError>
	{
		let mut temporaryFile = Temp::new_file().context(context)?;
		let temporaryFilePath = temporaryFile.to_path_buf();
		let bytes =
		{
			temporaryFilePath.createFileWithPngImage(&image)?;
			temporaryFilePath.modifyPngWithOxipng()?;
			temporaryFilePath.fileContentsAsBytes().context(&temporaryFilePath)?
		};
		temporaryFilePath.deleteOverridingPermissions().context(&temporaryFilePath)?;
		temporaryFile.release();
		
		Ok(bytes)
	}
}
