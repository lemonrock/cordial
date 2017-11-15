// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// See: https://developers.google.com/search/reference/robots_txt
#[derive(Debug, Clone, Deserialize)]
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
	pub(crate) fn renderResource(&self, hostName: &str, relative_root_urls: &BTreeSet<Cow<'static, str>>, mixOfSiteMapAndSiteMapIndexUrls: &BTreeSet<Url>, primaryHostName: &str, handlebars: &mut Handlebars, configuration: &Configuration, newRespones: &mut Responses, oldResponses: &Arc<Responses>) -> Result<(), CordialError>
	{
		let mut bodyUncompressed = Vec::with_capacity(1024);
		self.writeTo(&mut bodyUncompressed, relative_root_urls, &mixOfSiteMapAndSiteMapIndexUrls, primaryHostName).context(PathBuf::from("robots.txt"))?;
		
		let robotsTxtUrl = Url::parse(&format!("https://{}/robots.txt", hostName)).unwrap();
		let headers = generateHeaders(handlebars, &self.headers, None, HtmlVariant::Canonical, configuration, true, self.max_age_in_seconds, true, &robotsTxtUrl).unwrap();
		
		let bodyCompressed = self.compression.compress(&bodyUncompressed)?;
		let response = StaticResponse::new(StatusCode::Ok, ContentType::plaintext(), headers, bodyUncompressed, Some(bodyCompressed));
		newRespones.addResponse(robotsTxtUrl, RegularAndPjaxStaticResponse::regular(response), oldResponses.clone());
		
		Ok(())
	}
	
	#[inline(always)]
	fn writeTo<W: Write>(&self, writer: &mut W, relative_root_urls: &BTreeSet<Cow<'static, str>>, mixOfSiteMapAndSiteMapIndexUrls: &BTreeSet<Url>, primaryHostName: &str) -> io::Result<()>
	{
		for relative_root_url in relative_root_urls.iter()
		{
			for group in self.groups.iter()
			{
				group.writeTo(writer, relative_root_url)?;
			}
		}
		
		for siteMap in mixOfSiteMapAndSiteMapIndexUrls.iter()
		{
			writer.write_all(b"Sitemap: ")?;
			writer.write_all(siteMap.as_str().as_bytes())?;
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
