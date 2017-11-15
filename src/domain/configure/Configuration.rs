// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Configuration
{
	#[serde(default)] daemon: Daemon,
	#[serde(default = "Configuration::maximum_number_of_tls_sessions_default")] maximum_number_of_tls_sessions: u32,
	#[serde(default = "Configuration::http_keep_alive_default")] http_keep_alive: bool,
	#[serde(default)] enable_hsts_preloading_for_production: bool,
	#[serde(default, skip_deserializing)] resource_template: Option<HjsonValue>,
	#[serde(default)] pub(crate) localization: Localization,
	#[serde(default)] robots: RobotsTxt,
	#[serde(default)] site_map: SiteMap,
	#[serde(default)] rss: Option<RssChannel>,
	#[serde(default)] google_analytics: Option<String>,
	#[serde(default, skip_deserializing)] pub(crate) inputFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] outputFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] pub(crate) environment: String,
	#[serde(default = "Configuration::deploymentDate_default", skip_deserializing)] pub(crate) deploymentDate: SystemTime,
	#[serde(default, skip_deserializing)] pub(crate) deploymentVersion: String,
}

impl Default for Configuration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			daemon: Daemon::default(),
			maximum_number_of_tls_sessions: Self::maximum_number_of_tls_sessions_default(),
			http_keep_alive: Self::http_keep_alive_default(),
			enable_hsts_preloading_for_production: false,
			resource_template: None,
			localization: Localization::default(),
			robots: RobotsTxt::default(),
			site_map: SiteMap::default(),
			rss: None,
			google_analytics: None,
			inputFolderPath: PathBuf::default(),
			outputFolderPath: PathBuf::default(),
			environment: String::default(),
			deploymentDate: Self::deploymentDate_default(),
			deploymentVersion: String::default(),
		}
	}
}

impl Configuration
{
	#[inline(always)]
	pub(crate) fn resourceTemplate(&self) -> HjsonValue
	{
		self.resource_template.as_ref().unwrap().clone()
	}
	
	#[inline(always)]
	pub(crate) fn daemonizeAndBindSockets(&self, isDaemon: bool) -> Result<(::std::net::TcpListener, ::std::net::TcpListener), CordialError>
	{
		self.daemon.daemonizeAndBindSockets(&self.outputFolderPath, isDaemon)
	}
	
	#[inline(always)]
	pub(crate) fn reconfigure(environment: &str, inputFolderPath: &Path, outputFolderPath: &Path, oldResponses: Arc<Responses>) -> Result<(ServerConfig, HttpsStaticRequestHandler, HttpRedirectToHttpsRequestHandler, Self), CordialError>
	{
		Self::validateInputFiles(inputFolderPath)?;
		let configuration = Configuration::loadBaselineConfiguration(&inputFolderPath, environment, outputFolderPath)?;
		
		configuration.finishReconfigure(oldResponses)
	}
	
	#[inline(always)]
	pub(crate) fn primary_iso_639_1_alpha_2_language_code(&self) -> &str
	{
		&self.localization.primary_iso_639_1_alpha_2_language_code
	}
	
	#[inline(always)]
	fn finishReconfigure(self, oldResponses: Arc<Responses>) -> Result<(ServerConfig, HttpsStaticRequestHandler, HttpRedirectToHttpsRequestHandler, Self), CordialError>
	{
		let ourHostNames = self.ourHostNames()?;
		
		Ok
		(
			(
				self.tlsServerConfiguration()?,
				self.httpsStaticRequestHandler(&ourHostNames, oldResponses)?,
				self.httpRedirectToHttpsRequestHandler(ourHostNames),
				self,
			)
		)
	}
	
