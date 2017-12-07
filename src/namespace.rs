// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[macro_export]
macro_rules! namespace
{
	// Trailing comma
	($($key:expr => $value:expr,)+) => (namespace!($($key => $value),+));
	
	// Regular
	( $($key:expr => $value:expr),* ) =>
	{
		{
			let mut ownedMap = ::std::collections::BTreeMap::new();
			$(
				ownedMap.insert($key.to_owned(), $value.to_owned());
			)*
			$crate::xml::namespace::Namespace(ownedMap)
		}
	};
}
