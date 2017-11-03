// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
//use ::css::Stylesheet;
//use ::css::domain::HasCssRules;
//use ::css::domain::StyleRule;
use ::std::cell::Cell;
use ::std::cell::RefCell;
use ::std::rc::Rc;
use ::html5ever::rcdom::Node;
use ::html5ever::rcdom::NodeData::*;
use ::tendril::NonAtomic;
use ::tendril::Tendril;
use ::tendril::fmt::UTF8;


pub mod autoprefix;


include!("NodeExt.rs");
include!("RcDomExt.rs");

pub struct Purify
{}

impl Purify
{
//	pub fn operateOnStyleRules<H: HasCssRules, UseStyleRule: FnMut(&mut StyleRule)>(hasCssRules: &mut H, mut useStyleRule: UseStyleRule)
//	{
//		use ::css::domain::CssRule::*;
//
//		for cssRule in hasCssRules.css_rules_vec_mut().iter_mut()
//		{
//			match *cssRule
//			{
//				Style(ref mut styleRule) => useStyleRule(styleRule),
//
//				Media(ref mut media) => Self::operateOnStyleRules(media, useStyleRule),
//
//				Supports(ref mut supports) => Self::operateOnStyleRules(supports, useStyleRule),
//
//				Document(ref mut document) => Self::operateOnStyleRules(document, useStyleRule),
//
//				_ =>
//				{
//				}
//			}
//		}
//	}
//
//	pub fn x(filePath: &Path, stylesheet: &mut Stylesheet) -> Result<(), CordialError>
//	{
//		let dom = filePath.fileContentsAsHtmlDom()?;
//
//		dom.verify(filePath)?;
//
//		dom.recursivelyStripNodesOfCommentsAndProcessingInstructionAndCreateSaneDocType(filePath)?;
//
//		Self::operateOnStyleRules(stylesheet, |mut styleRule|
//		{
//			// do something with an &mut StyleRule...
//		});
//
//		/*
//			CSS
//				- strip all namespaced selectors (HTML5 is not XHTML)
//		*/
//
//		// TODO: Sort element attributes; check id has single value; sort class attribute
//
//		// TODO: Remove extra spaces in class attributes, img src, and others...
//
//		// TODO: Custom serialization to eliminate unnecessary " and ' in attributes
//
//		// TODO: convert all node names, attributes to lower case (can be done at serialization time)
//
//		filePath.createFileWithHtmlDom(&dom.document).context(filePath)?;
//		Ok(())
//	}
}
