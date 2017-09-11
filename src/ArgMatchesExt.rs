// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) trait ArgMatchesExt
{
	fn defaultPathForCommandLineOption(&self, key: &str, default: &str) -> PathBuf;
	
	fn configureStandardErrorLogging(&self);
}

impl<'a> ArgMatchesExt for ArgMatches<'a>
{
	fn defaultPathForCommandLineOption(&self, key: &str, default: &str) -> PathBuf
	{
		Path::new(self.value_of_os(key).unwrap_or(OsStr::new(default))).to_path_buf()
	}
	
	fn configureStandardErrorLogging(&self)
	{
		if let Err(error) = StandardErrorAnsiLog::initialise(self.is_present("quiet"), self.occurrences_of("verbosity") as usize)
		{
			eprintln!("Could not configure logging to standard error because '{}'", error);
			exit(1);
		}
	}
}
