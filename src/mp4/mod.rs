// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::mp4parse::MediaContext;
use ::mp4parse::read_mp4;
use ::mp4parse::SampleEntry;
use ::mp4parse::Track;
use ::mp4parse::VideoSampleEntry;
use ::std::io::Cursor;


include!("videoTrackDurationWidthAndHeight.rs");
include!("onlyAudioTrackDuration.rs");
