// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Configuration
{
	#[serde(default)] daemon: Daemon,
	#[serde(default = "Configuration::maximum_number_of_tls_sessions_default")] maximum_number_of_tls_sessions: u32,
	#[serde(default = "Configuration::http_keep_alive_default")] http_keep_alive: bool,
	#[serde(default)] enable_hsts_preloading_for_production: bool,
	#[serde(default)] allow_search_engine_indexing_for_production: bool,
	#[serde(default, skip_serializing, skip_deserializing)] resource_template: Option<HjsonValue>,
	#[serde(default)] pub(crate) localization: Localization,
	#[serde(default)] robots: RobotsTxt,
	#[serde(default)] site_map: SiteMap,
	#[serde(default)] rss: HashMap<Rc<RssChannelName>, RssChannel>,
	#[serde(default)] google_analytics: Option<String>,
	#[serde(default, skip_deserializing)] pub(crate) inputFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] outputFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] pub(crate) environment: String,
	#[serde(default = "Configuration::deploymentDate_default", skip_deserializing)] pub(crate) deploymentDate: SystemTime,
	#[serde(default, skip_deserializing)] pub(crate) deploymentVersion: String,
	#[serde(default, skip_deserializing)] pub(crate) luaFolderPath: Arc<PathBuf>,
	#[serde(default, skip_deserializing)] pub(crate) sassImportPaths: Vec<PathBuf>,
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
			allow_search_engine_indexing_for_production: false,
			resource_template: None,
			localization: Localization::default(),
			robots: RobotsTxt::default(),
			site_map: SiteMap::default(),
			rss: HashMap::default(),
			google_analytics: None,
			inputFolderPath: PathBuf::default(),
			outputFolderPath: PathBuf::default(),
			environment: String::default(),
			deploymentDate: Self::deploymentDate_default(),
			deploymentVersion: String::default(),
			luaFolderPath: Default::default(),
			sassImportPaths: Default::default(),
		}
	}
}

impl Configuration
{
	#[inline(always)]
	pub(crate) fn languageData<'a>(&'a self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<LanguageData<'a>, CordialError>
	{
		self.localization.languageData(iso639Dash1Alpha2Language)
	}
	
