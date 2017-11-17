// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of cordial, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait PathExt
{
	fn fileContentsInFolder<R, F: Fn(&Path) -> Option<Result<R, CordialError>>>(&self, filter: F) -> Result<BTreeMap<PathBuf, R>, CordialError>;
	
	fn deleteOverridingPermissions(&self) -> io::Result<()>;
	
	fn createFolder(&self) -> io::Result<()>;
	
	fn createSubFolder(&self, subFolderName: &str) -> io::Result<PathBuf>;
	
	fn recreateSubFolder(&self, subFolderName: &str) -> io::Result<PathBuf>;
	
	fn makeUserOnlyWritableFolder(&self) -> io::Result<()>;
	
	fn makeUserOnlyReadableFile(&self) -> io::Result<()>;
	
	fn utf8FileName(&self) -> Result<String, CordialError>;
	
	fn configurationFilePath(&self) -> PathBuf;
	
	fn appendExtension<T: AsRef<OsStr>>(&self, extension: T) -> PathBuf;
	
	fn hasCompressedFileExtension(&self) -> Result<bool, CordialError>;
	
	fn guessMimeTypeWithCharacterSet(&self) -> Result<Mime, CordialError>;
	
	#[inline(always)]
	fn appendCharacterSetToMimeType(&self, mimeWithoutCharacterSet: &'static str) -> Result<Mime, CordialError>;
	
	fn detectCharacterSetUpperCase(&self) -> io::Result<String>;
	
	fn registerAllHandlebarsTemplates(&self, prefix: &Path, handlebars: &mut Handlebars) -> Result<(), CordialError>;
	
	fn fileContentsAsBytes(&self) -> io::Result<Vec<u8>>;
	
	fn fileContentsAsBytesIfExtant(&self) -> io::Result<Option<Vec<u8>>>;
	
	fn fileContentsAsString(&self) -> io::Result<String>;
	
	fn fileContentsAsBufReader(&self, bufferSize: usize) -> io::Result<BufReader<File>>;
	
	fn fileContentsAsHandlebarsTemplate(&self, prefix: &Path, handlebars: &mut Handlebars) -> Result<(), CordialError>;
	
	fn fileContentsAsImage(&self, imageInputFormat: ImageInputFormat) -> Result<::image::DynamicImage, CordialError>;
	
	fn fileContentsAsPemX509Certificates(&self) -> Result<Vec<Certificate>, CordialError>;
	
	fn fileContentsAsPemRsaPrivateKey(&self) -> Result<PrivateKey, CordialError>;
	
	fn createFileWithByteContents(&self, bytes: &[u8]) -> io::Result<()>;
	
	fn createFileWithStringContents(&self, string: &str) -> io::Result<()>;
	
	fn createFileWithPngImage(&self, image: &::image::DynamicImage) -> Result<(), CordialError>;
	
	fn createFileWithCopyOf(&self, from: &Path) -> io::Result<()>;
	
	fn modifyPngWithOxipng(&self) -> Result<(), CordialError>;
	
	fn createParentFolderForFilePath(&self) -> io::Result<()>;
}

impl PathExt for Path
{
	fn fileContentsInFolder<R, F: Fn(&Path) -> Option<Result<R, CordialError>>>(&self, filter: F) -> Result<BTreeMap<PathBuf, R>, CordialError>
	{
		let mut matchedFilePaths = BTreeMap::new();
		for entry in self.read_dir().context(self)?
		{
			let entry = entry.context(self)?;
			let path = entry.path();
			
			let metadata = entry.metadata().context(&path)?;
			if metadata.is_file()
			{
				match filter(&path)
				{
					None => (),
					Some(value) =>
					{
						matchedFilePaths.insert(path, value?);
					}
				}
			}
		}
		Ok((matchedFilePaths))
	}
	
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
	
	//noinspection SpellCheckingInspection
	fn hasCompressedFileExtension(&self) -> Result<bool, CordialError>
	{
		match self.extension()
		{
			None => Err(CordialError::InvalidFile(self.to_path_buf(), "No file extension".to_owned())),
			Some(extension) => match extension.to_str()
			{
				None => Err(CordialError::InvalidFile(self.to_path_buf(), "File extension is not UTF-8".to_owned())),
				Some(extension) => Ok(CompressedFileExtensions.contains(extension)),
			}
		}
	}
	
	fn guessMimeTypeWithCharacterSet(&self) -> Result<Mime, CordialError>
	{
		// ::mime::APPLICATION_OCTET_STREAM
		let mimeWithoutCharacterSetString = match self.extension()
		{
			None => "text/html",
			Some(extension) => match extension.to_str()
			{
				// Non-UTF-8 file extension
				None => "application/octet-stream",
				Some(extension) =>
				{
					if extension.is_empty()
					{
						"text/html"
					}
					else
					{
						match ::mime_guess::get_mime_type_str(extension)
						{
							None => "application/octet-stream",
							Some(value) => value,
						}
					}
				}
			}
		};
		
		let mimeWithoutCharacterSet: Mime = mimeWithoutCharacterSetString.parse().unwrap();
		match mimeWithoutCharacterSet.suffix()
		{
			Some(mime::XML) => self.appendCharacterSetToMimeType(mimeWithoutCharacterSetString),
			_ =>
			{
				match mimeWithoutCharacterSet.subtype()
				{
					mime::XML => self.appendCharacterSetToMimeType(mimeWithoutCharacterSetString),
					mime::JSON => self.appendCharacterSetToMimeType(mimeWithoutCharacterSetString),
					mime::JAVASCRIPT => self.appendCharacterSetToMimeType(mimeWithoutCharacterSetString),
					_ => match mimeWithoutCharacterSet.type_()
					{
						mime::TEXT => self.appendCharacterSetToMimeType(mimeWithoutCharacterSetString),
						_ => Ok(mimeWithoutCharacterSet),
					}
				}
			}
		}
	}
	
	#[inline(always)]
	fn appendCharacterSetToMimeType(&self, mimeWithoutCharacterSet: &'static str) -> Result<Mime, CordialError>
	{
		let characterSet = self.detectCharacterSetUpperCase().context(self)?;
		
		let mime = if characterSet.as_str() == "US-ASCII"
		{
			format!("{}; charset=utf-8", mimeWithoutCharacterSet).parse().unwrap()
		}
		else
		{
			format!("{}; charset={}", mimeWithoutCharacterSet, characterSet.to_ascii_lowercase()).parse().unwrap()
		};
		Ok(mime)
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
	
	fn detectCharacterSetUpperCase(&self) -> io::Result<String>
	{
		use ::chardet::*;
		
		let metadata = self.metadata()?;
		let numberOfBytesToReadToHaveAHighConfidence = min(metadata.len() as usize, 4096);
		
		let mut bufferReader = self.fileContentsAsBufReader(numberOfBytesToReadToHaveAHighConfidence)?;
		let bytes = bufferReader.fill_buf()?;
		// Unpleasant API requires copy
		let (characterSetUpperCase, _confidence, _languageEmptyIfNoLanguageRelevant) = detect(&bytes.to_vec());
		//let encodingCrateCharacterSet = charset2encoding(&characterSetUpperCase);
		
		Ok(characterSetUpperCase)
	}
	
	fn registerAllHandlebarsTemplates(&self, prefix: &Path, handlebars: &mut Handlebars) -> Result<(), CordialError>
	{
		for entry in self.read_dir().context(self)?
		{
			let entry = entry.context(self)?;
			let path = entry.path();
			
			let metadata = entry.metadata().context(&path)?;
			if metadata.is_dir()
			{
				path.registerAllHandlebarsTemplates(prefix, handlebars)?
			}
			else if metadata.is_file()
			{
				if let Some(osStrExtension) = path.extension()
				{
					if let Some(utf8Extension) = osStrExtension.to_str()
					{
						if utf8Extension == "hbs" || utf8Extension == "handlebars"
						{
							path.fileContentsAsHandlebarsTemplate(prefix, handlebars)?
						}
					}
				}
			}
		}
		Ok(())
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
	
	fn fileContentsAsHandlebarsTemplate(&self, prefix: &Path, handlebars: &mut Handlebars) -> Result<(), CordialError>
	{
		let relativeEntryPath = self.strip_prefix(prefix).unwrap();
		match relativeEntryPath.to_str()
		{
			None => Err(CordialError::InvalidFile(self.to_path_buf(), "File path is not encoded in UTF-8".to_owned())),
			Some(name) =>
			{
				handlebars.unregister_template(name);
				Ok(handlebars.register_template_file(name, self).context(self)?)
			}
		}
	}
	
	fn fileContentsAsImage(&self, imageInputFormat: ImageInputFormat) -> Result<::image::DynamicImage, CordialError>
	{
		let reader = self.fileContentsAsBufReader(4096).context(self)?;
		
		use ImageInputFormat::*;
		use ::image::ImageFormat;
		let decoder = match imageInputFormat
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
			Targa => ImageFormat::TGA,
		};
		
		Ok(::image::load(reader, decoder).context(self)?)
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
	
	fn createFileWithPngImage(&self, image: &::image::DynamicImage) -> Result<(), CordialError>
	{
		self.createParentFolderForFilePath().context(self)?;
		let mut writer = File::create(self).context(self)?;
		image.save(&mut writer, ::image::ImageFormat::PNG).context(self)?;
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

// List derived from Wikipedia https://en.wikipedia.org/wiki/List_of_archive_formats , September 14, 2017
// - Includes obsolete, obscure and proprietary formats
// - With long range rzip (lrz) and brotli (br, bro)
// - With as many tarball suffixes as I could find.. (including gtar)
// - With common compressed image formats
// - With common compressed font formats (woff, woff2)
static CompressedFileExtensions: PhfSet<&'static str> = phf_set!
{
	"7z",
	"ace",
	"afa",
	"alz",
	"apk",
	"arc",
	"arj",
	"b1",
	"ba",
	"bh",
	"br",
	"bro",
	"bz",
	"bz2",
	"cab",
	"car",
	"cfs",
	"cpt",
	"dar",
	"dd",
	"dgc",
	"dmg",
	"ear",
	"F",
	"gca",
	
	"gif",
	"gtar",
	
	"gz",
	"ha",
	"hki",
	"ice",
	"jar",
	
	"jpe",
	"jpeg",
	"jpg",
	
	"kgb",
	"lha",
	"lrz",
	"lz",
	"lzh",
	"lzma",
	"lzo",
	"lzx",
	"pak",
	
	"png",
	
	"partimg",
	"paq6",
	"paq7",
	"paq8",
	"pea",
	"pim",
	"pit",
	"qda",
	"rar",
	"rk",
	"rz",
	"s7z",
	"sda",
	"sea",
	"sen",
	"sfark",
	"sfx",
	"shk",
	"sit",
	"sitx",
	"sqx",
	"sz",
	
	"svgz",
	
	"tgz",
	"tb2",
	"tbz",
	"tbz2",
	"tlz",
	"txz",
	"tZ",
	
	"uc",
	"uc0",
	"uc2",
	"uca",
	"ucn",
	"ue2",
	"uha",
	"ur2",
	"war",
	
	"webm",
	"webp",
	"woff",
	"woff2",
	
	"wim",
	"xar",
	"xp3",
	"xz",
	"yz1",
	"Z",
	"z",
	"zip",
	"zipx",
	"zoo",
	"zpaq",
	"zz",
};
