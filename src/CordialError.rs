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
		
		CouldNotRenderHandlebarsTemplate(cause: ::handlebars::TemplateRenderError)
		{
			cause(cause)
			description(cause.description())
			display("Could not render handlebars template because: {}", cause)
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
	pub fn couldNotFindResourceContentFile<R>(resource: &resource, primaryLanguage: &language, language: Option<&language>) -> Result<R, CordialError>
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
	
	pub fn executeCommandCapturingOnlyStandardError(command: &mut Command, context: &Path) -> Result<(), CordialError>
	{
		let output = command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).output().context(context)?;
		let exitStatus = output.status;
		match exitStatus.code()
		{
			None => Err(CordialError::InvalidFile(context.to_path_buf(), format!("command '{:?}' terminated by signal", command))),
			Some(code) => if code == 0
			{
				Ok(())
			}
			else
			{
				Err(CordialError::InvalidFile(context.to_path_buf(), format!("command '{:?}' exited with code {} and said: {}", command, code, String::from_utf8_lossy(&output.stderr))))
			},
		}
	}
}
