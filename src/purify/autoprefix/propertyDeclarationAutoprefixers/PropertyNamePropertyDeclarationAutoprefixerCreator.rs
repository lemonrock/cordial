// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


enum PropertyNamePropertyDeclarationAutoprefixerCreator
{
	Simple
	{
		featureName: &'static str,
		propertyNameAndRemoveIfUnprefixedSet: HashMap<&'static str, bool>,
		overrideIfEqualToOrLater: HashMap<AgentName, Version>,
		vendorPrefixTransformation: Box<Fn(VendorPrefix) -> VendorPrefix>,
		isMicrosoftMasqueradingAsWebkit: bool,
	},
}

impl PropertyNamePropertyDeclarationAutoprefixerCreator
{
	#[inline(always)]
	pub(crate) fn new(can_i_use: &CanIUse, our_rules: &AgentNameAndVersionSet) -> Vec<PropertyNamePropertyDeclarationAutoprefixer>
	{
		let generators = vec!
		[
			Self::simple("css-animation", false, hashset! { "transform", "animation", "animation-name", "animation-duration", "animation-delay", "animation-direction", "animation-fill-mode", "animation-iteration-count", "animation-play-state", "animation-timing-function" }),
			Self::simple("transforms2d", false, hashset! { "transform", "transform-origin" }),
			Self::simple("transforms3d", false, hashset! { "perspective", "perspective-origin", "transform-style" }),
			Self::overrideIfEqualToOrLater("transforms3d", false, hashset! { "backface-visibility" }, hashmap!
			{
				AgentName::AppleSafari => "9".parse().unwrap(),
				AgentName::AppleSafariIOs => "9.0-9.2".parse().unwrap(),
			}, Box::new(|vendorPrefix|
			{
				match vendorPrefix
				{
					ms => webkit,
					_ => vendorPrefix,
				}
			}), true),
			Self::simple("css-filters", false, hashset! { "filter" }),
			Self::simple("user-select-none", false, hashset! { "user-select" }),
			Self::simple("font-feature", false, hashset! { "font-feature-settings", "font-variant-ligatures", "font-language-override" }),
			Self::simple("font-kerning", false, hashset! { "font-kerning" }),
			Self::simple("css-hyphens", false, hashset! { "hyphens" }),
			Self::simple("css3-tabsize", false, hashset! { "tab-size" }),
			Self::simple("css-backdrop-filter", false, hashset! { "backdrop-filter" }),
			Self::simple("css-touch-action", false, hashset! { "touch-action" }),
			Self::simple("object-fit", false, hashset! { "object-fit", "object-position" }),
			Self::simple("css-shapes", false, hashset! { "shape-margin", "shape-outside", "shape-image-threshold" }),
			Self::simple("css-text-align-last", false, hashset! { "text-align-last" }),
			Self::simple("css-writing-mode", false, hashset! { "writing-mode" }),
			Self::simple("text-decoration", false, hashset! { "text-decoration-style", "text-decoration-color", "text-decoration-line", "text-decoration" }),
			Self::simple("text-size-adjust", false, hashset! { "text-size-adjust" }),
			Self::simple("css-masks", false, hashset! { "mask-clip", "mask-composite", "mask-image", "mask-origin", "mask-repeat", "mask-border-repeat", "mask-border-source", "mask", "mask-position", "mask-size", "mask-border", "mask-border-outset", "mask-border-width", "mask-border-slice" }),
			Self::simple("css-clip-path", false, hashset! { "clip-path" }),
			Self::simple("css-boxdecorationbreak", false, hashset! { "box-decoration-break" }),
			Self::simple("css-logical-props", false, hashset! { "border-inline-start", "border-inline-end", "margin-inline-start",  "margin-inline-end", "padding-inline-start", "padding-inline-end",  "border-block-start", "border-block-end", "margin-block-start",  "margin-block-end", "padding-block-start", "padding-block-end" }),
			Self::overrideIfEqualToOrLater("css-appearance", false, hashset! { "appearance" }, hashmap!
			{
				AgentName::MicrosoftEdge => "12".parse().unwrap(),
				AgentName::MicrosoftInternetExplorerMobile => "11".parse().unwrap(),
			}, true, Box::new(|vendorPrefix| vendorPrefix), false),
			Self::simple("text-emphasis", false, hashset! { "text-emphasis", "text-emphasis-position", "text-emphasis-style", "text-emphasis-color" }),
			// Firefox does not support "break-before", "break-after" and "break-inside", whether prefixed -moz- or not
			// Regional Webkit browsers do not support column-fill, including Blackberry 10 and UC Browser
			Self::simple("multicolumn", false, hashset! { "columns", "column-width", "column-gap", "column-rule", "column-rule-color", "column-rule-width",  "column-count", "column-rule-style", "column-span", "column-fill" }),
			// Chrome, Opera, Safari, newer webkit browsers support -webkit-column- prefixed "break-before", "break-after" and "break-inside" with only the values auto and always
			Self::overrideIfEqualToOrLater("multicolumn", false, hashset! { "break-before", "break-after", "break-inside" }, Box::new(|vendorPrefix|
			{
				match vendorPrefix
				{
					webkit => Unrecognised("webkit-column"),
					_ => vendorPrefix,
				}
			}), false),
			
			// Historic
			Self::simple("border-radius", false, hashset! { "border-radius", "border-top-left-radius", "border-top-right-radius", "border-bottom-right-radius", "border-bottom-left-radius" }),
			Self::simple("css-boxshadow", false, hashset! { "box-shadow" }),
			Self::simple("css-transitions", false, hashset! { "transition", "transition-property", "transition-duration", "transition-delay", "transition-timing-function" }),
			Self::simple("css3-boxsizing", false, hashset! { "box-sizing" }),
			Self::simple("background-img-opts", false, hashset! { "background-clip", "background-origin", "background-size" }),
			Self::simple("border-image", false, hashset! { "border-image" }),
			Self::simple("text-overflow", false, hashset! { "text-overflow" }),
			
			// Other
			// css-grid: Since support is complete without prefixes, except for older versions of Edge & IE which implement a different standard, there is very little point supporting this
			// flexbox: Again, support is complete for the current syntax without prefixes for all recent browsers.
			// css-text-spacing: Seems to be a missing caniuse feature
			
			// Not enough support or support might be in the process of deprecation or major change
			// - css-snappoints: existing support too broken
			// - css-regions: might be under deprecation
			// - css-text-decoration / text-decoration-skip: under deprecation; existing support too broken and should be done by hand-writing CSS
		];
		
		let mut result = Vec::new();
		
		for generator in generators.iter()
		{
			generator.generate(can_i_use, our_rules, &mut result);
		}
		
		result
	}
	
