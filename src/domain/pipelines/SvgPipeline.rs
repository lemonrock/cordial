// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct SvgPipeline
{
	#[serde(default = "max_age_in_seconds_long_default")] max_age_in_seconds: u32,
	#[serde(default = "is_downloadable_false_default")] is_downloadable: bool,
	#[serde(default = "is_versioned_true_default")] is_versioned: bool,
	#[serde(default)] language_aware: bool,
	#[serde(default)] input_format: SvgInputFormat,
	
	#[serde(default)] metadata: ImageMetaData,
	
	// Exists solely because of potential bugs in svg optimizer
	#[serde(default)] do_not_optimize: bool,
	
	// Responsive tips: https://useiconic.com/guides/using-iconic-responsively
	// SVG can be an 'icon-stack' (ie multiple images in one file), typically with less complexity for smaller sizes
	// Or individual image files, with width/height pre-set
	
	#[serde(default, skip_deserializing)] primaryImageDimensions: Cell<(u32, u32)>,
}

impl Default for SvgPipeline
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
			input_format: Default::default(),
			metadata: Default::default(),
			do_not_optimize: false,
			primaryImageDimensions: Default::default(),
		}
	}
}

impl Pipeline for SvgPipeline
{
	#[inline(always)]
	fn imageMetaData(&self) -> Result<&ImageMetaData, CordialError>
	{
		Ok(&self.metadata)
	}
	
	#[inline(always)]
	fn addToImgAttributes(&self, attributes: &mut Vec<Attribute>) -> Result<(), CordialError>
	{
		let dimensions = self.primaryImageDimensions.get();
		if dimensions.0 != 0
		{
			attributes.push("width".u32_attribute(dimensions.0));
		}
		if dimensions.1 != 0
		{
			attributes.push("height".u32_attribute(dimensions.1));
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn processingPriority(&self) -> ProcessingPriority
	{
		NoDependenciesEgImage
	}
	
	#[inline(always)]
	fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		self.input_format.resourceInputContentFileNamesWithExtension(resourceInputName)
	}
	
	#[inline(always)]
	fn is<'a>(&self) -> (bool, bool)
	{
		(self.is_versioned, self.language_aware)
	}
	
	#[inline(always)]
	fn execute(&self, _resources: &Resources, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: &LanguageData, ifLanguageAwareLanguageData: Option<&LanguageData>, configuration: &Configuration, _siteMapWebPages: &mut Vec<SiteMapWebPage>, _rssItems: &mut Vec<RssItem>) -> Result<Vec<(Url, HashMap<ResourceTag, Rc<UrlDataDetails>>, StatusCode, ContentType, Vec<(String, String)>, Vec<u8>, Option<(Vec<(String, String)>, Vec<u8>)>, bool)>, CordialError>
	{
		let url = resourceUrl.replaceFileNameExtension(".svg").url(languageData)?;
		
		let svgString = self.input_format.svgString(inputContentFilePath, resourceUrl, configuration)?;
	
		let document = Self::parseSvg(&svgString)?;
		
		const CanBeCompressed: bool = true;
		let headers = generateHeaders(handlebars, headerTemplates, ifLanguageAwareLanguageData, HtmlVariant::Canonical, configuration, CanBeCompressed, self.max_age_in_seconds, self.is_downloadable, &url)?;
		
		let width = Self::svgDimensionInPixels(&document, "width").unwrap_or(0);
		let height = Self::svgDimensionInPixels(&document, "height").unwrap_or(0);
		
		self.primaryImageDimensions.set((width, height));
		
		let body = if self.do_not_optimize
		{
			svgString.into_bytes()
		}
		else
		{
			Self::clean(document, svgString)?
		};
		
		let mimeType = mimeType("image/svg+xml");
		
		let urlDataDetails = Rc::new
		(
			UrlDataDetails::Image
			{
				width,
				height,
				mime: mimeType.clone(),
				size: body.len() as u64,
			}
		);
		
		let tags = hashmap!
		{
			default => urlDataDetails.clone(),
			
			smallest_image => urlDataDetails.clone(),
			largest_image => urlDataDetails.clone(),
			primary_image => urlDataDetails.clone(),
			width_image(width) => urlDataDetails.clone(),
			height_image(height) => urlDataDetails.clone(),
			width_height_image(width, height) => urlDataDetails.clone(),
		};
		
		Ok(vec![(url, tags, StatusCode::Ok, ContentType(mimeType), headers, body, None, CanBeCompressed)])
	}
}

impl SvgPipeline
{
	#[inline(always)]
	fn parseSvg(svgString: &str) -> Result<::svgdom::Document, CordialError>
	{
		use ::svgcleaner::ParseOptions as SvgParseOptions;
		static GenerousParseOptions: SvgParseOptions = SvgParseOptions
		{
			parse_comments: true,
			parse_declarations: true,
			parse_unknown_elements: true,
			parse_unknown_attributes: true,
			parse_px_unit: true,
			skip_unresolved_classes: false,
		};
		
		match ::svgcleaner::cleaner::parse_data(svgString, &GenerousParseOptions)
		{
			Err(error) => Err(CordialError::CouldNotParseSvg(error)),
			Ok(document) => Ok(document),
		}
	}
	
