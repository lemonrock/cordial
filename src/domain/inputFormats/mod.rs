// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use super::markdown::MarkdownPlugin;
use super::pipelines::mimeType;
use super::svg::Barcode;
use super::svg::Meme;
use super::svg::MonArtist;
use super::svg::QrCodeData;
use ::woff2_sys::convertTtfToWoff2;


include!("CssInputFormat.rs");
include!("FontInputFormat.rs");
include!("HtmlInputFormat.rs");
include!("ImageInputFormat.rs");
include!("InputFormat.rs");
include!("SvgInputFormat.rs");
