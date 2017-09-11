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
	pub(crate) fn new(environment: &str, inputFolderPath: PathBuf, outputFolderPath: PathBuf, isDaemon: bool) -> Self
	{
		Self
		{
			environment: environment.to_owned(),
			inputFolderPath,
			outputFolderPath,
			isDaemon,
			oldResources: Arc::new(Resources::empty())
		}
	}
	
	#[inline(always)]
	pub(crate) fn startWebserver(mut self) -> Result<(), CordialError>
	{
		let (configuration, serverConfig, httpRedirectToHttpsRequestHandler, httpsStaticRequestHandler) = self.justConfigurationReconfigure()?;
		
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
		let (_configuration, serverConfig, httpRedirectToHttpsRequestHandler, httpsStaticRequestHandler) = self.justConfigurationReconfigure()?;
		
		self.oldResources = httpsStaticRequestHandler.resources();
		
		updatableTlsServerConfigurationFactory.update(serverConfig);
		httpRequestHandlerFactory.update(httpRedirectToHttpsRequestHandler);
		httpsRequestHandlerFactory.update(httpsStaticRequestHandler);
		Ok(())
	}
	
	#[inline(always)]
	fn justConfigurationReconfigure(&self) -> Result<(Configuration, ServerConfig, HttpRedirectToHttpsRequestHandler, HttpsStaticRequestHandler), CordialError>
	{
		Configuration::reconfigure(&self.environment, &self.inputFolderPath, &self.outputFolderPath, self.oldResources.clone())
	}
	
	#[inline(always)]
	pub(crate) fn respondsToCtrlC(&self) -> bool
	{
		!self.isDaemon
	}
}
