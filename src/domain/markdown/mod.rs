// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::comrak::ComrakOptions;
use ::comrak::format_html;
use ::comrak::parse_document;
use ::comrak::nodes::{AstNode, NodeHtmlBlock, NodeValue};
use ::csv::Reader;
use ::std::str::from_utf8;
use ::svgbob::Grid;
use ::svgbob::Settings;
use ::typed_arena::Arena;


include!("AstNodeExt.rs");
include!("MarkdownParser.rs");
include!("MarkdownPlugin.rs");
