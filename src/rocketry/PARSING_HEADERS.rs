// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//	pub fn new(uncompressedContentFilePath: &Path) -> Result<Self, CordialError>
//	{
//		let mut uncompressedTemplateResponse = Response::new();
//
//		{
//			let headersFilePath = uncompressedContentFilePath.appendExtension("headers");
//			let mut headersFile = File::open(headersFilePath).context(&headersFilePath)?;
//			let mut lineIndex = 0;
//			for mut line in headersFile.lines()
//			{
//				let line = line.context(&headersFilePath)?;
//				match line.find(':')
//				{
//					None => return Err(CordialError::CouldNotReadHeaders(headersFilePath, format!("could not find headers separator (colon, :) at zero-based line {}", line))),
//					Some(index) =>
//					{
//						let headerValue = line.split_at(index);
//						let headerName = line;
//						if headerName.is_empty()
//						{
//							return Err(CordialError::CouldNotReadHeaders(headersFilePath, format!("there is an empty header name at zero-based line {}", line)));
//						}
//						if headerValue.is_empty()
//						{
//							return Err(CordialError::CouldNotReadHeaders(headersFilePath, format!("there is an empty header value at zero-based line {}", line)));
//						}
//						uncompressedTemplateResponse.adjoin_raw_header(headerName, headerValue);
//					}
//				}
//			}
//		}
//
//		let mut gzipTemplateResponse = Response::new();
//		{
//			gzipTemplateResponse.merge()
//
//			let gzipCompressedContentFilePath = uncompressedContentFilePath.appendExtension("gz");
//			if gzipCompressedContentFilePath.exists()
//			{
//
//			}
//		}
//
//		let brotliCompressedContentFilePath = uncompressedContentFilePath.appendExtension("br");
//
//		// TODO: PJAX. Consider whether we should do it using file extensions
//
//
//
//
//
//		Self
//		{
//			uncompressedTemplateResponse
//		}
//	}

