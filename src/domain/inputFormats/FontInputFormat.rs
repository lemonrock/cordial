// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum FontInputFormat
{
	TTF,
	OTF,
}

impl Default for FontInputFormat
{
	#[inline(always)]
	fn default() -> Self
	{
		FontInputFormat::TTF
	}
}

impl InputFormat for FontInputFormat
{
	#[inline(always)]
	fn fileExtensions(&self) -> &'static [&'static str]
	{
		use self::FontInputFormat::*;
		
		match *self
		{
			TTF => &[".ttf"],
			OTF => &[".otf"],
		}
	}
	
	#[inline(always)]
	fn allFileExtensions() -> &'static [&'static str]
	{
		&[
			".ttf",
			".otf",
		]
	}
}

impl FontInputFormat
{
	#[inline(always)]
	pub(crate) fn toWebFonts(option: Option<Self>, resourceUrl: &ResourceUrl, inputContentFilePath: &Path, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, maximumAgeInSeconds: u32, isDownloadable: bool, utf8_xml_metadata: &[u8], woff1_private_data: &[u8], woff1_iterations: u16, woff2_brotli_quality: u8, woff2_disallow_transforms: bool, include_ttf: Option<TtfMimeType>) -> Result<Vec<PipelineResponse>, CordialError>
	{
		let format = match option
		{
			Some(format) => format,
			None =>
			{
				use self::FontInputFormat::*;
				
				match inputContentFilePath.extension().unwrap().to_str().unwrap()
				{
					"ttf" => TTF,
					"otf" => OTF,
					_ => panic!("How is this possible?"),
				}
			}
		};
		
		format.process(resourceUrl, inputContentFilePath, headerGenerator, languageData, maximumAgeInSeconds, isDownloadable, &utf8_xml_metadata[..], &woff1_private_data[..], woff1_iterations, woff2_brotli_quality, woff2_disallow_transforms, include_ttf)
	}
	
	#[inline(always)]
	fn process(&self, resourceUrl: &ResourceUrl, inputContentFilePath: &Path, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, maximumAgeInSeconds: u32, isDownloadable: bool, utf8_xml_metadata: &[u8], woff1_private_data: &[u8], woff1_iterations: u16, woff2_brotli_quality: u8, woff2_disallow_transforms: bool, includeTrueTypeFont: Option<TtfMimeType>) -> Result<Vec<PipelineResponse>, CordialError>
	{
		const CanBeCompressed: bool = true;
		const CanNotBeCompressed: bool = false;
		
		let ttfBytes = inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?;
		
		let mut urls = Vec::with_capacity(3);
		
		// woff
		{
			let woffNumberOfIterations = match woff1_iterations
			{
				woffNumberOfIterations @ 0 ... 5000 => woffNumberOfIterations,
				_ => 5000,
			};
			let woffUrl = resourceUrl.replaceFileNameExtension(".woff2").url(languageData)?;
			let woffHeaders = headerGenerator.generateHeadersForAsset(CanNotBeCompressed, maximumAgeInSeconds, isDownloadable, &woffUrl)?;
			let woffBody = encodeWoff(&ttfBytes, woffNumberOfIterations, DefaultFontMajorVersion, DefaultFontMinorVersion, utf8_xml_metadata, woff1_private_data).context(inputContentFilePath)?.as_ref().to_vec();
			let NoPjax = None;
			urls.push((woffUrl, Self::defaultHashMap(&woffBody), StatusCode::Ok, content_type_font_woff(), woffHeaders, ResponseBody::binary(woffBody), NoPjax, CanNotBeCompressed));
		}
		
		// woff2
		{
			let woff2BrotliQuality = match woff2_brotli_quality
			{
				0 => 1,
				quality @ 1 ... 11 => quality,
				_ => 11,
			};
			let woff2Url = resourceUrl.replaceFileNameExtension(".woff2").url(languageData)?;
			let woff2Headers =  headerGenerator.generateHeadersForAsset(CanNotBeCompressed, maximumAgeInSeconds, isDownloadable, &woff2Url)?;
			let woff2Body = match convertTtfToWoff2(&ttfBytes, utf8_xml_metadata, woff2BrotliQuality, !woff2_disallow_transforms)
			{
				Err(()) => return Err(CordialError::Configuration("Could not encode font to WOFF2".to_owned())),
				Ok(body) => body,
			};
			let NoPjax = None;
			urls.push((woff2Url, Self::defaultHashMap(&woff2Body), StatusCode::Ok, content_type_font_woff2(), woff2Headers, ResponseBody::binary(woff2Body), NoPjax, CanNotBeCompressed));
		}
		
		if let Some(trueTypeFontContentType) = includeTrueTypeFont
		{
			let ttfUrl = resourceUrl.url(languageData)?;
			let ttfHeaders =  headerGenerator.generateHeadersForAsset(CanBeCompressed, maximumAgeInSeconds, isDownloadable, &ttfUrl)?;
			let NoPjax = None;
			urls.push((ttfUrl, Self::defaultHashMap(&ttfBytes), StatusCode::Ok, trueTypeFontContentType.contentType(), ttfHeaders, ResponseBody::binary(ttfBytes), NoPjax, CanBeCompressed));
		}
		
		Ok(urls)
	}
	
	#[inline(always)]
	fn defaultHashMap(body: &[u8]) -> HashMap<ResourceTag, Rc<UrlDataDetails>>
	{
		hashmap! { ResourceTag::default => Rc::new(UrlDataDetails::generic(body)) }
	}
}
