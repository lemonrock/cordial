// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TemplatePipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] language_aware: bool,
	
	#[serde(default)] template_parameters: Option<JsonMap<String, JsonValue>>,
	#[serde(default)] can_be_compressed: Option<bool>, // default is to use filename
	#[serde(default)] mime_type: Option<MimeSerde>, // default is to use filename, and sniff text formats, with US-ASCII interpreted as UTF-8
	#[serde(default = "TemplatePipeline::minify_default")] minify: bool,
	#[serde(default)] anchor_title: HashMap<Iso639Dash1Alpha2Language, Rc<String>>,
	#[serde(default = "TemplatePipeline::status_code_default", with = "::serde_with::StatusCodeSerde")] status_code: StatusCode,
}

impl Default for TemplatePipeline
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			max_age_in_seconds: max_age_in_seconds_long_default(),
			is_downloadable: is_downloadable_false_default(),
			is_versioned: is_versioned_true_default(),
			language_aware: false,
			template_parameters: None,
			can_be_compressed: None,
			mime_type: None,
			minify: TemplatePipeline::minify_default(),
			anchor_title: Default::default(),
			status_code: StatusCode::Ok,
		}
	}
}

impl Pipeline for TemplatePipeline
{
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority
	{
		NoDependenciesEgImage
	}

	#[inline(always)]
	fn is<'a>(&self) -> (bool, bool)
	{
		(self.is_versioned, self.language_aware)
	}
	
	#[inline(always)]
	fn anchorTitleAttribute(&self, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<Option<Rc<String>>, CordialError>
	{
		match self.anchor_title.get(&iso639Dash1Alpha2Language)
		{
			Some(anchorTitleAttribute) => Ok(Some(anchorTitleAttribute.clone())),
			None => match self.anchor_title.get(&fallbackIso639Dash1Alpha2Language)
			{
				Some(anchorTitleAttribute) => Ok(Some(anchorTitleAttribute.clone())),
				None => Ok(None)
			}
		}
	}

	#[inline(always)]
	fn execute(&self, _resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, handlebars: &HandlebarsWrapper, headerGenerator: &mut HeaderGenerator, languageData: &LanguageData, configuration: &Configuration, _rssChannelsToRssItems: &mut HashMap<Rc<RssChannelName>, Vec<RssItem>>, _siteMapWebPages: &mut Vec<SiteMapWebPage>) -> Result<Vec<PipelineResponse>, CordialError>
	{
		let inputCanonicalUrl = resourceUrl.url(languageData)?;

		let canBeCompressed = if self.can_be_compressed.is_none()
		{
			!inputContentFilePath.hasCompressedFileExtension()?
		}
		else
		{
			self.can_be_compressed.unwrap()
		};
		
		let mimeType = match self.mime_type
		{
			None => inputContentFilePath.guessMimeTypeWithCharacterSet()?,
			Some(ref mimeNewType) => mimeNewType.deref().clone(),
		};
		
		let headers = headerGenerator.generateHeadersForAsset(canBeCompressed, self.max_age_in_seconds, self.is_downloadable, &inputCanonicalUrl)?;
		
		let body = match mimeType.get_param(CHARSET)
		{
			Some(UTF_8) =>
			{
				let template = inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?;
				
				let body = HandlebarsTemplate
				{
					handlebars,
					configuration,
					iso639Dash1Alpha2Language: Some(languageData.iso639Dash1Alpha2Language),
					canBeCompressed,
					templateParameters: self.template_parameters.as_ref(),
				}.processNonHtmlTemplate(template)?;
				
				let body = if self.minify
				{
					// We do not minify CSS input, as it relies on (our) opinionated CSS parser.
					// We do not minify Javascript as there's no way to do this in pure Rust, for now.
					// The subtypes / suffixes zip, wbxml, cbor, (blank) and fastinfoset are binary
					match (mimeType.type_(), mimeType.subtype().as_str(), mimeType.suffix().map(|value| value.as_str()))
					{
						(TEXT, "xml", _) => minifyXml(body)?,
						(APPLICATION, "xml", _) => minifyXml(body)?,
						(_, _, Some("xml")) => minifyXml(body)?,
						
						(TEXT, "json", _) => Self::minifyJson(body)?,
						(APPLICATION, "json", _) => Self::minifyJson(body)?,
						(_, _, Some("json")) => Self::minifyJson(body)?,
						
						(APPLICATION, "json-seq", _) => Self::minifyJsonSeq(body)?,
						(_, _, Some("json-seq")) => Self::minifyJsonSeq(body)?,
						
						(TEXT, "html", _) => Self::minifyHtml(body, inputContentFilePath)?,
						
						_ => body.into_bytes(),
					}
				}
				else
				{
					body.into_bytes()
				};
				
				ResponseBody::utf8(body)
			}
			
			_ =>
			{
				if self.template_parameters.is_some()
				{
					return Err(CordialError::Configuration("Template parameters are only usable for textual resources which are UTF-8 encoded".to_owned()));
				}
				ResponseBody::binary(inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?)
			}
		};
		
		Ok(vec![(inputCanonicalUrl, hashmap! { default => Rc::new(UrlDataDetails::generic(&body)) }, self.status_code, ContentType(mimeType), headers, body, None, canBeCompressed)])
	}
}

impl TemplatePipeline
{
	#[inline(always)]
	fn minifyJson(body: String) -> Result<Vec<u8>, CordialError>
	{
		let json: ::serde_json::value::Value = ::serde_json::from_str(&body)?;
		Ok(::serde_json::to_vec(&json)?)
	}
	
