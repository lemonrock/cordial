// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
	pub(crate) fn toWebFonts(option: Option<Self>, resourceRelativeUrl: &str, configuration: &Configuration, inputContentFilePath: &Path, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, ifLanguageAwareLanguageData: Option<&LanguageData>, languageData: &LanguageData, max_age_in_seconds: u32, is_downloadable: bool, utf8_xml_metadata: &[u8], woff1_private_data: &[u8], woff1_iterations: u16, woff2_brotli_quality: u8, woff2_disallow_transforms: bool, include_ttf: bool) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
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
		
		format.process(resourceRelativeUrl, configuration, inputContentFilePath, handlebars, headerTemplates, ifLanguageAwareLanguageData, languageData, max_age_in_seconds, is_downloadable, &utf8_xml_metadata[..], &woff1_private_data[..], woff1_iterations, woff2_brotli_quality, woff2_disallow_transforms, include_ttf)
	}
	
	#[inline(always)]
	fn process(&self, resourceRelativeUrl: &str, configuration: &Configuration, inputContentFilePath: &Path, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, ifLanguageAwareLanguageData: Option<&LanguageData>, languageData: &LanguageData, max_age_in_seconds: u32, is_downloadable: bool, utf8_xml_metadata: &[u8], woff1_private_data: &[u8], woff1_iterations: u16, woff2_brotli_quality: u8, woff2_disallow_transforms: bool, include_ttf: bool) -> Result<Vec<(Url, HashMap<UrlTag, Rc<JsonValue>>, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		use self::UrlTag::*;
		
		const canBeCompressed: bool = false;
		
		let ttfBytes = inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?;
		
		let mut urls = Vec::with_capacity(3);
		
		// woff
		{
			let woffNumberOfIterations = match woff1_iterations
			{
				woffNumberOfIterations @ 0 ... 5000 => woffNumberOfIterations,
				_ => 5000,
			};
			let woffUrl = languageData.url(&ResourcePipeline::replaceFileNameExtension(resourceRelativeUrl, ".woff2"))?;
			let woffHeaders = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, &woffUrl)?;
			let woffBody = encodeWoff(&ttfBytes, woffNumberOfIterations, DefaultFontMajorVersion, DefaultFontMinorVersion, utf8_xml_metadata, woff1_private_data).context(inputContentFilePath)?.as_ref().to_vec();
			urls.push((woffUrl, hashmap! { default => Rc::new(JsonValue::Null) }, ContentType(ResourcePipeline::mimeType("font/woff")), woffHeaders, woffBody, None, canBeCompressed));
		}
		
		// woff2
		{
			let woff2BrotliQuality = match woff2_brotli_quality
			{
				0 => 1,
				quality @ 1 ... 11 => quality,
				_ => 11,
			};
			let woff2Url = languageData.url(&ResourcePipeline::replaceFileNameExtension(resourceRelativeUrl, ".woff2"))?;
			let woff2Headers = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, canBeCompressed, max_age_in_seconds, is_downloadable, &woff2Url)?;
			let woff2Body = match convertTtfToWoff2(&ttfBytes, utf8_xml_metadata, woff2BrotliQuality, !woff2_disallow_transforms)
			{
				Err(()) => return Err(CordialError::Configuration("Could not encode font to WOFF2".to_owned())),
				Ok(body) => body,
			};
			urls.push((woff2Url, hashmap! { default => Rc::new(JsonValue::Null) }, ContentType(ResourcePipeline::mimeType("font/woff2")), woff2Headers, woff2Body, None, canBeCompressed));
		}
		
		if include_ttf
		{
			let ttfUrl = languageData.url(resourceRelativeUrl)?;
			let ttfHeaders = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, true, max_age_in_seconds, is_downloadable, &ttfUrl)?;
			urls.push((ttfUrl, hashmap! { default => Rc::new(JsonValue::Null) }, ContentType(ResourcePipeline::mimeType("application/font-sfnt")), ttfHeaders, ttfBytes, None, canBeCompressed));
		}
		
		Ok(urls)
	}
}