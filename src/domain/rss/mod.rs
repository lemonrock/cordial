// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use self::feedly::*;
use self::googleplay::*;
use self::itunes::*;
use ::std::str::from_utf8_unchecked;


pub(crate) mod feedly;
pub(crate) mod googleplay;
pub(crate) mod itunes;


include!("ArticleLanguageSpecificRssItemVariant.rs");
include!("HtmlDocumentItem.rs");
include!("HtmlDocumentItemVariant.rs");
include!("NonZeroNumber.rs");
include!("PodcastLanguageSpecificRssItemVariant.rs");
include!("PodcastRssChannel.rs");
include!("RssCategoryName.rs");
include!("RssChannel.rs");
include!("RssChannelName.rs");
include!("RssChannelLanguageSpecific.rs");
include!("RssItem.rs");
include!("StylesheetLink.rs");
