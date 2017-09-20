// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct ImageSourceSet<'a>
{
	primaryUnversionedUrl: Url,
	inputContentFilePath: &'a Path,
	jpegQuality: Option<u8>,
	primaryImageWidth: u32,
	primaryImageHeight: u32,
	imagesInOrder: BTreeMap<u32, ::image::DynamicImage>,
}

impl<'a> ImageSourceSet<'a>
{
	#[inline(always)]
	pub(crate) fn new(inputContentFilePath: &'a Path, primaryUnversionedUrl: Url, jpegQuality: Option<u8>, primaryImage: ::image::DynamicImage) -> Self
	{
		let primaryImageWidth = primaryImage.width();
		let primaryImageHeight = primaryImage.height();
		
		let mut imagesInOrder = BTreeMap::new();
		imagesInOrder.insert(primaryImageWidth, primaryImage);
		
		Self
		{
			primaryUnversionedUrl,
			inputContentFilePath,
			jpegQuality,
			primaryImageWidth,
			primaryImageHeight,
			imagesInOrder,
		}
	}
	
	#[inline(always)]
	pub(crate) fn generate(&mut self, img_srcset: &[ImageSourceSetEntry]) -> Result<(), CordialError>
	{
		for imageSourceSetEntry in img_srcset.iter()
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
	pub(crate) fn processedImageSourceSet(&self) -> Vec<(Url, u32)>
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
				self.primaryUnversionedUrl.clone()
			}
			else
			{
				Self::widthUrl(width, &self.primaryUnversionedUrl, fileExtension)
			};
			
			imageSourceSet.push((url, width))
		}
		imageSourceSet
	}
	
	#[inline(always)]
	pub(crate) fn urls<F: FnMut(&Url, bool) -> Result<Vec<(String, String)>, CordialError>>(&self, mut headerGenerator: F, canBeCompressed: bool) -> Result<Vec<(Url, HashSet<UrlTag>, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
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
				(self.primaryUnversionedUrl.clone(), true)
			}
			else
			{
				(Self::widthUrl(width, &self.primaryUnversionedUrl, fileExtension), false)
			};
			
			let body = self.optimize(image)?;
			let headers = headerGenerator(&url, canBeCompressed)?;
			
			use self::UrlTag::*;
			
			let mut urlTags = hashset!(width_image(width), height_image(height), width_height_image(width, height));
			if isPrimary
			{
				urlTags.insert(default);
			}
			if index == 0
			{
				urlTags.insert(smallest);
			}
			if index == finalIndex
			{
				urlTags.insert(largest);
			}
			urls.push((url, urlTags, contentType.clone(), headers, body, None, canBeCompressed));
			index += 1;
		}
		Ok(urls)
	}
	
	fn widthUrl(width: u32, url: &Url, fileExtension: &'static str) -> Url
	{
		let fileName = url.fileNameOrIndexNamePercentDecodedUntrusted(fileExtension);
		
		let mut replacementFileName = String::with_capacity(fileName.len() + 6);
		let index = match fileName.rfind('.')
		{
			None => fileName.len(),
			Some(index) => index,
		};
		let (left, right) = fileName.split_at(index);
		replacementFileName.push_str(left);
		replacementFileName.push_str(&format!("-{}w", width));
		replacementFileName.push_str(right);
		
		url.clone().pushReplacementFileName(&replacementFileName)
	}
	
	fn optimize(&self, image: &::image::DynamicImage) -> Result<Vec<u8>, CordialError>
	{
		let bytes = if let Some(jpegQuality) = self.jpegQuality
		{
			let quality = match jpegQuality
			{
				0 => 1,
				quality @ 0 ... 100 => quality,
				_ => 100
			};
			
			// create PNG bytes
			let mut pngBytes = Vec::with_capacity(32 * 1024);
			{
				let mut writer = BufWriter::with_capacity(pngBytes.len(), &mut pngBytes);
				image.save(&mut writer, ::image::ImageFormat::PNG).context(self.inputContentFilePath)?;
			}
			
			// create JPEG
			CordialError::executeCommandCapturingStandardOut(Command::new("guetzli").env_clear().args(&["--nomemlimit", "--quality", &format!("{}", quality), "-", "-"]), self.inputContentFilePath, pngBytes)?
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
