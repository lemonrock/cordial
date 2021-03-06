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
		CouldNotParseSvg(parseError: ::svgdom::Error)
		{
			description("Could not parse SVG")
			display("Could not parse SVG because '{:?}'", parseError)
			from()
		}
		
		CouldNotParseMp3(cause: ::mp3_metadata::Error)
		{
			cause(cause)
			description("Could not parse MP3")
			display("Could not parse MP3 because {:?}", cause)
			from()
		}
		
		// mp4parse Error does not implement ::std::error::Error
		CouldNotParseMp4(parseError: ::mp4parse::Error)
		{
			description("Could not parse MP4")
			display("Could not parse MP4 because '{:?}'", parseError)
			from()
		}
		
		CouldNotCleanSvg(cleanError: ::svgcleaner::Error)
		{
			description("Could not clean SVG")
			display("Could not clean SVG because '{:?}'", cleanError)
			from()
		}
		
		CouldNotParseMonArtistText(error: ::mon_artist::grid::ParseError)
		{
			description("Could not parse MonArtist text")
			display("Could not parse MonArtist text because '{:?}'", error)
			from()
		}
		
		CouldNotExecuteLuaCode(error: ::hlua::LuaError)
		{
			description("Could not execute Lua code")
			display("Could not execute Lua code because '{:?}'", error)
			from()
		}
		
		HjsonDeserialization(path: PathBuf, cause: ::serde_hjson::error::Error)
		{
			cause(cause)
			description(cause.description())
			display("HJSON in {:?} could not be deserialized '{}'", path, cause)
			context(path: &'a Path, cause: ::serde_hjson::error::Error) -> (path.to_path_buf(), cause)
		}
	
		CouldNotUseJson(cause: ::serde_json::Error)
		{
			cause(cause)
			description(cause.description())
			display("JSON couldn't be used because: {}", cause)
			from()
		}
	
		CouldNotSerializeJson(cause: ::serde_json::error::Error)
		{
			cause(cause)
			description(cause.description())
			display("JSON couldn't be serialized because: {}", cause)
		}
	
		CouldNotDeserializeJson(cause: ::serde_json::error::Error)
		{
			cause(cause)
			description(cause.description())
			display("JSON couldn't be deserialized because: {}", cause)
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

		CouldNotCompileSass(path: PathBuf, cause: ::sass_alt::SassCompileError)
		{
			cause(cause)
			description(cause.description())
			display("The file {:?} can not be compiled from SASS to CSS because: {}", path, &cause)
			context(path: &'a Path, cause: ::sass_alt::SassCompileError) -> (path.to_path_buf(), cause)
		}
		
		CouldNotConvertToCString(cause: ::std::ffi::NulError)
		{
			cause(cause)
			description(cause.description())
			display("Could not convert to CString because: {}", &cause)
			from()
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
		
		CouldNotRenderHandlebars(cause: ::handlebars::RenderError)
		{
			cause(cause)
			description(cause.description())
			display("Could not render handlebars because: {}", cause)
			from()
		}
		
		CouldNotRenderHandlebarsTemplate(cause: ::handlebars::TemplateRenderError)
		{
			cause(cause)
			description(cause.description())
			display("Could not render handlebars template because: {}", cause)
			from()
		}
		
		CouldNotReadXml(cause: ::xml::reader::Error)
		{
			cause(cause)
			description(cause.description())
			display("Could not read XML because: {}", cause)
			from()
		}
		
		CouldNotWriteXml(cause: ::xml::writer::Error)
		{
			cause(cause)
			description(cause.description())
			display("Could not write XML because: {}", cause)
			from()
		}
		
		CouldNotEncodeBarcode(cause: ::barcoders::error::Error)
		{
			cause(cause)
			description(cause.description())
			display("Could not encode barcode because: {}", cause)
			from()
		}
		
		CouldNotCreateQrCode(cause: ::qrcode::types::QrError)
		{
			display("Could not create QR code because: {}", cause)
			from()
		}
		
		CouldNotFormatMarkdownToHtml
		{
			description("could not format markdown to HTML")
			display("Could not format markdown to HTML")
		}
		
		BadHtml(cause: ::css_purify::html5ever_ext::HtmlError)
		{
			cause(cause)
			description(cause.description())
			display("Could not read HTML because: {}", cause)
			from()
		}
		
		SelfReferencingResource(cause: ::std::cell::BorrowError)
		{
			cause(cause)
			description(cause.description())
			display("Resource borrows a resource that directly or indirectly references itself, eg an HTML page with a image resource url which is itself; {}", cause)
			from()
		}
		
		SelfReferencingMutResource(cause: ::std::cell::BorrowMutError)
		{
			cause(cause)
			description(cause.description())
			display("Resource borrows mutably a resource that directly or indirectly references itself, eg an HTML page with a image resource url which is itself; {}", cause)
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

unsafe impl Send for CordialError
{
}

impl CordialError
{
	pub(crate) fn couldNotFindResourceContentFile<R>(resource: &Resource, primaryLanguage: Iso639Dash1Alpha2Language, language: Option<Iso639Dash1Alpha2Language>) -> Result<R, CordialError>
	{
		let resourceName = resource.name();
		let reason = match language
		{
			None => format!("there is no language neutral content for resource {:?}", resourceName),
			Some(language) => if primaryLanguage == language
			{
				format!("there is no primary language ({:?}) content for resource {:?}", primaryLanguage, resourceName)
			}
			else
			{
				format!("there is no language ({:?}) or primary language ({:?}) content for resource {:?}", language, primaryLanguage, resourceName)
			},
		};
		
		Err(CordialError::CouldNotFindResourceContentFile(reason))
	}
}
