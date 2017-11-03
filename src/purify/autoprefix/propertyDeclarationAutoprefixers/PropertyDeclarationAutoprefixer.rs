// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// Prefixes anything inside a style rule, typically property declarations.
pub trait PropertyDeclarationAutoprefixer
{
	/// Prefixes with any relevant vendor prefixes
	/// If operating within, say, @-moz-document, only -moz- prefixes are only applied if they are relevant
	fn autoprefix<I: HasImportance>(&self, property_declarations: &mut PropertyDeclarations<I>, parent_vendor_prefix: Option<&VendorPrefix>);
}