	#[inline(always)]
	fn tlsServerConfiguration(&self) -> Result<ServerConfig, CordialError>
	{
		let serverHostNamesWithPrimaryFirst = self.localization.serverHostNamesWithPrimaryFirst()?;
		
		let mut serverConfig = ServerConfig::new();
		serverConfig.set_protocols(&["http/1.1".to_owned()]);  // TODO: When HTTP/2 is supported by hyper, add "h2"
		serverConfig.set_persistence(ServerSessionMemoryCache::new(self.maximum_number_of_tls_sessions as usize));
		serverConfig.cert_resolver = RsaManyServersResolvesServerCert::new(&self.inputFolderPath, &self.environment, serverHostNamesWithPrimaryFirst)?;
		Ok(serverConfig)
	}
	
	#[inline(always)]
	fn httpRedirectToHttpsRequestHandler(&self, ourHostNames: HashSet<String>) -> HttpRedirectToHttpsRequestHandler
	{
		HttpRedirectToHttpsRequestHandler::new(self.daemon.https_socket.port(), ourHostNames, self.http_keep_alive)
	}
	
	#[inline(always)]
	fn httpsStaticRequestHandler(&self, ourHostNames: &HashSet<String>, oldResponses: Arc<Responses>) -> Result<HttpsStaticRequestHandler, CordialError>
	{
		let mut handlebars = self.registerHandlebarsTemplates()?;
		
		let resources = self.discoverResources()?;
		
		let newResources = self.renderResources(resources, &oldResponses, &ourHostNames, &mut handlebars)?;
		Ok(HttpsStaticRequestHandler::new(newResources, self.http_keep_alive, self.enable_hsts_preloading_for_production))
	}
	
	#[inline(always)]
	fn registerHandlebarsTemplates(&self) -> Result<Handlebars, CordialError>
	{
		let mut handlebars = Handlebars::new();
		
		// Register any default templates here
		
		// Register any helpers here
		
		// Register any decorators here
		
		let handlebarsTemplatesFolderPath = self.inputFolderPath.join("templates");
		if handlebarsTemplatesFolderPath.exists() && handlebarsTemplatesFolderPath.is_dir()
		{
			handlebarsTemplatesFolderPath.registerAllHandlebarsTemplates(&handlebarsTemplatesFolderPath, &mut handlebars)?;
		}
		Ok(handlebars)
	}
	
	#[inline(always)]
	fn discoverResources(&self) -> Result<Resources, CordialError>
	{
		DiscoverResources::discover(&self, &self.inputFolderPath)
	}
	
