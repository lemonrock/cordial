// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// See: https://developers.google.com/search/reference/robots_txt
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct RobotsTxt
{
	#[serde(default)] headers: HashMap<String, String>,
	#[serde(default = "RobotsTxt::max_age_in_seconds_default")] max_age_in_seconds: u32,
	#[serde(default)] compression: Compression,
	#[serde(default)] groups: Vec<RobotGroup>,
	#[serde(default)] generate_yandex_primary_host: bool,
}

impl Default for RobotsTxt
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			headers: HashMap::default(),
			max_age_in_seconds: 60,
			compression: Compression::default(),
			groups: vec![RobotGroup::default()],
			generate_yandex_primary_host: true,
		}
	}
}

impl RobotsTxt
{
	#[inline(always)]
	pub(crate) fn renderRobotsTxt(&self, hostName: &str, robotsTxtConfiguration: &RobotsTxtConfiguration, primaryHostName: &str, handlebars: &HandlebarsWrapper, configuration: &Configuration, newResponses: &mut Responses, oldResponses: &Arc<Responses>) -> Result<(), CordialError>
	{
		let mut bodyUncompressed = Vec::with_capacity(1024);
		self.writeTo(&mut bodyUncompressed, robotsTxtConfiguration, primaryHostName).context(PathBuf::from("robots.txt"))?;
		
		let robotsTxtUrl = Url::parse(&format!("https://{}/robots.txt", hostName)).unwrap();
		
		const CanBeCompressed: bool = true;
		const CanBeDownloaded: bool = true;
		let headers = HeaderGenerator
		{
			handlebars,
			headerTemplates: &self.headers,
			ifLanguageAwareLanguageData: None,
			configuration,
		}.generateHeadersForAsset(CanBeCompressed, self.max_age_in_seconds, CanBeDownloaded, &robotsTxtUrl)?;
		
		let bodyCompressed = self.compression.compress(&bodyUncompressed)?;
		let response = StaticResponse::new(StatusCode::Ok, content_type_text_plain_utf8(), headers, ResponseBody::utf8(bodyUncompressed), Some(bodyCompressed));
		newResponses.addResponse(robotsTxtUrl, RegularAndPjaxStaticResponse::regular(response), oldResponses.clone());
		
		Ok(())
	}
	
	#[inline(always)]
	fn writeTo<W: Write>(&self, writer: &mut W, robotsTxtConfiguration: &RobotsTxtConfiguration, primaryHostName: &str) -> io::Result<()>
	{
		for relativeUrlPathForRobotDirective in robotsTxtConfiguration.relativeUrlPathsForRobotDirective.iter()
		{
			for group in self.groups.iter()
			{
				group.writeTo(writer, relativeUrlPathForRobotDirective)?;
			}
		}
		
		for siteMapIndexUrl in robotsTxtConfiguration.siteMapIndexUrls.iter()
		{
			writer.write_all(b"Sitemap: ")?;
			writer.write_all(siteMapIndexUrl.as_str().as_bytes())?;
			writer.write_all(b"\n")?;
		}
		if self.generate_yandex_primary_host
		{
			writer.write_all(b"Host: ")?;
			writer.write_all(primaryHostName.as_bytes())?;
			writer.write_all(b"\n")?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn max_age_in_seconds_default() -> u32
	{
		60
	}
}
