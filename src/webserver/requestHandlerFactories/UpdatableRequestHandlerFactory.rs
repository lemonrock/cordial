// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug)]
pub struct UpdatableRequestHandlerFactory<R: RequestHandler>
{
	current: RwLock<Arc<R>>,
}

impl<R: RequestHandler> RequestHandlerFactory for UpdatableRequestHandlerFactory<R>
{
	type Item = R;
	
	#[inline(always)]
	fn produce(&self) -> Arc<Self::Item>
	{
		(*self.current.read().unwrap()).clone()
	}
}

impl<R: RequestHandler> UpdatableRequestHandlerFactory<R>
{
	#[inline(always)]
	pub(crate) fn new(requestHandler: R) -> Arc<Self>
	{
		Arc::new
		(
			Self
			{
				current: RwLock::new(Arc::new(requestHandler))
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn update(&self, requestHandler: R)
	{
		*self.current.write().unwrap() = Arc::new(requestHandler);
	}
}