	#[inline(always)]
	fn renderResources(&self, mut resources: Resources, oldResponses: &Arc<Responses>, ourHostNames: &HashSet<String>, handlebars: &mut Handlebars) -> Result<Responses, CordialError>
	{
		let mut newResources = Responses::new(self.deploymentDate, ourHostNames);
		let mut siteMapWebPages = self.languagesHashMap();
		let mut rssItems = self.languagesHashMap();
		
		for processingPriority in ProcessingPriority::All.iter()
		{
			for resource in resources.values_mut()
			{
				if resource.hasProcessingPriority(*processingPriority)
				{
					resource.render(&resources, &mut newResources, oldResponses, self, handlebars, &mut siteMapWebPages, &mut rssItems)?;
				}
			}
		}
		
		self.renderResourcesSiteMapsAndRobotsTxt(&mut newResources, oldResponses, handlebars, &siteMapWebPages)?;
		
		self.renderRssFeeds(&mut newResources, oldResponses, handlebars, &rssItems, &resources)?;
		
		newResources.addAnythingThatIsDiscontinued(oldResponses);
		
		Ok(newResources)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn renderResourcesSiteMapsAndRobotsTxt(&self, newResponses: &mut Responses, oldResponses: &Arc<Responses>, handlebars: &mut Handlebars, siteMapWebPages: &HashMap<String, Vec<SiteMapWebPage>>) -> Result<(), CordialError>
	{
		let mut robotsTxtByHostName = BTreeMap::new();
		
		self.visitLanguagesWithPrimaryFirst(|languageData, _isPrimaryLanguage|
		{
			let siteMapIndexUrlsAndListOfLanguageUrls = robotsTxtByHostName.entry(languageData.language.host.to_owned()).or_insert_with(|| (BTreeSet::new(), BTreeSet::new()));
			self.site_map.renderResource(languageData, handlebars, self, newResponses, oldResponses, &mut siteMapIndexUrlsAndListOfLanguageUrls.0, siteMapWebPages)?;
			siteMapIndexUrlsAndListOfLanguageUrls.1.insert(languageData.language.relative_root_url(languageData.iso_639_1_alpha_2_language_code));
			
			Ok(())
		})?;
		
		let primaryHostName = self.primaryLanguageHost()?;
		
		for (hostName, siteMapIndexUrlsAndListOfLanguageUrls) in robotsTxtByHostName.iter()
		{
			self.robots.renderResource(hostName, &siteMapIndexUrlsAndListOfLanguageUrls.1, &siteMapIndexUrlsAndListOfLanguageUrls.0, primaryHostName, handlebars, self, newResponses, oldResponses)?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn renderRssFeeds(&self, newResponses: &mut Responses, oldResponses: &Arc<Responses>, handlebars: &mut Handlebars, rssItems: &HashMap<String, Vec<RssItem>>, resources: &Resources) -> Result<(), CordialError>
	{
		if let Some(rss) = self.rss.as_ref()
		{
			let primary_iso_639_1_alpha_2_language_code = self.primary_iso_639_1_alpha_2_language_code();
			
			self.visitLanguagesWithPrimaryFirst(|languageData, _isPrimaryLanguage|
			{
				let googleAnalytics = self.google_analytics.as_ref().map(|value| value.as_str());
				rss.renderResource(languageData, handlebars, self, newResponses, oldResponses, rssItems, primary_iso_639_1_alpha_2_language_code, resources, googleAnalytics)
			})
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn loadBaselineConfiguration(inputFolderPath: &Path, environment: &str, outputFolderPath: &Path) -> Result<Configuration, CordialError>
	{
		let configurationHjson = loadHjson(&inputFolderPath.join("configuration.hjson"))?;
		
		let environmentInputFolderPath = inputFolderPath.join(environment);
		
		let configurationHjson = loadHjsonIfExtantAndMerge(&environmentInputFolderPath.join("public.configuration.hjson"), configurationHjson)?;
		
		let configurationHjson = loadHjsonIfExtantAndMerge(&environmentInputFolderPath.join("private.configuration.hjson"), configurationHjson)?;
		
		let resource_template = match configurationHjson.find("resource_template")
		{
			None => HjsonValue::Object(HjsonMap::new()),
			Some(value) => value.clone(),
		};
		
		let mut configuration: Configuration = deserializeHjson(configurationHjson)?;
		
		configuration.resource_template = Some(resource_template);
		configuration.inputFolderPath = inputFolderPath.to_path_buf();
		configuration.outputFolderPath = outputFolderPath.to_path_buf();
		configuration.environment = environment.to_owned();
		configuration.deploymentVersion = Self::deploymentVersion(configuration.deploymentDate);
		
		Ok(configuration)
	}
	
	#[inline(always)]
	fn deploymentVersion(deploymentDate: SystemTime) -> String
	{
		// Monday, September 11, 2017; time is start of day
		const SensibleBaseLineSystemTimeInSeconds: u64 = 1505088000;
		
		let timeStamp = deploymentDate.duration_since(UNIX_EPOCH).unwrap().as_secs() - SensibleBaseLineSystemTimeInSeconds;
		let consistent = timeStamp.to_be();
		let raw: [u8; 8] = unsafe { transmute(consistent) };
		base64Encode(&raw, URL_SAFE_NO_PAD)
	}
	
	#[inline(always)]
	fn validateInputFiles(inputFolderPath: &Path) -> Result<(), CordialError>
	{
		let mut errors = Vec::with_capacity(256);
		Self::isFileValid(&mut errors, inputFolderPath, inputFolderPath);
		if errors.is_empty()
		{
			Ok(())
		}
		else
		{
			Err(CordialError::ConfigurationInputFilesAreInvalid(errors))
		}
	}
	
	#[inline(always)]
	fn isFileValid(errors: &mut Vec<String>, inputFolderPath: &Path, path: &Path)
	{
		let metadata = match path.symlink_metadata()
		{
			Err(_) =>
			{
				errors.push(format!("{:?} is unreadable", path));
				return;
			},
			Ok(metadata) => metadata,
		};
		
		let fileType = metadata.file_type();
		
		if fileType.is_symlink()
		{
			// is this an absolute symlink?
			let pointsTo = path.read_link().unwrap();
			if !pointsTo.is_relative()
			{
				errors.push(format!("{:?} is an absolute symlink (is not relative)", path));
				return;
			}
			if pointsTo.has_root()
			{
				errors.push(format!("{:?} is an absolute symlink (has a root)", path));
				return;
			}
			
			match path.canonicalize()
			{
				Err(_) =>
				{
					errors.push(format!("{:?} is a broken symlink", path));
				},
				Ok(canonicalPath) =>
				{
					if canonicalPath.strip_prefix(inputFolderPath).is_err()
					{
						errors.push(format!("{:?} is a symlink that points outside of input {:?} to {:?}", path, inputFolderPath, canonicalPath));
					}
				}
			}
		}
		else if fileType.is_dir()
		{
			match path.read_dir()
			{
				Err(error) => errors.push(format!("Could not read contents of folder {:?} because {}", path, error)),
				Ok(readDir) =>
				{
					for entry in readDir
					{
						match entry
						{
							Err(error) => errors.push(format!("Could not read entry in folder {:?} because {}", path, error)),
							Ok(entry) =>
								{
									let path = entry.path();
									Self::isFileValid(errors, inputFolderPath, &path);
								}
						}
					}
				}
			};
			
		}
		else if fileType.is_file()
		{
		}
		else if fileType.is_block_device()
		{
			errors.push(format!("{:?} is a block device", path));
		}
		else if fileType.is_char_device()
		{
			errors.push(format!("{:?} is a char device", path));
		}
		else if fileType.is_char_device()
		{
			errors.push(format!("{:?} is a char device", path));
		}
		else if fileType.is_fifo()
		{
			errors.push(format!("{:?} is a FIFO", path));
		}
		else if fileType.is_socket()
		{
			errors.push(format!("{:?} is a socket", path));
		}
		else
		{
			errors.push(format!("{:?} is unknown (?Solaris Door?)", path));
		}
	}
	
	#[inline(always)]
	fn primaryLanguageHost(&self) -> Result<&str, CordialError>
	{
		Ok(&self.primaryLanguage()?.host)
	}
	
	#[inline(always)]
	fn primaryLanguage(&self) -> Result<&Language, CordialError>
	{
		self.localization.primaryLanguage()
	}
	
	#[inline(always)]
	fn ourHostNames(&self) -> Result<HashSet<String>, CordialError>
	{
		self.localization.serverHostNames()
	}
	
	#[inline(always)]
	fn numberOfLanguages(&self) -> usize
	{
		self.localization.numberOfLanguages()
	}
	
	#[inline(always)]
	fn languagesHashMap<R>(&self) -> HashMap<String, R>
	{
		HashMap::with_capacity(self.numberOfLanguages())
	}
	
	#[inline(always)]
	pub(crate) fn visitLanguagesWithPrimaryFirst<F: FnMut(&LanguageData, bool) -> Result<(), CordialError>>(&self, visitor: F) -> Result<(), CordialError>
	{
		self.localization.visitLanguagesWithPrimaryFirst(visitor)
	}
	
	#[inline(always)]
	fn maximum_number_of_tls_sessions_default() -> u32
	{
		4096
	}
	
	#[inline(always)]
	fn http_keep_alive_default() -> bool
	{
		true
	}
	
	#[inline(always)]
	fn deploymentDate_default() -> SystemTime
	{
		SystemTime::now()
	}
}