	#[inline(always)]
	pub(crate) fn primaryLanguageData<'a>(&'a self) -> Result<LanguageData<'a>, CordialError>
	{
		self.localization.languageData(self.localization.fallbackIso639Dash1Alpha2Language())
	}
	
	#[inline(always)]
	pub(crate) fn facebookOpenGraphLocaleStr(&self, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<&str, CordialError>
	{
		self.localization.facebookOpenGraphLocaleStr(iso639Dash1Alpha2Language)
	}
	
	#[inline(always)]
	pub(crate) fn otherLanguages(&self, excludeIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> HashMap<Iso639Dash1Alpha2Language, Language>
	{
		self.localization.otherLanguages(excludeIso639Dash1Alpha2Language)
	}
	
	#[inline(always)]
	pub(crate) fn fallbackIso639Dash1Alpha2Language(&self) -> Iso639Dash1Alpha2Language
	{
		self.localization.fallbackIso639Dash1Alpha2Language()
	}
	
	#[inline(always)]
	pub(crate) fn sassOptions<'p>(&'p self, precision: u8, input_syntax: InputSyntax) -> SassOptions<'p, PathBuf>
	{
		SassOptions
		{
			output_style: OutputStyle::Compressed,
			source_comments: false,
			source_map_embed: false,
			source_map_contents: false,
			source_map_file_urls: false,
			omit_source_map_url: true,
			indent: CString::new("").unwrap(),
			linefeed: CString::new("\n").unwrap(),
			precision,
			input_syntax,
			include_paths: self.sassImportPaths.as_slice(),
			function_list: Rc::new(SassFunctionList::new(vec![])),
			importer_list: Rc::new(SassImporterList::new(vec![])),
			header_list: Rc::new(SassImporterList::new(vec![])),
		}
	}
	
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
		let configuration = Self::loadConfiguration(&inputFolderPath, environment, outputFolderPath)?;
		
		configuration.finishReconfigure(oldResponses)
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
		let handlebars = self.registerHandlebarsTemplates()?;
		
		let resources = self.discoverResources()?;
		
		let newResources = self.render(resources, &oldResponses, &ourHostNames, &handlebars)?;
		Ok(HttpsStaticRequestHandler::new(newResources, self.http_keep_alive, self.enable_hsts_preloading_for_production, self.allow_search_engine_indexing_for_production))
	}
	
	#[inline(always)]
	fn registerHandlebarsTemplates(&self) -> Result<HandlebarsWrapper, CordialError>
	{
		HandlebarsWrapper::new(&self.inputFolderPath.join("templates"), self)
	}
	
	#[inline(always)]
	fn discoverResources(&self) -> Result<Resources, CordialError>
	{
		DiscoverResources::discover(&self, &self.inputFolderPath)
	}
	
	#[inline(always)]
	fn render(&self, resources: Resources, oldResponses: &Arc<Responses>, ourHostNames: &HashSet<String>, handlebars: &HandlebarsWrapper) -> Result<Responses, CordialError>
	{
		// TODO: Load cache of Responses
		// However, need to check that Responses hash is valid
		// TODO: Multi-thread creation of Responses
		
		
		let mut newResponses = Responses::new(self.deploymentDate, ourHostNames);
		
		{
			let mut rssChannelsByLanguage = self.rssChannelsByLanguage();
			let mut siteMapWebPages = self.languagesHashMap();
			
			self.renderResources(&mut newResponses, oldResponses, handlebars, &resources, &mut rssChannelsByLanguage, &mut siteMapWebPages)?;
			
			self.renderRssFeeds(&mut newResponses, oldResponses, handlebars, &resources, &rssChannelsByLanguage)?;
			
			self.renderSiteMapsAndRobotsTxt(&mut newResponses, oldResponses, handlebars, &resources, &siteMapWebPages)?;
		}
		
		newResponses.addAnythingThatIsDiscontinued(oldResponses);
		
		Ok(newResponses)
	}
	
	#[inline(always)]
	fn rssChannelsByLanguage(&self) -> HashMap<Iso639Dash1Alpha2Language, HashMap<Rc<RssChannelName>, Vec<RssItem>>>
	{
		let mut rssChannelsByLanguage = self.languagesHashMap();
		self.visitLanguagesWithPrimaryFirst(|languageData, _isPrimaryLanguage|
		{
			let mut rssChannelsToRssItems = HashMap::with_capacity(self.rss.len());
			for rssChannelName in self.rss.keys()
			{
				rssChannelsToRssItems.insert(rssChannelName.clone(), Vec::new());
			}
			rssChannelsByLanguage.insert(languageData.iso639Dash1Alpha2Language, rssChannelsToRssItems);
			
			Ok(())
		}).unwrap();
		rssChannelsByLanguage
	}
	
	#[inline(always)]
	fn renderResources(&self, newResponses: &mut Responses, oldResponses: &Arc<Responses>, handlebars: &HandlebarsWrapper, resources: &Resources, rssChannelsByLanguage: &mut HashMap<Iso639Dash1Alpha2Language, HashMap<Rc<RssChannelName>, Vec<RssItem>>>, siteMapWebPages: &mut HashMap<Iso639Dash1Alpha2Language, Vec<SiteMapWebPage>>) -> Result<(), CordialError>
	{
		for processingPriority in ProcessingPriority::All.iter()
		{
			for (resourceUrl, resource) in resources.iter()
			{
				let hasProcessingPriority = resource.borrow().hasProcessingPriority(*processingPriority);
				if hasProcessingPriority
				{
					resource.borrow_mut().renderResource(resourceUrl, resources, newResponses, oldResponses, self, handlebars, rssChannelsByLanguage, siteMapWebPages)?;
				}
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn renderRssFeeds(&self, newResponses: &mut Responses, oldResponses: &Arc<Responses>, handlebars: &HandlebarsWrapper, resources: &Resources, rssChannelsByLanguage: &HashMap<Iso639Dash1Alpha2Language, HashMap<Rc<RssChannelName>, Vec<RssItem>>>) -> Result<(), CordialError>
	{
		let fallbackIso639Dash1Alpha2Language = self.fallbackIso639Dash1Alpha2Language();
		let googleAnalytics = self.google_analytics.as_ref().map(|value| value.as_str());
		for (iso639Dash1Alpha2Language, rssChannels) in rssChannelsByLanguage.iter()
		{
			let languageData = self.languageData(*iso639Dash1Alpha2Language)?;
			
			for (rssChannelName, rssItems) in rssChannels.iter()
			{
				let rssChannel = self.rss.get(rssChannelName).unwrap();
				rssChannel.renderRssChannel(fallbackIso639Dash1Alpha2Language, &languageData, handlebars, self, newResponses, oldResponses, resources, googleAnalytics, rssChannelName, rssItems)?;
			}
		}
		Ok(())
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn renderSiteMapsAndRobotsTxt(&self, newResponses: &mut Responses, oldResponses: &Arc<Responses>, handlebars: &HandlebarsWrapper, resources: &Resources, siteMapWebPages: &HashMap<Iso639Dash1Alpha2Language, Vec<SiteMapWebPage>>) -> Result<(), CordialError>
	{
		let mut robotsTxtByHostName = BTreeMap::new();
		
		self.visitLanguagesWithPrimaryFirst(|languageData, _isPrimaryLanguage|
		{
			let mut robotsConfiguration = robotsTxtByHostName.entry(languageData.language.host.to_owned()).or_insert_with(|| RobotsTxtConfiguration::default());
			
			self.site_map.renderSiteMap(languageData, handlebars, self, resources, newResponses, oldResponses, &mut robotsConfiguration, siteMapWebPages)?;
			
			robotsConfiguration.addRelativeUrlPathForRobotDirective(languageData);
			
			Ok(())
		})?;
		
		let primaryHostName = self.primaryLanguageHost()?;
		
		for (hostName, robotsTxtConfiguration) in robotsTxtByHostName.iter()
		{
			self.robots.renderRobotsTxt(hostName, robotsTxtConfiguration, primaryHostName, handlebars, self, newResponses, oldResponses)?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn loadConfiguration(inputFolderPath: &Path, environment: &str, outputFolderPath: &Path) -> Result<Configuration, CordialError>
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
		configuration.luaFolderPath = Arc::new(inputFolderPath.join("lua"));
		
		#[inline(always)]
		fn findSassImportPaths(inputFolderPath: &Path) -> Result<Vec<PathBuf>, CordialError>
		{
			let mut importPaths = Vec::with_capacity(16);
			let sassImportsPath = inputFolderPath.join("sass-imports");
			for entry in sassImportsPath.read_dir().context(&sassImportsPath)?
			{
				let entry = entry.context(&sassImportsPath)?;
				
				let path = entry.path();
				
				if entry.file_type().context(&path)?.is_dir()
				{
					importPaths.push(path)
				}
			}
			
			Ok(importPaths)
		}
		configuration.sassImportPaths = findSassImportPaths(inputFolderPath)?;
		
		
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
	fn languagesHashMap<R>(&self) -> HashMap<Iso639Dash1Alpha2Language, R>
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
