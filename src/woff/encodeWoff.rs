// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) fn encodeWoff(ttfData: &[u8], numberOfIterations: u16, fontMajorVersion: u8, fontMinorVersion: u16, utf8_xml_metadata: &[u8], private_data: &[u8]) -> Result<Malloc<[u8]>, WoffError>
{
	debug_assert!(numberOfIterations <= 5000, "numberOfIterations must be less than or equal to 5000, not '{}'", numberOfIterations);
	debug_assert!(fontMajorVersion < 2, "fontMajorVersion must be less than 2, not '{}'", fontMajorVersion);
	debug_assert!((ttfData.len() as u64) < (u32::max_value() as u64), "ttfData can not exceed 4Gb");
	
	let (woffPointer, woffLength) =
	{
		let mut woffLength = unsafe { uninitialized() };
		let mut status = unsafe { uninitialized() };
		let woffPointer = unsafe { ::sfnt2woff_zopfli_sys::woffEncode(ttfData.as_ptr(), ttfData.len() as u32, fontMajorVersion as u16, fontMinorVersion, numberOfIterations as i32, &mut woffLength, &mut status) };
		if woffPointer.is_null()
		{
			return Err(WoffError::process(status));
		}
		(woffPointer, woffLength)
	};
	
	let (woffPointer, woffLength) =
	if utf8_xml_metadata.is_empty()
	{
		(woffPointer, woffLength)
	}
	else
	{
		let mut newWoffLength = woffLength;
		let mut status = unsafe { uninitialized() };
		let newWoffPointer = unsafe { ::sfnt2woff_zopfli_sys::woffSetMetadata(woffPointer, &mut newWoffLength, utf8_xml_metadata.as_ptr(), utf8_xml_metadata.len() as u32, &mut status) };
		if newWoffPointer.is_null()
		{
			unsafe { ::libc::free(woffPointer as *mut c_void); }
			return Err(WoffError::process(status));
		}
		(newWoffPointer, newWoffLength)
	};
	
	
	let (woffPointer, woffLength) =
	if private_data.is_empty()
	{
		(woffPointer, woffLength)
	}
	else
	{
		let mut newWoffLength = woffLength;
		let mut status = unsafe { uninitialized() };
		let newWoffPointer = unsafe { ::sfnt2woff_zopfli_sys::woffSetPrivateData(woffPointer, &mut newWoffLength, private_data.as_ptr(), private_data.len() as u32, &mut status) };
		if newWoffPointer.is_null()
		{
			unsafe { ::libc::free(woffPointer as *mut c_void); }
			return Err(WoffError::process(status));
		}
		(newWoffPointer, newWoffLength)
	};
	
	Ok(unsafe { Malloc::from_array(woffPointer as *mut _, woffLength as usize) })
}
