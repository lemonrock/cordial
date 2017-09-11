// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) enum pipeline
{
	md
	{
		#[serde(default = "pipeline::max_age_in_seconds_none_default")] max_age_in_seconds: u32,
		#[serde(default)] is_leaf: bool,
		
		abstracts: HashMap<String, Abstract>,
		/*
		{
			en:
			{
				title: String,
				description: String,
				extract: String, // markdown / handlebars template
			}
		}
		*/
		
		// status-code overrides
			// 301 - Moved Perm (OLD)
			// 302 - Moved Temp (OLD)
			// 303 - See Other
			// 307 - Moved Temp (HTTP/1.1)
			// 308 - Moved Perm (HTTP 2 / HTTP rewrite)
	},
	raster_image
	{
		#[serde(default = "pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		input_format: InputImageFormat,
		jpeg_quality: Option<u8>,
		#[serde(default)] transformations: Vec<ImageTransformation>,
	},
	sass
	{
		#[serde(default = "pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		#[serde(default = "pipeline::cssDefaultPrecision")] precision: u8,
	},
	scss
	{
		#[serde(default = "pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		#[serde(default = "pipeline::cssDefaultPrecision")] precision: u8,
	},
	svg
	{
		#[serde(default = "pipeline::max_age_in_seconds_long_default")] max_age_in_seconds: u32,
		#[serde(default = "pipeline::is_downloadable_false_default")] is_downloadable: bool,
		#[serde(default = "pipeline::is_versioned_true_default")] is_versioned: bool,
		#[serde(default)] language_aware: bool,
		skip: bool,
	}
//	sitemap, // xml  https://github.com/netvl/xml-rs
//	robots,
//	rss, // xml
//	json,
//	// js,
//	gif (animations only),
//  favicon
//	redirect,
	// eg for temp or perm redirect
	// empty body
	// Needs access to site configuration in order to write out the 'Location' header
	// header field values are visible US-ASCII, ie 32 - 126 incl
}

impl pipeline
{
	fn cssDefaultPrecision() -> u8
	{
		5
	}
	
	#[inline(always)]
	fn max_age_in_seconds_none_default() -> u32
	{
		0
	}
	
	#[inline(always)]
	fn max_age_in_seconds_long_default() -> u32
	{
		31536000
	}
	
	#[inline(always)]
	fn is_downloadable_false_default() -> bool
	{
		false
	}
	
	#[inline(always)]
	fn is_versioned_true_default() -> bool
	{
		true
	}
	
	pub(crate) fn resourceInputContentFileNamesWithExtension(&self, resourceInputName: &str) -> Vec<String>
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
					withExtension.push_str(fileExtension);
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
	
