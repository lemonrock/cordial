// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) fn onlyAudioTrackDuration(bytes: &[u8]) -> Result<u64, CordialError>
{
	let mut context = MediaContext::new();
	
	read_mp4(&mut Cursor::new(bytes), &mut context)?;
	
	if context.tracks.len() != 1
	{
		return Err(CordialError::Configuration("There must be only one track in an audio/mp4; strip it".to_owned()));
	}
	
	let audioTrack = context.tracks.get(0).unwrap();
	
	match audioTrack.data
	{
		Some(SampleEntry::Audio(_)) =>
		{
			let mut durationInSeconds = 0;
			
			let timescaleUnitsPerSecond = match audioTrack.timescale.map(|trackTimeScale| trackTimeScale.0)
			{
				Some(timescaleUnitsPerSecond) => Some(timescaleUnitsPerSecond),
				None => context.timescale.map(|contextTimeScale| contextTimeScale.0)
			};
			
			if let Some(timeUnits) = audioTrack.duration.map(|trackScaledTime| trackScaledTime.0)
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
			Ok(durationInSeconds)
		}
		
		_ => Err(CordialError::Configuration("There must be only one track in an audio/mp4, and it must be audio".to_owned()))
	}
}
