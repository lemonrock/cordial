// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum CssInputFormat
{
	sass,
	scss,
	css,
}

impl Default for CssInputFormat
{
	#[inline(always)]
	fn default() -> Self
	{
		CssInputFormat::scss
	}
}

impl InputFormat for CssInputFormat
{
	#[inline(always)]
	fn fileExtensions(&self) -> &'static [&'static str]
	{
		use self::CssInputFormat::*;
		
		match *self
		{
			sass => &[".sass"],
			scss => &[".scss"],
			css => &[".css"],
		}
	}
	
	#[inline(always)]
	fn allFileExtensions() -> &'static [&'static str]
	{
		&[
			".sass",
			".scss",
			".css",
		]
	}
}

impl CssInputFormat
{
	#[inline(always)]
	pub(crate) fn toCss(option: Option<Self>, inputContentFilePath: &Path, precision: u8, handlebarsTemplate: &HandlebarsTemplate, maximum_release_age_from_can_i_use_database_last_updated_in_weeks: u16, minimum_usage_threshold: UsagePercentage, regional_usages: &[RegionalUsages]) -> Result<Vec<u8>, CordialError>
	{
		let format = match option
		{
			Some(format) => format,
			None =>
			{
				use self::CssInputFormat::*;
				
				match inputContentFilePath.extension().unwrap().to_str().unwrap()
				{
					"sass" => sass,
					"scss" => scss,
					"css" => css,
					_ => panic!("How is this possible?"),
				}
			}
		};
		
		let raw = inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?;
		
		let input = handlebarsTemplate.processNonHtmlTemplate(raw)?;
		
		let cssString = format.processCss(inputContentFilePath, precision, handlebarsTemplate.configuration, input)?;
		
		Self::validateCssAndAutoprefix(inputContentFilePath, &cssString, maximum_release_age_from_can_i_use_database_last_updated_in_weeks, minimum_usage_threshold, regional_usages)
	}
	
	#[inline(always)]
	fn processCss(&self, inputContentFilePath: &Path, precision: u8, configuration: &Configuration, input: String) -> Result<String, CordialError>
	{
		use self::CssInputFormat::*;
		
		match *self
		{
			css => Ok(input),
			sass => Self::toCssFromSassOrScss(inputContentFilePath, precision, configuration, &input, true),
			scss => Self::toCssFromSassOrScss(inputContentFilePath, precision, configuration, &input, false),
		}
	}
	
	#[inline(always)]
	fn validateCssAndAutoprefix(inputContentFilePath: &Path, cssString: &str, maximum_release_age_from_can_i_use_database_last_updated_in_weeks: u16, minimum_usage_threshold: UsagePercentage, regional_usages: &[RegionalUsages]) -> Result<Vec<u8>, CordialError>
	{
		match Stylesheet::parse(cssString)
		{
			Err(error) => Err(CordialError::InvalidFile(inputContentFilePath.to_path_buf(), format!("CSS '{:?}' at line (one-based) {:?}, text {}", error.error, error.location, error.slice))),
			Ok(mut stylesheet) =>
			{
				let regional_usages: Vec<&RegionalUsage> = regional_usages.iter().map(|regional_usages| regional_usages.regional_usage()).collect();
				let (can_i_use, choices) = sensible_choices(maximum_release_age_from_can_i_use_database_last_updated_in_weeks, minimum_usage_threshold, &regional_usages);
				autoprefix_stylesheet(&mut stylesheet, &can_i_use, &choices);
				Ok(stylesheet.to_bytes(false))
			}
		}
	}
	
	#[inline(always)]
	fn toCssFromSassOrScss(inputContentFilePath: &Path, precision: u8, configuration: &Configuration, sassInput: &str, isSass: bool) -> Result<String, CordialError>
	{
		let options = ::sass_rs::Options
		{
			output_style: ::sass_rs::OutputStyle::Compressed,
			precision: precision as usize,
			indented_syntax: isSass,
			include_paths: configuration.findSassImportPaths()?,
		};
		
		match ::sass_rs::compile_string(&sassInput, options)
		{
			Err(error) => return Err(CordialError::CouldNotCompileSass(inputContentFilePath.to_path_buf(), error)),
			Ok(css) => Ok(css),
		}
	}
}