	#[inline(always)]
	fn minifyJsonSeq(body: String) -> Result<Vec<u8>, CordialError>
	{
		let body = body.as_str();
		
		#[inline(always)]
		fn minifyJsonInSequence(mut writer: &mut Vec<u8>, _sequence: usize, body: &str, index: usize, sequenceStartIndex: usize) -> Result<usize, CordialError>
		{
			const AsciiRecordSeparator: &'static [u8] = b"\x1E";
			writer.write_all(AsciiRecordSeparator).unwrap();
			
			let jsonBytes = &body[sequenceStartIndex + 1 .. index];
			let json: ::serde_json::value::Value = ::serde_json::from_str(jsonBytes)?;
			::serde_json::to_writer(&mut writer, &json)?;
			
			const LineFeed: &'static [u8] = b"\x10";
			writer.write_all(LineFeed).unwrap();
			
			Ok(index + 1)
		}
		
		fn processNextJsonSequence(writer: &mut Vec<u8>, sequence: usize, body: &str, mut index: usize) -> Result<usize, CordialError>
		{
			let sequenceStartIndex = index;
			index += 1;
			let length = body.len();
			
			while index != length
			{
				const LineFeed: &'static str = "\x10";
				if &body[index .. index + 1] == LineFeed
				{
					return minifyJsonInSequence(writer, sequence, body, index, sequenceStartIndex);
				}
				index += 1;
			}
			Err(CordialError::Configuration(format!("JSON-SEQ sequence {} (zero-based) does not end with ASCII Line Feed (LF) control code", sequence)))
		}
		
		let mut writer = Vec::with_capacity(body.len());
		let mut sequence = 0;
		let mut index = 0;
		let length = body.len();
		while index != length
		{
			const AsciiRecordSeparator: &'static str = "\x1E";
			if &body[index .. index + 1] != AsciiRecordSeparator
			{
				return Err(CordialError::Configuration(format!("JSON-SEQ sequence {} (zero-based) does not start with ASCII Record Separator (RS) control code", sequence)));
			}
			index = processNextJsonSequence(&mut writer, sequence, body, index)?;
			sequence += 1;
		}
		
		writer.shrink_to_fit();
		Ok(writer)
	}
	
	#[inline(always)]
	fn minifyHtml(body: String, inputContentFilePath: &Path) -> Result<Vec<u8>, CordialError>
	{
		let bytes = body.into_bytes();
		let rcDom = RcDom::from_bytes_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type(bytes.as_slice(), inputContentFilePath)?;
		
		const html_head_and_body_tags_are_optional: bool = true;
		Ok(rcDom.minify_to_bytes(html_head_and_body_tags_are_optional))
	}
	
	#[inline(always)]
	fn minify_default() -> bool
	{
		true
	}
	
	#[inline(always)]
	fn status_code_default() -> StatusCode
	{
		StatusCode::Ok
	}
}
