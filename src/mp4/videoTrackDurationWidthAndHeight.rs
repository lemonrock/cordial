// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) fn videoTrackDurationWidthAndHeight(bytes: &[u8]) -> Result<(u16, u16, u64), CordialError>
{
	fn processVideoTrack(context: &MediaContext, track: &Track, video: &VideoSampleEntry) -> Result<(u16, u16, u64), CordialError>
	{
		let mut durationInSeconds = 0;
		
		let timescaleUnitsPerSecond = match track.timescale.map(|trackTimeScale| trackTimeScale.0)
		{
			Some(timescaleUnitsPerSecond) => Some(timescaleUnitsPerSecond),
			None => context.timescale.map(|contextTimeScale| contextTimeScale.0)
		};
		
		if let Some(timeUnits) = track.duration.map(|trackScaledTime| trackScaledTime.0)
		{
			if let Some(timescaleUnitsPerSecond) = timescaleUnitsPerSecond
			{
				// Shouldn't be allowed during the parse, but taking no chances.
				if timescaleUnitsPerSecond != 0
				{
					durationInSeconds = timeUnits / timescaleUnitsPerSecond;
				}
			}
		}
		
		let width = video.width;
		let height = video.height;
		
		return Ok((width, height, durationInSeconds))
	}
	
	let mut context = MediaContext::new();
	
	read_mp4(&mut Cursor::new(bytes), &mut context)?;
	
	if context.tracks.len() == 0 || context.tracks.len() > 2
	{
		return Err(CordialError::Configuration("There must be only one or two tracks in a video/mp4; strip it".to_owned()));
	}
	
	let firstTrack = context.tracks.get(0).unwrap();
	
	match firstTrack.data
	{
		Some(SampleEntry::Video(ref video)) => return processVideoTrack(&context, firstTrack, video),
		
		Some(SampleEntry::Audio(_)) => (),
		
		_ => return Err(CordialError::Configuration("There must be only audio and video tracks in a video/mp4; strip it".to_owned()))
	}
	
	let secondTrack = context.tracks.get(1).unwrap();
	
	match secondTrack.data
	{
		Some(SampleEntry::Video(ref video)) => processVideoTrack(&context, firstTrack, video),
		
		_ => Err(CordialError::Configuration("There must be only audio and video tracks in a video/mp4; strip it".to_owned()))
	}
}
