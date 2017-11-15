// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct ImageSourceSet<'a>
{
	inputContentFilePath: &'a Path,
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
	pub(crate) fn new(inputContentFilePath: &'a Path, resourceRelativeUrlWithoutFileNameExtension: &'a str, jpegQuality: Option<u8>, jpegSpeedOverCompression: bool, primaryImage: ::image::DynamicImage, languageData: &'a LanguageData) -> Self
	{
		let primaryImageWidth = primaryImage.width();
		let primaryImageHeight = primaryImage.height();
		
		let mut imagesInOrder = BTreeMap::new();
		imagesInOrder.insert(primaryImageWidth, primaryImage);
		
		Self
		{
			inputContentFilePath,
			resourceRelativeUrlWithoutFileNameExtension,
			jpegQuality,
			jpegSpeedOverCompression,
			primaryImageWidth,
			primaryImageHeight,
			imagesInOrder,
			languageData
		}
	}
	
	#[inline(always)]
	pub(crate) fn add(&mut self, imageSourceSetEntries: &[ImageSourceSetEntry]) -> Result<(), CordialError>
	{
		for imageSourceSetEntry in imageSourceSetEntries.iter()
		{
			let (width, image) = imageSourceSetEntry.cropAndResize(self.primaryImage())?;
			self.imagesInOrder.insert(width, image);
		}
		Ok(())
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
	pub(crate) fn processedImageSourceSet(&self) -> Result<Vec<(Url, u32)>, CordialError>
	{
		let fileExtension = if self.jpegQuality.is_some()
		{
			".jpg"
		}
		else
		{
			".png"
		};
		
		let mut imageSourceSet = Vec::with_capacity(self.imagesInOrder.len());
		for width in self.imagesInOrder.keys()
		{
			let width = *width;
			let url = if width == self.primaryImageWidth
			{
				Self::primaryUrl(self.resourceRelativeUrlWithoutFileNameExtension, fileExtension, self.languageData)?
			}
			else
			{
				Self::widthUrl(self.resourceRelativeUrlWithoutFileNameExtension, fileExtension, self.languageData, width)?
			};
			
			imageSourceSet.push((url, width))
		}
		Ok(imageSourceSet)
	}
	
	#[inline(always)]
	pub(crate) fn urls<F: FnMut(&Url) -> Result<Vec<(String, String)>, CordialError>>(&self, mut headerGenerator: F) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		let (contentType, fileExtension) = if self.jpegQuality.is_some()
		{
			(ContentType::jpeg(), ".jpg")
		}
		else
		{
			(ContentType::png(), ".png")
		};
		
		let mut urls = Vec::with_capacity(self.imagesInOrder.len());
		let mut index = 0;
		let finalIndex = self.imagesInOrder.len() - 1;
		for (width, image) in self.imagesInOrder.iter()
		{
			let width = *width;
			let (url, isPrimary) = if width == self.primaryImageWidth
			{
				(Self::primaryUrl(self.resourceRelativeUrlWithoutFileNameExtension, fileExtension, self.languageData)?, true)
			}
			else
			{
				(Self::widthUrl(self.resourceRelativeUrlWithoutFileNameExtension, fileExtension, self.languageData, width)?, false)
			};
			
			let body = self.optimize(image)?;
			let headers = headerGenerator(&url)?;
			
			let height = image.height();
			let jsonValue = Rc::new
			(
				json!
				({
					"width": width,
					"height": height,
					"mime": contentType.0.as_ref().to_owned(),
					"size": body.len() as u64,
				})
			);
			
			use self::UrlTag::*;
			let mut urlTags = hashmap!
			{
				width_image(width) => jsonValue.clone(),
				height_image(height) => jsonValue.clone(),
				width_height_image(width, height) => jsonValue.clone()
			};
			
			if isPrimary
			{
				urlTags.insert(default, jsonValue.clone());
				urlTags.insert(primary_image, jsonValue.clone());
			}
			if index == 0
			{
				urlTags.insert(smallest_image, jsonValue.clone());
			}
			if index == finalIndex
			{
				urlTags.insert(largest_image, jsonValue.clone());
			}
			urls.push((url, urlTags, StatusCode::Ok, contentType.clone(), headers, body, None, false));
			index += 1;
		}
		Ok(urls)
	}
	
	pub(crate) fn widthUrl(resourceRelativeUrlWithoutFileNameExtension: &str, fileExtension: &'static str, languageData: &LanguageData, width: u32) -> Result<Url, CordialError>
	{
		let mut path = String::with_capacity(resourceRelativeUrlWithoutFileNameExtension.len() + 10);
		path.push_str(resourceRelativeUrlWithoutFileNameExtension.as_ref());
		path.push_str(&format!("-{}w", width));
		path.push_str(fileExtension);
		
		Ok(languageData.url(&path)?)
	}
	
	pub(crate) fn primaryUrl(resourceRelativeUrlWithoutFileNameExtension: &str, fileExtension: &'static str, languageData: &LanguageData) -> Result<Url, CordialError>
	{
		let mut path = String::with_capacity(resourceRelativeUrlWithoutFileNameExtension.len() + 10);
		path.push_str(resourceRelativeUrlWithoutFileNameExtension.as_ref());
		path.push_str(fileExtension);
		
		Ok(languageData.url(&path)?)
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
			let mut temporaryFile = Temp::new_file().context(self.inputContentFilePath)?;
			let temporaryFilePath = temporaryFile.to_path_buf();
			let bytes =
			{
				temporaryFilePath.createFileWithPngImage(&image)?;
				temporaryFilePath.modifyPngWithOxipng()?;
				temporaryFilePath.fileContentsAsBytes().context(&temporaryFilePath)?
			};
			temporaryFilePath.deleteOverridingPermissions().context(&temporaryFilePath)?;
			temporaryFile.release();
			
			bytes
		};
		Ok(bytes)
	}
}
