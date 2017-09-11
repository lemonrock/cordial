// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Configuration
{
	#[serde(default = "Configuration::http_socket_default")] http_socket: ServerSocket,
	#[serde(default = "Configuration::https_socket_default")] https_socket: ServerSocket,
	#[serde(default = "Configuration::maximum_number_of_tls_sessions_default")] maximum_number_of_tls_sessions: u32,
	#[serde(default = "Configuration::http_keep_alive_default")] http_keep_alive: bool,
	#[serde(default, skip_deserializing)] resource_template: Option<HjsonValue>,
	localization: localization,
	#[serde(default, skip_deserializing)] inputFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] cacheFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] siteOutputFolderPath: PathBuf,
	#[serde(default, skip_deserializing)] environment: String,
	#[serde(default, skip_deserializing)] deploymentVersion: String,
}

impl Configuration
{
	#[inline(always)]
	pub(crate) fn serverSockets<'a>(&'a self) -> (&'a ServerSocket, &'a ServerSocket)
	{
		(&self.http_socket, &self.https_socket)
	}
	
	#[inline(always)]
	pub(crate) fn reconfigure(environment: &str, inputFolderPath: &Path, outputFolderPath: &Path) -> Result<(Self, ServerConfig, HttpRedirectToHttpsRequestHandler, HttpsStaticRequestHandler), CordialError>
	{
		Self::validateInputFiles(inputFolderPath)?;
		let cacheFolderPath = outputFolderPath.createSubFolder("cache").context(outputFolderPath)?;
		let siteOutputFolderPath = outputFolderPath.recreateSubFolder("site").context(outputFolderPath)?;
		let configuration = Configuration::loadBaselineConfiguration(&inputFolderPath, environment, cacheFolderPath, siteOutputFolderPath)?;
		
		configuration.finishReconfigure()
	}
	
	#[inline(always)]
	fn finishReconfigure(self) -> Result<(Self, ServerConfig, HttpRedirectToHttpsRequestHandler, HttpsStaticRequestHandler), CordialError>
	{
		let resources = DiscoverResources::discoverResources(&self, &self.inputFolderPath)?;
		
		let (tlsServerConfiguration, mut httpsHandler, httpHandler) = self.requestHandlers()?;
		
		{
			let visitor = |languageCode: &str, language: &language, _isPrimaryLanguage: bool|
			{
				for (_, resource) in resources.iter()
				{
					resource.createOutput(&self.environment, languageCode, language, &self.localization, &self.siteOutputFolderPath, &self.inputFolderPath, &mut httpsHandler, &self.deploymentVersion)?
				}
				Ok(())
			};
			self.visitLanguagesWithPrimaryFirst(visitor)?;
		}
		
		Ok((self, tlsServerConfiguration, httpHandler, httpsHandler))
	}
	
	#[inline(always)]
	fn loadBaselineConfiguration(inputFolderPath: &Path, environment: &str, cacheFolderPath: PathBuf, siteOutputFolderPath: PathBuf) -> Result<Configuration, CordialError>
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
		configuration.cacheFolderPath = cacheFolderPath;
		configuration.siteOutputFolderPath = siteOutputFolderPath;
		configuration.environment = environment.to_owned();
		configuration.deploymentVersion = Self::deploymentVersion();
		
		Ok(configuration)
	}
	
	// This scheme, over 10 years, will use a maximum of 4 bytes, thus giving a version string of 6 URL-safe bytes (or 7 if one includes a period)
	// This data should be cache'd
	#[inline(always)]
	fn deploymentVersion() -> String
	{
		// Monday, September 11, 2017; time is start of day
		const SensibleBaseLineSystemTimeInSeconds: u64 = 1505088000;
		
		let timeStamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - SensibleBaseLineSystemTimeInSeconds;
		let consistent = timeStamp.to_be();
		let raw: [u8; 8] = unsafe { transmute(consistent) };
		base64Encode(&raw, URL_SAFE_NO_PAD)
	}
	
	#[inline(always)]
	fn resource_template(&self) -> HjsonValue
	{
		self.resource_template.as_ref().unwrap().clone()
	}
	
	#[inline(always)]
	fn visitLanguagesWithPrimaryFirst<F: FnMut(&str, &language, bool) -> Result<(), CordialError>>(&self, visitor: F) -> Result<(), CordialError>
	{
		self.localization.visitLanguagesWithPrimaryFirst(visitor)
	}
	
	#[inline(always)]
	fn requestHandlers(&self) -> Result<(ServerConfig, HttpsStaticRequestHandler, HttpRedirectToHttpsRequestHandler), CordialError>
	{
		let ourHostNames = self.localization.serverHostNames()?;
		Ok
		(
			(
				self.tlsServerConfiguration()?,
				HttpsStaticRequestHandler::new(&ourHostNames, self.http_keep_alive),
				HttpRedirectToHttpsRequestHandler::new(self.https_socket.port(), ourHostNames, self.http_keep_alive)
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
	fn http_socket_default() -> ServerSocket
	{
		ServerSocket
		{
			socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
			time_to_live: 0,
			only_v6: false,
			reuse_address: false,
			reuse_port: false,
			backlog: 0,
			linger: None,
		}
	}
	
	#[inline(always)]
	fn https_socket_default() -> ServerSocket
	{
		ServerSocket
		{
			socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8443),
			time_to_live: 0,
			only_v6: false,
			reuse_address: false,
			reuse_port: false,
			backlog: 0,
			linger: None,
		}
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
	
}
