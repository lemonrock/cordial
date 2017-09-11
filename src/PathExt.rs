// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of cordial, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait PathExt
{
	fn deleteOverridingPermissions(&self) -> io::Result<()>;
	
	fn createFolder(&self) -> io::Result<()>;
	
	fn createSubFolder(&self, subFolderName: &str) -> io::Result<PathBuf>;
	
	fn recreateSubFolder(&self, subFolderName: &str) -> io::Result<PathBuf>;
	
	fn makeUserOnlyWritableFolder(&self) -> io::Result<()>;
	
	fn makeUserOnlyReadableFile(&self) -> io::Result<()>;
	
	fn utf8FileName(&self) -> Result<String, CordialError>;
	
	fn configurationFilePath(&self) -> PathBuf;
	
	fn appendExtension<T: AsRef<OsStr>>(&self, extension: T) -> PathBuf;
	
	fn fileContentsAsBytes(&self) -> io::Result<Vec<u8>>;
	
	fn fileContentsAsBytesIfExtant(&self) -> io::Result<Option<Vec<u8>>>;
	
	fn fileContentsAsString(&self) -> io::Result<String>;
	
	fn fileContentsAsBufReader(&self, bufferSize: usize) -> io::Result<BufReader<File>>;
	
	fn fileContentsAsImage(&self, inputImageFormat: InputImageFormat) -> Result<::image::DynamicImage, CordialError>;
	
	fn fileContentsAsSvgDocument(&self) -> Result<(::svgdom::Document, String), CordialError>;
	
	fn fileContentsAsPemX509Certificates(&self) -> Result<Vec<Certificate>, CordialError>;
	
	fn fileContentsAsPemRsaPrivateKey(&self) -> Result<PrivateKey, CordialError>;
	
	fn createFileWithByteContents(&self, bytes: &[u8]) -> io::Result<()>;
	
	fn createFileWithStringContents(&self, string: &str) -> io::Result<()>;
	
	fn createFileWithPngImage(&self, image: ::image::DynamicImage) -> Result<(), CordialError>;
	
	fn createFileWithCleanedSvgFrom(&self, from: &Path) -> Result<(), CordialError>;
	
	fn createFileWithCopyOf(&self, from: &Path) -> io::Result<()>;
	
	fn modifyPngWithOxipng(&self) -> Result<(), CordialError>;
	
	fn createParentFolderForFilePath(&self) -> io::Result<()>;
}

impl PathExt for Path
{
	fn deleteOverridingPermissions(&self) -> io::Result<()>
	{
		let metadata = self.symlink_metadata()?;
		let fileType = metadata.file_type();
		let mut permissions = metadata.permissions();
		
		if fileType.is_symlink()
		{
			// Is is possible to set permissions on the symlink itself on BSD-derived systems
			//permissions.set_mode(0o666);
			//set_permissions(&self, permissions)?;
			remove_file(&self)
		}
		else
		{
			let path = self.canonicalize()?;
			if fileType.is_dir()
			{
				permissions.set_mode(0o777);
				set_permissions(&path, permissions)?;
			
				for entry in self.read_dir()?
				{
					let entry = entry?;
					entry.path().deleteOverridingPermissions()?;
				}
			
				remove_dir(&path)
			}
			else
			{
				// File, block device, char device, fifo or socket (Solaris Doors are not supported)
				permissions.set_mode(0o666);
				set_permissions(&path, permissions)?;
				remove_file(&path)
			}
		}
	}
	
	fn createFolder(&self) -> io::Result<()>
	{
		create_dir_all(self)?;
		self.makeUserOnlyWritableFolder()
	}
	
	fn createSubFolder(&self, subFolderName: &str) -> io::Result<PathBuf>
	{
		let subFolderPath = self.join(subFolderName);
		
		create_dir_all(&subFolderPath)?;
		subFolderPath.makeUserOnlyWritableFolder()?;
		Ok(subFolderPath)
	}
	
	fn recreateSubFolder(&self, subFolderName: &str) -> io::Result<PathBuf>
	{
		let subFolderPath = self.join(subFolderName);
		
		subFolderPath.deleteOverridingPermissions()?;
		
		self.createSubFolder(subFolderName)
	}
	
	fn makeUserOnlyWritableFolder(&self) -> io::Result<()>
	{
		let metadata = self.symlink_metadata()?;
		let mut permissions = metadata.permissions();
		permissions.set_mode(0o700);
		set_permissions(&self, permissions)
	}
	
