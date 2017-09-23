// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug)]
	pub enum CordialError
	{
		CouldNotFindResourceContentFile(reason: String)
		{
			description("Could not find resource content file")
			display("Could not find resource content file because {}", reason)
		}
	
		Io(path: PathBuf, cause: ::std::io::Error)
		{
			cause(cause)
			description(cause.description())
			display("I/O error with {:?} was '{}'", path, cause)
			context(path: &'a Path, cause: ::std::io::Error) -> (path.to_path_buf(), cause)
			context(path: PathBuf, cause: ::std::io::Error) -> (path, cause)
			context(path: &'a PathBuf, cause: ::std::io::Error) -> (path.clone(), cause)
		}
		
		CouldNotRegisterHandlebarsTemplate(path: PathBuf, cause: ::handlebars::TemplateFileError)
		{
			cause(cause)
			description(cause.description())
			display("Handlebars template file register error with {:?} was '{}'", path, cause)
			context(path: &'a Path, cause: ::handlebars::TemplateFileError) -> (path.to_path_buf(), cause)
		}
		
		CouldNotCompressData(compressionAlgorithmName: &'static str, cause: ::std::io::Error)
		{
			cause(cause)
			description(cause.description())
			display("I/O whilst compressing data using {} was '{}'", compressionAlgorithmName, cause)
		}

		ConfigurationInputFilesAreInvalid(errors: Vec<String>)
		{
			description("One or more input files are invalid")
			display("Configuration input files are invalid because: {:?}", errors)
		}

		CouldNotParseUrl(reason: String, cause: ::url::ParseError)
		{
			cause(cause)
			description(&reason)
			display("Could not parse URL because: {}", reason)
			context(reason: String, cause: ::url::ParseError) -> (reason, cause)
		}
		
		// svgdom Error does not implement ::std::error::Error
		CouldNotParseSvg(path: PathBuf, parseError: ::svgdom::Error)
		{
			description("Could not parse SVG")
			display("Could not parse SVG in {:?} because '{:?}'", path, parseError)
		}
		
		CouldNotCleanSvg(path: PathBuf, cleanError: ::svgcleaner::Error)
		{
			description("Could not clean SVG")
			display("Could not clean SVG in {:?} because '{:?}'", path, cleanError)
		}
		
		HjsonDeserialization(path: PathBuf, cause: ::serde_hjson::error::Error)
		{
			cause(cause)
			description(cause.description())
			display("HJSON in {:?} could not be deserialized '{}'", path, cause)
			context(path: &'a Path, cause: ::serde_hjson::error::Error) -> (path.to_path_buf(), cause)
		}

		CouldNotDeserializeJson(cause: ::serde_json::error::Error)
		{
			cause(cause)
			description(cause.description())
			display("JSON (configuration) couldn't be deserialized because: {}", cause)
		}
		
		CouldNotStartDaemon(cause: ::daemonize::DaemonizeError)
		{
			cause(cause)
			description(cause.description())
			display("Could not start cordial daemon because: {}", cause)
			from()
		}

		InvalidFile(path: PathBuf, reason: String)
		{
			description(&reason)
			display("The file {:?} can not be used because: {}", path, reason)
		}

		CouldNotCompileSass(path: PathBuf, reason: String)
		{
			description(&reason)
			display("The file {:?} can not be compiled because: {}", path, reason)
		}
		
		Configuration(reason: String)
		{
			description(&reason)
			display("Configuration is invalid because: {}", reason)
		}

		BadImage(path: PathBuf, cause: ::image::ImageError)
		{
			cause(cause)
			description(cause.description())
			display("Image in {:?} could not be loaded because: '{}'", path, cause)
			context(path: &'a Path, cause: ::image::ImageError) -> (path.to_path_buf(), cause)
		}
		
		CouldNotEncodeTtfToWoff(path: PathBuf, cause: self::woff::WoffError)
		{
			cause(cause)
			description(cause.description())
			display("TTF data in {:?} could not be converted to WOFF because : '{}'", path, cause)
			context(path: &'a Path, cause: self::woff::WoffError) -> (path.to_path_buf(), cause)
		}
		
		CouldNotPerceptuallyEncodeJpegWithGuetzli(cause: ::guetzli_sys::GuetzliError)
		{
			cause(cause)
			description(cause.description())
			display("JPEG could not be perceptually encoded because: '{}'", cause)
			from()
		}
		
		CouldNotRenderHandlebarsTemplate(cause: ::handlebars::TemplateRenderError)
		{
			cause(cause)
			description(cause.description())
			display("Could not render handlebars template because: {}", cause)
			from()
		}
		
		CouldNotWriteXml(cause: ::xml::writer::Error)
		{
			cause(cause)
			description(cause.description())
			display("Could not write XML because: {}", cause)
			from()
		}
		
		Other(cause: Box<::std::error::Error>)
		{
			cause(&**cause)
			description(cause.description())
            from()
		}
	}
}

impl CordialError
{
	pub(crate) fn couldNotFindResourceContentFile<R>(resource: &Resource, primaryLanguage: &Language, language: Option<&Language>) -> Result<R, CordialError>
	{
		let resourceName = resource.name();
		let reason = match language
		{
			None => format!("there is no language neutral content for resource {:?}", resourceName),
			Some(language) => if primaryLanguage == language
			{
				format!("there is no primary language ({}) content for resource {:?}", primaryLanguage.iso_3166_1_alpha_2_country_code(), resourceName)
			}
			else
			{
				format!("there is no language ({}) or primary language ({}) content for resource {:?}", language.iso_3166_1_alpha_2_country_code(), primaryLanguage.iso_3166_1_alpha_2_country_code(), resourceName)
			},
		};
		
		Err(CordialError::CouldNotFindResourceContentFile(reason))
	}
	
//	pub(crate) fn executeCommandCapturingStandardOut(command: &mut Command, context: &Path, standardIn: Vec<u8>) -> Result<Vec<u8>, CordialError>
//	{
//		let mut child = command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().context(context)?;
//
//		{
//			let input = child.stdin.as_mut().unwrap();
//			let mut inputWriter = BufWriter::new(input);
//			inputWriter.write_all(&standardIn).context(context)?;
//		}
//
//		let output = child.wait_with_output().context(context)?;
//
//		let exitStatus = output.status;
//		match exitStatus.code()
//		{
//			None => Err(CordialError::InvalidFile(context.to_path_buf(), format!("command '{:?}' terminated by signal", command))),
//			Some(code) => if code == 0
//			{
//				Ok(output.stdout)
//			}
//			else
//			{
//				Err(CordialError::InvalidFile(context.to_path_buf(), format!("command '{:?}' exited with code {} and said: {}", command, code, String::from_utf8_lossy(&output.stderr))))
//			},
//		}
//	}
}
