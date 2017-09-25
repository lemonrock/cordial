// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
	pub(crate) fn toCss(option: Option<Self>, inputContentFilePath: &Path, precision: u8, configuration: &Configuration, handlebars: Option<&mut Handlebars>) -> Result<Vec<u8>, CordialError>
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
		format.processCss(inputContentFilePath, precision, configuration, handlebars)
	}
	
	#[inline(always)]
	fn processCss(&self, inputContentFilePath: &Path, precision: u8, configuration: &Configuration, handlebars: Option<&mut Handlebars>) -> Result<Vec<u8>, CordialError>
	{
		use self::CssInputFormat::*;
		
		match *self
		{
			css => Ok(inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?),
			sass => Self::toCssFromSassOrScss(inputContentFilePath, precision, configuration, handlebars, true),
			scss => Self::toCssFromSassOrScss(inputContentFilePath, precision, configuration, handlebars, false),
		}
	}
	
	#[inline(always)]
	fn toCssFromSassOrScss(inputContentFilePath: &Path, precision: u8, configuration: &Configuration, handlebars: Option<&mut Handlebars>, isSass: bool) -> Result<Vec<u8>, CordialError>
	{
		let options = ::sass_rs::Options
		{
			output_style: ::sass_rs::OutputStyle::Compressed,
			precision: precision as usize,
			indented_syntax: isSass,
			include_paths: Self::findSassImportPaths(configuration)?,
		};
		
		let content = inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?;
		let sassInput = if let Some(handlebars) = handlebars
		{
			handlebars.register_escape_fn(::handlebars::no_escape);
			let sassInput = handlebars.template_render(&content, &json!({}))?;
			handlebars.unregister_escape_fn();
			sassInput
		}
		else
		{
			content
		};
		
		match ::sass_rs::compile_string(&sassInput, options)
		{
			Err(error) => return Err(CordialError::CouldNotCompileSass(inputContentFilePath.to_path_buf(), error)),
			Ok(css) => Ok(css.as_bytes().to_owned()),
		}
	}
	
	#[inline(always)]
	fn findSassImportPaths(configuration: &Configuration) -> Result<Vec<String>, CordialError>
	{
		let sassFolderPath = &configuration.inputFolderPath;
		
		let mut importPaths = Vec::with_capacity(16);
		let sassImportsPath = sassFolderPath.join("sass-imports");
		for entry in sassImportsPath.read_dir().context(&sassImportsPath)?
		{
			let entry = entry.context(&sassImportsPath)?;
			
			let path = entry.path();
			
			if entry.file_type().context(&path)?.is_dir()
			{
				match path.into_os_string().into_string()
				{
					Err(_) => return Err(CordialError::InvalidFile(entry.path(), "a component of the path is not valid UTF-8".to_owned())),
					Ok(importPath) => importPaths.push(importPath),
				}
			}
		}
		
		Ok(importPaths)
	}
}