	fn makeUserOnlyReadableFile(&self) -> io::Result<()>
	{
		let metadata = self.symlink_metadata()?;
		let mut permissions = metadata.permissions();
		permissions.set_mode(0o400);
		set_permissions(&self, permissions)
	}
	
	fn utf8FileName(&self) -> Result<String, CordialError>
	{
		match self.file_name().unwrap().to_str()
		{
			None => Err(CordialError::InvalidFile(self.to_path_buf(), "it has a file name which is not valid UTF-8".to_owned())),
			Some(fileName) => Ok(fileName.to_owned())
		}
	}
	
	fn appendExtension<T: AsRef<OsStr>>(&self, extension: T) -> PathBuf
	{
		let mut fileExtension = self.extension().unwrap().to_os_string();
		fileExtension.push(".");
		fileExtension.push(extension);
		self.with_extension(fileExtension)
	}
	
	fn configurationFilePath(&self) -> PathBuf
	{
		// Is there a configuration file?
		let originalFileName = self.file_name().unwrap();
		let mut configurationFileName = OsString::with_capacity(originalFileName.len() + 5);
		configurationFileName.push(originalFileName);
		configurationFileName.push(".toml");
		self.with_file_name(configurationFileName)
	}
	
	fn fileContentsAsBytes(&self) -> io::Result<Vec<u8>>
	{
		let metadata = self.metadata()?;
		
		let mut file = File::open(self)?;
		let mut bytes = Vec::with_capacity(metadata.len() as usize);
		file.read_to_end(&mut bytes)?;
		Ok(bytes)
	}
	
	fn fileContentsAsBytesIfExtant(&self) -> io::Result<Option<Vec<u8>>>
	{
		let mut file = match File::open(self)
		{
			Err(_) => return Ok(None),
			Ok(file) => file,
		};
		let metadata = self.metadata()?;
		let mut bytes = Vec::with_capacity(metadata.len() as usize);
		file.read_to_end(&mut bytes)?;
		Ok(Some(bytes))
	}
	
	fn fileContentsAsString(&self) -> io::Result<String>
	{
		let metadata = self.metadata()?;
		
		let mut file = File::open(self)?;
		let mut buffer = String::with_capacity(metadata.len() as usize);
		file.read_to_string(&mut buffer)?;
		Ok(buffer)
	}
	
	fn fileContentsAsBufReader(&self, bufferSize: usize) -> io::Result<BufReader<File>>
	{
		let file = File::open(self)?;
		Ok(BufReader::with_capacity(bufferSize, file))
	}
	
	fn fileContentsAsImage(&self, inputImageFormat: InputImageFormat) -> Result<::image::DynamicImage, CordialError>
	{
		let reader = self.fileContentsAsBufReader(4096).context(self)?;
		
		use InputImageFormat::*;
		use ::image::ImageFormat;
		let decoder = match inputImageFormat
		{
			PNG => ImageFormat::PNG,
			JPEG => ImageFormat::JPEG,
			GIF => ImageFormat::GIF,
			BMP => ImageFormat::BMP,
			ICO => ImageFormat::ICO,
			TIFF => ImageFormat::TIFF,
			WebP => ImageFormat::WEBP,
			PPM => ImageFormat::PPM,
			HDR => ImageFormat::HDR,
			TGA => ImageFormat::TGA,
		};
		
		Ok(::image::load(reader, decoder).context(self)?)
	}
	
	fn fileContentsAsSvgDocument(&self) -> Result<(::svgdom::Document, String), CordialError>
	{
		let svgString = self.fileContentsAsString().context(self)?;
		
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
		
		match ::svgcleaner::cleaner::parse_data(&svgString, &GenerousParseOptions)
		{
			Err(error) => Err(CordialError::CouldNotParseSvg(self.to_path_buf(), error)),
			Ok(document) => Ok((document, svgString)),
		}
	}
	
	fn fileContentsAsPemX509Certificates(&self) -> Result<Vec<Certificate>, CordialError>
	{
		let mut reader = self.fileContentsAsBufReader(4096).context(self)?;
		
		// certs() provides an error of '()'...
		::rustls::internal::pemfile::certs(&mut reader).map_err(|_| CordialError::InvalidFile(self.to_path_buf(), "Does not contain any PEM-encoded X.509 certificates".to_owned()))
	}
	