	fn clean(document: ::svgdom::Document, svgString: String) -> Result<Vec<u8>, CordialError>
	{
		use ::svgcleaner::CleaningOptions as SvgCleanOptions;
		use ::svgcleaner::cleaner::clean_doc as svgDocumentCleaner;
		use ::svgdom::WriteOptions as SvgWriteOptions;
		use ::svgdom::WriteOptionsPaths as SvgWriteOptionsPaths;
		use ::svgcleaner::cleaner::write_buffer;
		
		static MinifyingWriteOptions: SvgWriteOptions = SvgWriteOptions
		{
			indent: ::svgdom::Indent::None,
			use_single_quote: false,
			trim_hex_colors: true,
			write_hidden_attributes: false,
			remove_leading_zero: true,
			paths: SvgWriteOptionsPaths
			{
				use_compact_notation: true,
				join_arc_to_flags: false,  // Apparently this optimisation is not properly implemented by some SVG viewers
				remove_duplicated_commands: true,
				use_implicit_lineto_commands: true,
			},
			simplify_transform_matrices: true,
		};
		
		// NOTE: write options aren't used by this method but are required...
		if let Err(error) = svgDocumentCleaner(&document, &SvgCleanOptions::default(), &MinifyingWriteOptions)
		{
			return Err(CordialError::CouldNotCleanSvg(error));
		}
		
		let mut buffer = Vec::with_capacity(svgString.len());
		write_buffer(&document, &MinifyingWriteOptions, &mut buffer);
		
		// Write out the smaller of the original or cleaned
		let result = if buffer.len() > svgString.len()
		{
			svgString.as_bytes().to_owned()
		}
		else
		{
			buffer
		};
		
		Ok(result)
	}
	
	fn svgDimensionInPixels(document: &::svgdom::Document, attributeName: &str) -> Option<u32>
	{
		use ::svgdom::AttributeValue;
		use ::svgdom::types::Length;
		use ::svgdom::types::LengthUnit;
		
		let root = document.root();
		let attributes = root.attributes();
		match attributes.get_value(attributeName)
		{
			None => None,
			Some(&AttributeValue::Length(Length { num, unit })) => if num > 0.0
			{
				match unit
				{
					LengthUnit::None | LengthUnit::Px => Some(num as u32),
					LengthUnit::In => Some((num * 96.0) as u32),
					LengthUnit::Cm => Some((num * 96.0 / 2.54) as u32),
					LengthUnit::Mm => Some((num * 9.6 / 2.54) as u32),
					LengthUnit::Pt => Some((num * 16.0) as u32),
					LengthUnit::Pc => Some((num * 96.0/72.0) as u32),
					_ => None,
				}
			}
			else
			{
				None
			},
			_ => None,
		}
	}
}
