// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum FacebookOpenGraphTypeDiscriminant
{
	// All of these are from http://ogp.me/
	music_song,
	music_album,
	music_playlist,
	music_radio_station,
	video_movie,
	video_episode,
	video_tv_show,
	video_other,
	article,
	book,
	profile,
	website,
	
	// From https://developers.facebook.com/docs/reference/opengraph/object-type/business.business/
	business,
}