	fn fileContentsAsPemRsaPrivateKey(&self) -> Result<PrivateKey, CordialError>
	{
		let mut reader = self.fileContentsAsBufReader(4096).context(self)?;
		
		// rsa_private_keys() provides an error of '()'...
		let mut rsaPrivateKeys = ::rustls::internal::pemfile::rsa_private_keys(&mut reader).map_err(|_| CordialError::InvalidFile(self.to_path_buf(), "Does not contain any PEM-encoded RSA private keys".to_owned()))?;
		
		if rsaPrivateKeys.len() != 1
		{
			return Err(CordialError::InvalidFile(self.to_path_buf(), "Does not contain exactly one (1) PEM-encoded RSA private key".to_owned()));
		}
		
		let x = rsaPrivateKeys.drain(..).next().unwrap();
		Ok(x)
	}
	
	fn createParentFolderForFilePath(&self) -> io::Result<()>
	{
		self.parent().unwrap().createFolder()
	}
	
	fn createFileWithByteContents(&self, buffer: &[u8]) -> io::Result<()>
	{
		self.createParentFolderForFilePath()?;
		let mut file = File::create(self)?;
		file.write_all(buffer)
	}
	
	fn createFileWithStringContents(&self, string: &str) -> io::Result<()>
	{
		self.createFileWithByteContents(string.as_bytes())
	}
	
	fn createFileWithCopyOf(&self, from: &Path) -> io::Result<()>
	{
		self.createParentFolderForFilePath()?;
		fs::copy(from, self)?;
		Ok(())
	}
	
	fn createFileWithPngImage(&self, image: ::image::DynamicImage) -> Result<(), CordialError>
	{
		self.createParentFolderForFilePath().context(self)?;
		let mut writer = File::create(self).context(self)?;
		image.save(&mut writer, ::image::ImageFormat::PNG).context(self)?;
		Ok(())
	}
	
	fn createFileWithCleanedSvgFrom(&self, from: &Path) -> Result<(), CordialError>
	{
		use ::svgcleaner::CleaningOptions as SvgCleanOptions;
		use ::svgcleaner::cleaner::clean_doc as svgDocumentCleaner;
		
		self.createParentFolderForFilePath().context(self)?;
		
		let (document, svgString) = from.fileContentsAsSvgDocument()?;
		
		use ::svgdom::WriteOptions as SvgWriteOptions;
		use ::svgdom::WriteOptionsPaths as SvgWriteOptionsPaths;
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
			return Err(CordialError::CouldNotCleanSvg(self.to_path_buf(), error));
		}
		
		let mut buffer = Vec::with_capacity(svgString.len());
		::svgcleaner::cleaner::write_buffer(&document, &MinifyingWriteOptions, &mut buffer);
		
		// Write out the smaller of the original or cleaned
		if buffer.len() > svgString.len()
		{
			self.createFileWithStringContents(&svgString).context(self)?;
		}
		else
		{
			self.createFileWithByteContents(&buffer).context(self)?;
		}
		Ok(())
	}
	
	fn modifyPngWithOxipng(&self) -> Result<(), CordialError>
	{
		let num_cpus = num_cpus::get();
		let thread_count = num_cpus + (num_cpus >> 1);
		
		let options = ::oxipng::Options
		{
			backup: false,
			out_file: self.to_path_buf(),
			out_dir: None,
			stdout: false,
			fix_errors: true,
			pretend: false,
			recursive: false,
			clobber: true,
			create: true,
			force: false,
			preserve_attrs: true,
			verbosity: None,
			filter: hashset!
			{
				0,
				1,
				2,
				3,
				4,
				5
			},
			interlace: None,
			compression: hashset!
			{
				9
			},
			memory: hashset!
			{
				9
			},
			strategies: hashset!
			{
				0,
				1,
				2,
				3
			},
			window: 15,
			bit_depth_reduction: true,
			color_type_reduction: true,
			palette_reduction: true,
			idat_recoding: true,
			strip: ::oxipng::headers::Headers::Safe,
			deflate: ::oxipng::deflate::Deflaters::Zopfli,
			use_heuristics: false,
			threads: thread_count,
		};
		
		match ::oxipng::optimize(self, &options)
		{
			Err(error) => Err(CordialError::InvalidFile(self.to_path_buf(), format!("{}", error))),
			Ok(()) => Ok(())
		}
	}
}
