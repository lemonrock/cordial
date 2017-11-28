// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use super::ResourceTag::*;
use webserver::headers::commonCacheControlHeader;
use ::chrono::NaiveTime;


include!("FacebookId.rs");
include!("FacebookOpenGraph.rs");
include!("FacebookOpenGraphAlbum.rs");
include!("FacebookOpenGraphAlbumReleaseType.rs");
include!("FacebookOpenGraphArticleContentTier.rs");
include!("FacebookOpenGraphAgeRestriction.rs");
include!("FacebookOpenGraphBusinessHours.rs");
include!("FacebookOpenGraphContentRestriction.rs");
include!("FacebookOpenGraphCountryRestriction.rs");
include!("FacebookOpenGraphDayOfWeek.rs");
include!("FacebookOpenGraphDeterminer.rs");
include!("FacebookOpenGraphProductGender.rs");
include!("FacebookOpenGraphProfileGender.rs");
include!("FacebookOpenGraphSong.rs");
include!("FacebookOpenGraphType.rs");
include!("FacebookOpenGraphTypeDiscriminant.rs");
include!("FacebookOpenGraphVideo.rs");
include!("FacebookOpenGraphVideoActor.rs");
include!("HtmlAbstract.rs");
include!("HtmlDocumentData.rs");
include!("HtmlOutputFormat.rs");
include!("HtmlUrls.rs");
include!("TwitterAtHandle.rs");
include!("TwitterCard.rs");
include!("TwitterCardImageMatch.rs");
include!("TwitterCardType.rs");
include!("TwitterCardAppReference.rs");
