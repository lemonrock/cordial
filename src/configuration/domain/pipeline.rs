// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub enum pipeline
{
	md
	{
		#[serde(default)] is_leaf: bool,
		
		// TODO: Needs to be reworked to be per-language
		title: String,
		description: String,
		extract_markdown: String, // markdown
	},
	raster_image
	{
		input_format: InputImageFormat,
		
		jpeg_quality: Option<u8>,
		
		#[serde(default)] transformations: Vec<ImageTransformation>,
	},
	sass
	{
		#[serde(default = "pipeline::cssDefaultPrecision")] precision: u8,
	},
	scss
	{
		#[serde(default = "pipeline::cssDefaultPrecision")] precision: u8,
	},
	svg
	{
		skip: bool,
	}
//	sitemap, // xml
//	robots,
//	rss, // xml
//	json,
//	// js,
//	png,
//	jpeg,
//	gif,
//	svg,
//	redirect,
	// eg for temp or perm redirect
	// empty body
	// 301 - Moved Perm (OLD)
	// 302 - Moved Temp (OLD)
	// 303 - See Other
	// 307 - Moved Temp (HTTP/1.1)
	// 308 - Moved Perm (HTTP 2 / HTTP rewrite)
	// Needs access to site configuration in order to write out the 'Location' header
	// header field values are visible US-ASCII, ie 32 - 126 incl
}

impl pipeline
{
	fn cssDefaultPrecision() -> u8
	{
		5
	}
	
	pub fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
	{
		let mut result = Vec::with_capacity(4);
		
		use self::pipeline::*;
		match *self
		{
			md { ..} =>
			{
				for fileExtension in vec![".markdown", ".md"]
				{
					let mut withExtension = String::with_capacity(resourceInputName.len() + 3);
					withExtension.push_str(resourceInputName);
					withExtension.push_str(".md");
					result.push(withExtension);
				}
			}
			raster_image { input_format, .. } =>
			{
				let first = resourceInputName.rmatch_indices(".").next();
				
				for fileExtension in input_format.fileExtensions()
				{
					let index = first.unwrap().0;
					let mut withExtension = String::with_capacity(index + fileExtension.len());
					let slice = if first.is_some()
					{
						&resourceInputName[0..index]
					}
					else
					{
						resourceInputName
					};
					withExtension.push_str(slice);
					withExtension.push_str(fileExtension);
					
					result.push(withExtension);
				}
			}
			
			_ => result.push(resourceInputName.to_owned()),
		}
		
		result
	}
	
