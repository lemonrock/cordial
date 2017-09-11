// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct UpdatableTlsServerConfigurationFactory
{
	current: RwLock<Arc<ServerConfig>>,
}

impl UpdatableTlsServerConfigurationFactory
{
	pub(crate) fn new(tlsServerConfiguration: ServerConfig) -> Arc<Self>
	{
		Arc::new
		(
			Self
			{
				current: RwLock::new(Arc::new(tlsServerConfiguration))
			}
		)
	}
	
	#[inline(always)]
	fn produce(&self) -> Arc<ServerConfig>
	{
		(*self.current.read().unwrap()).clone()
	}
	
	#[inline(always)]
	pub(crate) fn update(&self, tlsServerConfiguration: ServerConfig)
	{
		*self.current.write().unwrap() = Arc::new(tlsServerConfiguration);
	}
}
