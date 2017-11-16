// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn generateHeaders(handlebars: &mut Handlebars, headerTemplates: &HashMap<String, String>, languageData: Option<&LanguageData>, htmlVariant: HtmlVariant, configuration: &Configuration, canBeCompressed: bool, maximumAge: u32, isDownloadable: bool, url: &Url) -> Result<Vec<(String, String)>, CordialError>
{
	let localization = &configuration.localization;
	let deploymentVersion = &configuration.deploymentVersion;
	
	let mut headers = Vec::with_capacity(headerTemplates.len() * 2);
	
	let isPjax = htmlVariant == HtmlVariant::PJAX;
	
	let vary = if isPjax
	{
		headers.push(("X-PJAX-Version".to_owned(), format!("{}", deploymentVersion)));
		
		if canBeCompressed
		{
			Some("content-encoding, x-pjax")
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
	
	if maximumAge == 0
	{
		headers.push(("Cache-Control".to_owned(), "no-cache".to_owned()))
	}
	else
	{
		headers.push(("Cache-Control".to_owned(), format!("max-age={}; no-transform; immutable", maximumAge)))
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
	headers.push(("Content-Disposition".to_owned(), format!("{}; filename*=utf-8''{}", variant, utf8_percent_encode(&fileNameUtf8, USERINFO_ENCODE_SET))));
	
	let (ourLanguage, otherLanguages) = match languageData
	{
		None => (None, None),
		Some(&LanguageData { iso639Dash1Alpha2Language, language }) =>
		{
			headers.push(("Content-Language".to_owned(), iso639Dash1Alpha2Language.to_iso_639_1_alpha_2_language_code().to_owned()));
			
			let mut ourLanguage = HashMap::with_capacity(2);
			ourLanguage.insert("iso639Dash1Alpha2Language", iso639Dash1Alpha2Language.to_iso_639_1_alpha_2_language_code());
			ourLanguage.insert("iso_3166_1_alpha_2_country_code", language.iso3166Dash1Alpha2CountryCode().to_iso_3166_1_alpha_2_language_code());
			(Some(ourLanguage), Some(localization.otherLanguages(iso639Dash1Alpha2Language)))
		}
	};
	
	handlebars.register_escape_fn(::handlebars::no_escape);
	{
		for (headerName, headerTemplate) in headerTemplates.iter()
		{
			let json = &json!
			({
				"environment": &configuration.environment,
				"html_variant": htmlVariant,
				"our_language": ourLanguage,
				"localization": localization,
				"other_languages": otherLanguages,
				"can_be_compressed": canBeCompressed,
				"deployment_date": configuration.deploymentDate,
				"deployment_version": deploymentVersion,
				"current_headers": &headers,
				
				"header": headerName,
			});
			
			let headerValue = handlebars.template_render(headerTemplate, &json)?;
			if !headerName.is_ascii()
			{
				return Err(CordialError::Configuration(format!("Non-ASCII header name '{}' for {}", headerName, url)))
			}
			if !headerValue.is_ascii()
			{
				return Err(CordialError::Configuration(format!("Non-ASCII header value '{}' for header name '{}' for {}", headerValue, headerName, url)))
			}
			headers.push((headerName.to_owned(), headerValue));
		}
	}
	handlebars.unregister_escape_fn();
	
	headers.shrink_to_fit();
	Ok(headers)
}
