// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Clone)]
pub(crate) struct HandlebarsTemplate<'a>
{
	pub(crate) handlebars: &'a HandlebarsWrapper,
	pub(crate) configuration: &'a Configuration,
	pub(crate) iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>,
	pub(crate) canBeCompressed: bool,
	pub(crate) templateParameters: Option<&'a JsonMap<String, JsonValue>>,
}

impl<'a> HandlebarsTemplate<'a>
{
	#[inline(always)]
	pub(crate) fn processNonHtmlTemplate(&self, raw: String) -> Result<String, CordialError>
	{
		if let Some(ref templateParameters) = self.templateParameters
		{
			let json = &json!
			({
				"configuration": self.configuration,
				"iso_639_1 alpha_2_language_code": self.iso639Dash1Alpha2Language,
				"template_parameters": templateParameters,
				"can_be_compressed": self.canBeCompressed,
			});
			
			self.handlebars.renderWithEscapeFunction(::handlebars::no_escape, |templateRenderer| templateRenderer.template_render(&raw, json))
		}
		else
		{
			Ok(raw)
		}
	}
	
	#[inline(always)]
	pub(crate) fn processHttpTemplate(&self, templateRenderer: &HandlebarsTemplateRenderer, headerName: &str, headerTemplate: &str) -> Result<String, CordialError>
	{
		if !headerName.is_ascii()
		{
			return Err(CordialError::Configuration(format!("Non-ASCII header name '{}'", headerName)));
		}
		
		let json = &json!
		({
			"configuration": self.configuration,
			"iso_639_1 alpha_2_language_code": self.iso639Dash1Alpha2Language,
			"template_parameters": self.templateParameters,
			"can_be_compressed": self.canBeCompressed,
		});
		
		let headerValue = templateRenderer.template_render(headerTemplate, &json)?;
		
		if headerValue.is_ascii()
		{
			Ok(headerValue)
		}
		else
		{
			Err(CordialError::Configuration(format!("Non-ASCII header value '{}' for header name '{}'", headerValue, headerName)))
		}
	}
}
