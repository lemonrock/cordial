// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct HeaderGenerator<'a>
{
	pub(crate) handlebars: &'a HandlebarsWrapper,
	pub(crate) headerTemplates: &'a HashMap<String, String>,
	pub(crate) ifLanguageAwareLanguageData: Option<&'a LanguageData<'a>>,
	pub(crate) configuration: &'a Configuration,
}

impl<'a> HeaderGenerator<'a>
{
	#[inline(always)]
	pub(crate) fn generateHeadersForAsset(&mut self, canBeCompressed: bool, maximumAgeInSeconds: u32, isDownloadable: bool, url: &Url) -> Result<Vec<(String, String)>, CordialError>
	{
		self.generateHeaders(false, canBeCompressed, maximumAgeInSeconds, isDownloadable, url)
	}
	
	#[inline(always)]
	pub(crate) fn generateHeaders(&mut self, isPjax: bool, canBeCompressed: bool, maximumAgeInSeconds: u32, isDownloadable: bool, url: &Url) -> Result<Vec<(String, String)>, CordialError>
	{
		let deploymentVersion = &self.configuration.deploymentVersion;
		
		let mut headers = Vec::with_capacity(self.headerTemplates.len() * 2);
		
		let vary = if isPjax
		{
			headers.push(("X-PJAX-Version".to_owned(), format!("{}", deploymentVersion)));
			
			if canBeCompressed
			{
				Some("content-encoding,x-pjax")
			}
			else
			{
				Some("x-pjax")
			}
		}
		else
		{
			if canBeCompressed
			{
				Some("content-encoding")
			}
			else
			{
				None
			}
		};
		if let Some(vary) = vary
		{
			headers.push(("Vary".to_owned(), vary.to_owned()));
		}
		
		if maximumAgeInSeconds == 0
		{
			headers.push(("Cache-Control".to_owned(), "no-cache".to_owned()))
		}
		else
		{
			headers.push(("Cache-Control".to_owned(), format!("max-age={};no-transform;immutable", maximumAgeInSeconds)))
		}
		
		let fileNameUtf8 = url.fileNameOrIndexNamePercentDecodedUntrusted(".html").to_owned();
		let variant = if isDownloadable
		{
			"attachment"
		}
		else
		{
			"inline"
		};
		headers.push(("Content-Disposition".to_owned(), format!("{};filename*=utf-8''{}", variant, utf8_percent_encode(&fileNameUtf8, USERINFO_ENCODE_SET))));
		
		let iso639Dash1Alpha2Language = match self.ifLanguageAwareLanguageData
		{
			None => None,
			Some(&LanguageData { iso639Dash1Alpha2Language, .. }) =>
			{
				headers.push(("Content-Language".to_owned(), iso639Dash1Alpha2Language.to_iso_639_1_alpha_2_language_code().to_owned()));
				Some(iso639Dash1Alpha2Language)
			}
		};
		
		self.handlebars.renderWithEscapeFunction(::handlebars::no_escape, |templateRenderer|
		{
			for (headerName, headerTemplate) in self.headerTemplates.iter()
			{
				let mut templateParameters = JsonMap::with_capacity(1);
				templateParameters.insert("headers".to_owned(), json!(&headers));
				
				let headerValue = HandlebarsTemplate
				{
					handlebars: self.handlebars,
					configuration: self.configuration,
					iso639Dash1Alpha2Language,
					canBeCompressed,
					templateParameters: Some(&templateParameters),
				}.processHttpTemplate(&templateRenderer, headerName, headerTemplate)?;
				
				headers.push((headerName.to_owned(), headerValue));
			}
			Ok(())
		})?;
		
		headers.shrink_to_fit();
		Ok(headers)
	}
}