	#[inline(always)]
	fn simple(featureName: &str, removeIfUnprefixed: bool, propertyNames: HashSet<&'static str>) -> Self
	{
		PropertyNamePropertyDeclarationAutoprefixerCreator::Simple
		{
			featureName,
			propertyNameAndRemoveIfUnprefixedSet: propertyNames.iter().map(|propertyName| (propertyName, removeIfUnprefixed)).collect(),
			overrideIfEqualToOrLater: HashSet::default(),
			vendorPrefixTransformation: Box::new(|vendorPrefix| vendorPrefix),
			isMicrosoftMasqueradingAsWebkit: false,
		}
	}
	
	// property name prefix should ONLY apply to -webkit-
	// ie we need a vendor prefix transformation function
	#[inline(always)]
	fn overrideIfEqualToOrLater(featureName: &str, removeIfUnprefixed: bool, propertyNames: HashSet<&'static str>, overrideIfEqualToOrLater: HashMap<AgentName, Version>, vendorPrefixTransformation: Box<Fn(VendorPrefix) -> VendorPrefix>, isMicrosoftMasqueradingAsWebkit: bool) -> Self
	{
		PropertyNamePropertyDeclarationAutoprefixerCreator::Simple
		{
			featureName,
			propertyNameAndRemoveIfUnprefixedSet: propertyNames.iter().map(|propertyName| (propertyName, removeIfUnprefixed)).collect(),
			overrideIfEqualToOrLater,
			vendorPrefixTransformation,
			isMicrosoftMasqueradingAsWebkit,
		}
	}
	
	#[inline(always)]
	fn generate(&self, can_i_use: &CanIUse, our_rules: &AgentNameAndVersionSet, autoprefixers: &mut Vec<PropertyNamePropertyDeclarationAutoprefixer>)
	{
		use self::PropertyNamePropertyDeclarationAutoprefixerCreator::*;
		
		match *self
		{
			Simple { ref featureName, ref propertyNameAndRemoveIfUnprefixedSet, ref overrideIfEqualToOrLater, ref vendorPrefixTransformation, isMicrosoftMasqueradingAsWebkit } =>
			{
				let featureName = featureName(featureName);
				
				let mut vendorPrefixes = Rc::new(BTreeSet::new());
				our_rules.support_for_a_feature(can_i_use, &featureName, |agent, version, support|
				{
					if support.requires_prefix() ||
					// override requires_prefix()
					{
						// this occurs for Safari and iOS Safari for the 'backface-visibility' property
						if let Some(agentVersion) = overrideIfEqualToOrLater.get(agent.agent_name())
						{
							version >= agentVersion
						}
						else
						{
							false
						}
					}
					{
						let mut vendorPrefix = vendorPrefixTransformation(mapPrefixToVendorPrefix(agent.prefix(version)));
						Rc::get_mut(vendorPrefixes).unwrap().insert(vendorPrefix);
					}
				});
				
				for &(propertyName, removeUnprefixedPropertyName) in propertyNameAndRemoveIfUnprefixedSet.iter()
				{
					autoprefixers.push
					(
						PropertyNamePropertyDeclarationAutoprefixer
						{
							propertyName,
							removeUnprefixedPropertyName,
							vendorPrefixes: vendorPrefixes.clone(),
							propertyNamePrefix,
							isMicrosoftMasqueradingAsWebkit,
						}
					);
				}
			},
		}
	}
}