	pub(crate) fn resourceOutputRelativeUrl(&self, parentHierarchy: &[String], resourceInputName: &str) -> (String, Option<&'static str>)
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
	
	pub(crate) fn isFor<'a>(&self, deploymentVersion: &'a str) -> (u32, bool, bool, bool, ContentType, bool, Option<&'a str>)
	{
		use self::pipeline::*;
		match *self
		{
			md { max_age_in_seconds, .. } => (max_age_in_seconds, false, false, true, ContentType::html(), false, None),
			raster_image { max_age_in_seconds, is_downloadable, is_versioned, language_aware, jpeg_quality, .. } =>
			{
				let contentType = if jpeg_quality.is_some()
				{
					ContentType::jpeg()
				}
				else
				{
					ContentType::png()
				};
				(max_age_in_seconds, language_aware, true, false, contentType, is_downloadable, if is_versioned { Some(deploymentVersion) } else { None })
			}
			sass { max_age_in_seconds, is_downloadable, is_versioned, language_aware, .. } => (max_age_in_seconds, language_aware, true, true, ContentType(TEXT_CSS), is_downloadable, if is_versioned { Some(deploymentVersion) } else { None }),
			scss { max_age_in_seconds, is_downloadable, is_versioned, language_aware, .. } => (max_age_in_seconds, language_aware, true, true, ContentType(TEXT_CSS), is_downloadable, if is_versioned { Some(deploymentVersion) } else { None }),
			svg { max_age_in_seconds, is_downloadable, is_versioned, language_aware, .. } => (max_age_in_seconds, language_aware, true, true, ContentType(Mime::from_str("image/svg+xml").unwrap()), is_downloadable, if is_versioned { Some(deploymentVersion) } else { None }),
		}
	}
	
	pub(crate) fn execute(&self, inputContentFilePath: &Path, _variant: Variant, inputFolderPath: &Path) -> Result<Vec<u8>, CordialError>
	{
		use self::pipeline::*;
		match *self
		{
			md { .. } =>
			{
				panic!("Implement me");
			}
			
			raster_image { input_format, jpeg_quality, ref transformations, .. } => Self::raster_image(inputContentFilePath, input_format, jpeg_quality, transformations),
			
			sass { precision, .. } => Self::sass_or_scss(inputContentFilePath, precision, inputFolderPath, true),
			
			scss { precision, .. } => Self::sass_or_scss(inputContentFilePath, precision, inputFolderPath, false),
			
			svg { skip, .. } =>
			{
				if skip
				{
					Ok(inputContentFilePath.fileContentsAsBytes().context(inputContentFilePath)?)
				}
				else
				{
					inputContentFilePath.fileContentsAsACleanedSvgFrom()
				}
			}
		}
	}
	
	fn raster_image(inputContentFilePath: &Path, input_format: InputImageFormat, jpeg_quality: Option<u8>, transformations: &[ImageTransformation]) -> Result<Vec<u8>, CordialError>
	{
		let image = inputContentFilePath.fileContentsAsImage(input_format)?;
		
		// transform
		let image = ImageTransformation::applyTransformations(image, transformations);
		
		// save & optimize
		if jpeg_quality.is_some()
		{
			let quality = match jpeg_quality.unwrap()
			{
				0 => 1,
				quality @ 0 ... 100 => quality,
				_ => 100
			};
			
			// create PNG bytes
			let mut pngBytes = Vec::with_capacity(128 * 1024);
			{
				let mut writer = BufWriter::with_capacity(pngBytes.len(), &mut pngBytes);
				image.save(&mut writer, ::image::ImageFormat::PNG).context(inputContentFilePath)?;
			}
			
			// create JPEG
			Ok(CordialError::executeCommandCapturingStandardOut(Command::new("guetzli").env_clear().args(&["--nomemlimit", "--quality", &format!("{}", quality), "-", "-"]), inputContentFilePath, pngBytes)?)
		}
		else
		{
			let mut temporaryFile = Temp::new_file().context(inputContentFilePath)?;
			let temporaryFilePath = temporaryFile.to_path_buf();
			
			temporaryFilePath.createFileWithPngImage(image)?;
			temporaryFilePath.modifyPngWithOxipng()?;
			
			let bytes = temporaryFilePath.fileContentsAsBytes().context(&temporaryFilePath)?;
			
			temporaryFilePath.deleteOverridingPermissions().context(&temporaryFilePath)?;
			temporaryFile.release();
			
			Ok(bytes)
		}
	}
	
	fn sass_or_scss(inputContentFilePath: &Path, precision: u8, inputFolderPath: &Path, isSass: bool) -> Result<Vec<u8>, CordialError>
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
						Err(_) => return Err(CordialError::InvalidFile(entry.path(), "a component of the path is not valid UTF-8".to_owned())),
						Ok(importPath) => importPaths.push(importPath),
					}
				}
			}
			
			Ok(importPaths)
		}
		
		let options = ::sass_rs::Options
		{
			output_style: ::sass_rs::OutputStyle::Compressed,
			precision: precision as usize,
			indented_syntax: isSass,
			include_paths: findImportPaths(inputFolderPath)?,
		};
		
		match ::sass_rs::compile_file(inputContentFilePath, options)
		{
			Err(error) => return Err(CordialError::CouldNotCompileSass(inputContentFilePath.to_path_buf(), error)),
			Ok(css) => Ok(css.as_bytes().to_owned()),
		}
	}
}
