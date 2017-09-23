// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.



use ::malloc_buf::Malloc;
use ::libc::c_void;
use ::std::mem::uninitialized;


include!("encodeWoff.rs");
include!("WoffError.rs");


pub const DefaultNumberOfIterations: u16 = 15;

pub const DefaultFontMajorVersion: u8 = 0;

pub const DefaultFontMinorVersion: u16 = 0;
