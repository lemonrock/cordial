// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


thread_local!(static WebAppManifestSerializationStateThreadLocal: RefCell<Option<WebAppManifestSerializationState>> = RefCell::new(None));

pub(crate) struct WebAppManifestSerializationState
{
	resources: *const Resources,
	fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language,
	iso639Dash1Alpha2Language: Option<Iso639Dash1Alpha2Language>,
}

impl WebAppManifestSerializationState
{
	// Never inlined, so can not transform function and remove hackToKeepReferenceToResourcesAlive
	#[inline(never)]
	pub(crate) fn with<Callback: FnOnce() -> Result<R, CordialError>, R>(resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, callback: Callback) -> Result<(R, usize), CordialError>
	{
		Self::before(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language);
		
		// TODO: To be very defensive, we need to catch panics
		let result = callback();
		
		Self::after();
		
		// ?Needed to keep reference to resources alive?
		let hackToKeepReferenceToResourcesAlive = resources.len();
		result.map(|actualResultWeCareAbout| (actualResultWeCareAbout, hackToKeepReferenceToResourcesAlive))
	}
	
	#[inline(always)]
	fn before(resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language)
	{
		WebAppManifestSerializationStateThreadLocal.with(|state|
		{
			assert!(state.borrow().is_none());
			
			*state.borrow_mut() = Some
			(
				WebAppManifestSerializationState
				{
					resources: resources as *const _,
					fallbackIso639Dash1Alpha2Language,
					iso639Dash1Alpha2Language: Some(iso639Dash1Alpha2Language),
				}
			)
		});
	}
	
	#[inline(always)]
	pub(crate) fn urlDataFrom<S: Serializer>(resourceUrl: &ResourceUrl, resourceTag: ResourceTag) -> Result<Rc<UrlData>, S::Error>
	{
		let resourceReference = ResourceReference
		{
			resource: resourceUrl.clone(),
			tag: resourceTag
		};
		Self::urlData::<S>(&resourceReference)
	}
	
	#[inline(always)]
	pub(crate) fn urlData<S: Serializer>(resourceReference: &ResourceReference) -> Result<Rc<UrlData>, S::Error>
	{
		WebAppManifestSerializationStateThreadLocal.with(|refCell|
		{
			let borrowed = refCell.borrow();
			let this = borrowed.as_ref().unwrap();
			
			resourceReference.urlDataMandatory(unsafe { &*this.resources }, this.fallbackIso639Dash1Alpha2Language, this.iso639Dash1Alpha2Language).map_err(|cordialError| S::Error::custom(cordialError))
		})
	}
	
	#[inline(always)]
	fn after()
	{
		WebAppManifestSerializationStateThreadLocal.with(|state| *state.borrow_mut() = None);
	}
}
