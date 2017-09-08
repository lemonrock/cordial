// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone, Deserialize)]
pub struct Configuration
{
	#[serde(default = "Configuration::http_socket_default")] http_socket: ServerSocket,
	#[serde(default = "Configuration::https_socket_default")] https_socket: ServerSocket,
	#[serde(default = "Configuration::maximum_number_of_tls_sessions_default")] maximum_number_of_tls_sessions: u32,
	#[serde(default = "Configuration::http_keep_alive_default")] http_keep_alive: bool,
	#[serde(default, skip_deserializing)] resource_template: Option<HjsonValue>,
	#[serde(default, skip_deserializing)] environment: String,
	localization: localization,
}

impl Configuration
{
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
	
	pub fn loadBaselineConfiguration(inputFolderPath: &Path, environment: &str) -> Result<Configuration, CordialError>
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
		configuration.environment = environment.to_owned();
		
		Ok(configuration)
	}
	
	#[inline(always)]
	pub fn resource_template(&self) -> HjsonValue
	{
		self.resource_template.as_ref().unwrap().clone()
	}
	
	#[inline(always)]
	pub fn primaryLanguage(&self) -> Result<&language, CordialError>
	{
		self.localization.primaryLanguage()
	}
	
	#[inline(always)]
	pub fn visitLanguagesWithPrimaryFirst<F: FnMut(&str, &language, bool) -> Result<(), CordialError>>(&self, visitor: F) -> Result<(), CordialError>
	{
		self.localization.visitLanguagesWithPrimaryFirst(visitor)
	}
	
	#[inline(always)]
	pub fn webserver(&self,  inputFolderPath: &Path, respondsToCtrlC: bool) -> Result<(), CordialError>
	{
		let tlsServerConfiguration = self.tlsServerConfiguration(inputFolderPath)?;
		
		let httpHandler = HttpRedirectToHttpsRequestHandler
		{
			portToRedirectTo: self.https_socket.port(),
		};
		
		let httpsHandler = HttpsStaticRequestHandler
		{
		
		};
		
		Webserver::start(tlsServerConfiguration, &self.http_socket, &self.https_socket, self.http_keep_alive, Arc::new(self.localization.serverHostNames()?), Arc::new(httpHandler), Arc::new(httpsHandler), respondsToCtrlC).context(inputFolderPath)?;
		Ok(())
	}
	
	#[inline(always)]
	fn tlsServerConfiguration(&self, inputFolderPath: &Path) -> Result<Arc<ServerConfig>, CordialError>
	{
		let serverHostNamesWithPrimaryFirst = self.localization.serverHostNamesWithPrimaryFirst()?;
		
		let mut serverConfig = ServerConfig::new();
		serverConfig.set_protocols(&["http/1.1".to_owned()]);  // TODO: When HTTP/2 is supported by hyper, add "h2"
		serverConfig.set_persistence(ServerSessionMemoryCache::new(self.maximum_number_of_tls_sessions as usize));
		serverConfig.cert_resolver = RsaManyServersResolvesServerCert::new(inputFolderPath, &self.environment, serverHostNamesWithPrimaryFirst)?;
		Ok(Arc::new(serverConfig))
	}
}
