// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// Holds a set of style rules autoprefixers and applies their rules in turn.
/// Does not handle the intersection of property declaration that needs both a name and value prefixing.
#[derive(Default, Debug, Clone)]
pub struct CompositePropertyDeclarationAutoprefixer
{
	propertyNameAutoprefixers: Vec<PropertyNamePropertyDeclarationAutoprefixer>,
	simplePropertyValueAutoprefixers: Vec<SimplePropertyValuePropertyDeclarationAutoprefixer>,
}

impl PropertyDeclarationAutoprefixer for CompositePropertyDeclarationAutoprefixer
{
	#[inline(always)]
	fn autoprefix<H: HasPropertyDeclarations<I>, I: HasImportance>(&self, property_declarations: &mut H, parent_vendor_prefix: Option<&VendorPrefix>)
	{
		for autoprefixer in self.propertyNameAutoprefixers.iter()
		{
			autoprefixer.autoprefix(property_declarations, parent_vendor_prefix);
		}
		for autoprefixer in self.simplePropertyValueAutoprefixers.iter()
		{
			autoprefixer.autoprefix(property_declarations, parent_vendor_prefix);
		}
	}
}

impl CompositePropertyDeclarationAutoprefixer
{
	#[inline(always)]
	pub(crate) fn new(can_i_use: &CanIUse, our_rules: &AgentNameAndVersionSet) -> Self
	{
		Self
		{
			propertyNameAutoprefixers: PropertyNamePropertyDeclarationAutoprefixerCreator::new(can_i_use, our_rules),
			simplePropertyValueAutoprefixers: SimplePropertyValuePropertyDeclarationAutoprefixerCreator::new(can_i_use, our_rules),
		}
	}
}
