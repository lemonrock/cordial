// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// This autoprefixer will only work for simple, 'whole-value' properties, eg those that have a CSS ident token as a value.
/// It does not work when a property-name also needs to be vendor-prefixed, although that is currently an unnecessary requirement.
#[derive(Debug, Clone)]
pub struct SimplePropertyValuePropertyDeclarationAutoprefixer
{
	propertyName: &'static str,
	propertyValue: &'static str,
	removeUnprefixedProperty: bool,
	vendorPrefixes: BTreeSet<VendorPrefix>,
}

impl PropertyDeclarationAutoprefixer for SimplePropertyValuePropertyDeclarationAutoprefixer
{
	fn autoprefix<H: HasPropertyDeclarations<I>, I: HasImportance>(&self, property_declarations: &mut H, parent_vendor_prefix: Option<&VendorPrefix>)
	{
		let mut list = property_declarations.property_declarations_vec_mut();
		
		let mut index = 0;
		while index != list.len()
		{
			// This ghastly code is because of Rust's borrow checker not allowing simultaneous immutable and mutable references to list via an index in the list, even though we know full well there isn't a real problem in this code
			let vendorPrefixed =
			{
				let propertyDeclaration = list.get(index).unwrap();
				
				if propertyDeclaration.hasAsciiNameIgnoringCase(&self.propertyName)
				{
					let mut vendorPrefixed = Vec::with_capacity(self.vendorPrefixes.len());
					
					for vendorPrefix in self.vendorPrefixes.iter().rev()
					{
						// TODO: Be more efficient and do this outside of the loop
						// This logic 'filters' out vendor prefixes that would be ignored because we are evaluating a property inside, say, @-moz-document, where a prefixed property, such as -webkit-appearance, would make no sense
						let include = if let Some(parentVendorPrefix) = parent_vendor_prefix
						{
							parentVendorPrefix == vendorPrefix
						}
						else
						{
							true
						};
						
						if include
						{
							vendorPrefixed.push(PropertyDeclaration
							{
								vendor_prefix: Some(vendorPrefix.clone()),
								name: propertyDeclaration.name.clone(),
								value: match &propertyDeclaration.value
								{
									value @ &UnparsedPropertyValue::CssWideKeyword(_) => value.clone(),
									value @ &UnparsedPropertyValue::SpecifiedValue(ref specifiedValue) => if &specifiedValue.originalCss == self.propertyValue
									{
										UnparsedPropertyValue::SpecifiedValue(vendorPrefix.prefix(&specifiedValue.originalCss))
									}
									else
									{
										value.clone()
									},
								},
								importance: propertyDeclaration.importance,
							});
						}
					}
					
					Some(vendorPrefixed)
				}
				else
				{
					None
				}
			};
			
			// TODO: Inefficient
			index += if let Some(mut vendorPrefixed) = vendorPrefixed
			{
				let indexIncrement = vendorPrefixed.len();
				
				for propertyDeclaration in vendorPrefixed.drain(..)
				{
					list.insert(index, propertyDeclaration);
				}
				
				if self.removeUnprefixedProperty
				{
					list.remove(index + indexIncrement);
					indexIncrement
				}
				else
				{
					indexIncrement + 1
				}
			}
			else
			{
				1
			};
		}
	}
}