	pub fn resourceOutputRelativeUrl(&self, parentHierarchy: &[String], resourceInputName: &str) -> (String, Option<&'static str>)
	{
		let mut relativeUrl = String::with_capacity(1024);
		for parent in parentHierarchy
		{
			relativeUrl.push_str(parent);
			relativeUrl.push('/');
		}
		
		use self::pipeline::*;
		let additionalContentFileNameIfAny = match *self
		{
			md { is_leaf, .. } =>
			{
				relativeUrl.push_str(resourceInputName);
				
				if !is_leaf
				{
					relativeUrl.push('/');
					Some("index.html")
				}
				else
				{
					None
				}
			}
			raster_image { jpeg_quality, .. } =>
			{
				let first = resourceInputName.rmatch_indices(".").next();
				let index = first.unwrap().0;
				
				let slice = if first.is_some()
				{
					&resourceInputName[0..index]
				}
				else
				{
					resourceInputName
				};
				relativeUrl.push_str(slice);
				
				let fileExtension = if jpeg_quality.is_none()
				{
					".png"
				}
				else
				{
					".jpg"
				};
				relativeUrl.push_str(fileExtension);
				
				None
			}
			_ =>
			{
				relativeUrl.push_str(resourceInputName);
				None
			}
		};
		
		(relativeUrl, additionalContentFileNameIfAny)
	}
	
	pub fn execute(&self, resource: &resource, primaryLanguage: &language, language: &language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash: Option<&str>, siteOutputFolderPath: &Path, canonicalizedInputFolderPath: &Path) -> Result<(), CordialError>
	{
		use self::pipeline::*;
		match *self
		{
			md { .. } =>
			{
				panic!("Implement me");
			}
			
			raster_image { input_format, jpeg_quality, ref transformations } => if let Some(languageIndependentInputContentFilePath) = languageIndependentInputContentFilePath(resource, primaryLanguage, language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash)?
			{
				Self::raster_image(resource, primaryLanguage, languageIndependentInputContentFilePath, siteOutputFolderPath, input_format, jpeg_quality, transformations)
			}
			else
			{
				Ok(())
			},
			
			sass { precision } => if let Some(languageIndependentInputContentFilePath) = languageIndependentInputContentFilePath(resource, primaryLanguage, language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash)?
			{
				Self::sass_or_scss(resource, primaryLanguage, languageIndependentInputContentFilePath, siteOutputFolderPath, precision, canonicalizedInputFolderPath, false)
			}
			else
			{
				Ok(())
			},
			
			scss { precision } => if let Some(languageIndependentInputContentFilePath) = languageIndependentInputContentFilePath(resource, primaryLanguage, language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash)?
			{
				Self::sass_or_scss(resource, primaryLanguage, languageIndependentInputContentFilePath, siteOutputFolderPath, precision, canonicalizedInputFolderPath, true)
			}
			else
			{
				Ok(())
			},
			
			svg { skip } => if let Some(languageIndependentInputContentFilePath) = languageIndependentInputContentFilePath(resource, primaryLanguage, language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash)?
			{
				resource.createFinalResourceContent(primaryLanguage, None, siteOutputFolderPath, |outputFilePath|
				{
					if skip
					{
						outputFilePath.createFileWithCopyOf(&languageIndependentInputContentFilePath).context(outputFilePath)?
					}
					else
					{
						outputFilePath.createFileWithCleanedSvgFrom(&languageIndependentInputContentFilePath)?
					}
					
					Ok(vec![outputFilePath])
				})
			}
			else
			{
				Ok(())
			},
		}
	}
	
	fn raster_image(resource: &resource, primaryLanguage: &language, inputContentFilePath: PathBuf, siteOutputFolderPath: &Path, input_format: InputImageFormat, jpeg_quality: Option<u8>, transformations: &[ImageTransformation]) -> Result<(), CordialError>
	{
		let mut image = inputContentFilePath.fileContentsAsImage(input_format)?;
		
		// transform
		let image = ImageTransformation::applyTransformations(image, transformations);
		
		// save & optimize
		resource.createFinalResourceContent(primaryLanguage, None, siteOutputFolderPath, |outputFilePath|
		{
			if jpeg_quality.is_some()
			{
				let temporaryFile = Temp::new_file().context(&outputFilePath)?;
				let temporaryFilePath = temporaryFile.to_path_buf();
				temporaryFilePath.createFileWithPngImage(image)?;
				
				let quality = match jpeg_quality.unwrap()
				{
					0 => 1,
					quality @ 0 ... 100 => quality,
					_ => 100
				};
				
				CordialError::executeCommandCapturingOnlyStandardError(Command::new("guetzli").env_clear().args(&["--nomemlimit", "--quality", &format!("{}", quality)]).arg(&temporaryFilePath).arg(&outputFilePath), &outputFilePath)?;
				
				temporaryFilePath.deleteOverridingPermissions();
				
				temporaryFile.release();
			}
			else
			{
				outputFilePath.createFileWithPngImage(image)?;
				
				outputFilePath.modifyPngWithOxipng()?;
			}
			
			Ok(vec![outputFilePath])
		})
	}
	
	fn sass_or_scss(resource: &resource, primaryLanguage: &language, inputContentFilePath: PathBuf, siteOutputFolderPath: &Path, precision: u8, canonicalizedInputFolderPath: &Path, isSass: bool) -> Result<(), CordialError>
	{
		fn findImportPaths(sassFolderPath: &Path) -> Result<Vec<String>, CordialError>
		{
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
						Err(_) => return Err(CordialError::InvalidFile(path, "a component of the path is not valid UTF-8".to_owned())),
						Ok(importPath) => importPaths.push(importPath),
					}
				}
			}
			Ok(importPaths)
		}
		
		let importPaths = findImportPaths(&canonicalizedInputFolderPath)?;
		
		let options = ::sass_rs::Options
		{
			output_style: ::sass_rs::OutputStyle::Compressed,
			precision: precision as usize,
			indented_syntax: isSass,
			include_paths: importPaths,
		};
		
		resource.createFinalResourceContent(primaryLanguage, None, siteOutputFolderPath, |outputFilePath|
		{
			match ::sass_rs::compile_file(inputContentFilePath, options)
			{
				Err(error) => return Err(CordialError::CouldNotCompile(inputContentFilePath, error)),
				Ok(css) => outputFilePath.createFileWithStringContents(&css).context(&outputFilePath)?,
			}
			Ok(vec![outputFilePath])
		})
	}
}

#[inline(always)]
fn languageIndependentInputContentFilePath(resource: &resource, primaryLanguage: &language, language: &language, variantSubPathEgAcceleratedMobilePagesWithTrailingSlash: Option<&str>) -> Result<Option<PathBuf>, CordialError>
{
	if language != primaryLanguage
	{
		return Ok(None);
	}
	
	if variantSubPathEgAcceleratedMobilePagesWithTrailingSlash.is_some()
	{
		return Ok(None);
	}
	
	resource.inputContentFilePath(primaryLanguage, None).map(|path| Some(path))
}
