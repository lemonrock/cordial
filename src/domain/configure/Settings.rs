// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct Settings
{
	environment: String,
	inputFolderPath: PathBuf,
	outputFolderPath: PathBuf,
	isDaemon: bool,
	oldResources: Arc<Resources>,
}

impl Settings
{
	#[inline(always)]
	pub(crate) fn new(environment: &str, uncanonicalizedInputFolderPath: PathBuf, uncanonicalizedOutputFolderPath: PathBuf, isDaemon: bool) -> Self
	{
		let (inputFolderPath, outputFolderPath) = Self::canonicalizeInputAndOutputFolderPaths(uncanonicalizedInputFolderPath, uncanonicalizedOutputFolderPath);
		
		Self
		{
			environment: environment.to_owned(),
			inputFolderPath,
			outputFolderPath,
			isDaemon,
			oldResources: Arc::new(Resources::empty(SystemTime::now()))
		}
	}
	
	#[inline(always)]
	pub(crate) fn startWebserver(mut self) -> Result<(), CordialError>
	{
		let (serverConfig, httpsStaticRequestHandler, httpRedirectToHttpsRequestHandler, configuration) = self.justConfigurationReconfigure()?;
		
		self.oldResources = httpsStaticRequestHandler.resources();
		
		let updatableTlsServerConfigurationFactory = UpdatableTlsServerConfigurationFactory::new(serverConfig);
		let httpRequestHandlerFactory = UpdatableRequestHandlerFactory::new(httpRedirectToHttpsRequestHandler);
		let httpsRequestHandlerFactory = UpdatableRequestHandlerFactory::new(httpsStaticRequestHandler);
		
		let context = self.inputFolderPath.clone();
		
		let (httpSocket, httpsSocket) = configuration.daemonizeAndBindSockets(self.isDaemon)?;
		Webserver::start(updatableTlsServerConfigurationFactory, httpSocket, httpsSocket, httpRequestHandlerFactory, httpsRequestHandlerFactory, self).context(context)?;
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn reconfigure(&mut self, updatableTlsServerConfigurationFactory: &Arc<UpdatableTlsServerConfigurationFactory>, httpRequestHandlerFactory: &Arc<UpdatableRequestHandlerFactory<HttpRedirectToHttpsRequestHandler>>, httpsRequestHandlerFactory: &Arc<UpdatableRequestHandlerFactory<HttpsStaticRequestHandler>>) -> Result<(), CordialError>
	{
		let (serverConfig, httpsStaticRequestHandler, httpRedirectToHttpsRequestHandler, _configuration) = self.justConfigurationReconfigure()?;
		
		self.oldResources = httpsStaticRequestHandler.resources();
		
		updatableTlsServerConfigurationFactory.update(serverConfig);
		httpRequestHandlerFactory.update(httpRedirectToHttpsRequestHandler);
		httpsRequestHandlerFactory.update(httpsStaticRequestHandler);
		Ok(())
	}
	
	#[inline(always)]
	fn justConfigurationReconfigure(&self) -> Result<(ServerConfig, HttpsStaticRequestHandler, HttpRedirectToHttpsRequestHandler, Configuration), CordialError>
	{
		Configuration::reconfigure(&self.environment, &self.inputFolderPath, &self.outputFolderPath, self.oldResources.clone())
	}
	
	#[inline(always)]
	pub(crate) fn respondsToCtrlC(&self) -> bool
	{
		!self.isDaemon
	}
	
	fn canonicalizeInputAndOutputFolderPaths(uncanonicalizedInputFolderPath: PathBuf, uncanonicalizedOutputFolderPath: PathBuf) -> (PathBuf, PathBuf)
	{
		let canonicalizedInputFolderPath = match uncanonicalizedInputFolderPath.metadata()
		{
			Err(error) =>
			{
				fatal(format!("Could not read from --input {:?} because '{}'", uncanonicalizedInputFolderPath, error), 2);
			}
			Ok(metadata) =>
			{
				if !metadata.is_dir()
				{
					fatal(format!("--input {:?} is not a folder path", uncanonicalizedInputFolderPath), 2);
				}
				match uncanonicalizedInputFolderPath.canonicalize()
				{
					Err(error) => fatal(format!("Could not canonicalize --input {:?} because '{}'", uncanonicalizedInputFolderPath, error), 2),
					Ok(canonicalizedInputFolderPath) => canonicalizedInputFolderPath,
				}
			}
		};
		
		if !canonicalizedInputFolderPath.is_dir()
		{
			fatal(format!("Canonicalized input path {:?} is a not a folder", canonicalizedInputFolderPath), 1);
		}
		
		if let Err(error) = create_dir_all(&uncanonicalizedOutputFolderPath)
		{
			fatal(format!("Could not create --output {:?} because '{}'", canonicalizedInputFolderPath, error), 2);
		}
		
		let canonicalizedOutputFolderPath = match uncanonicalizedOutputFolderPath.canonicalize()
		{
			Err(error) => fatal(format!("Could not canonicalize --output {:?} because '{}'", canonicalizedInputFolderPath, error), 2),
			Ok(canonicalizedOutputFolderPath) => canonicalizedOutputFolderPath,
		};
		
		(canonicalizedInputFolderPath, canonicalizedOutputFolderPath)
	}
}
