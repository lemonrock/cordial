// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of cordial, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub fn createOutputStructure(outputFolderPath: &Path) -> (PathBuf, PathBuf, PathBuf, PathBuf, PathBuf, PathBuf)
{
	if let Err(error) = create_dir_all(outputFolderPath)
	{
		fatal(format!("Could not create --output {:?} because '{}'", outputFolderPath, error), 2);
	}

	let canonicalizedOutputFolderPath = match outputFolderPath.canonicalize()
	{
		Err(error) => fatal(format!("Could not canonicalize --output {:?} because '{}'", outputFolderPath, error), 2),
		Ok(canonicalizedOutputFolderPath) => canonicalizedOutputFolderPath,
	};
	
	if let Err(error) = canonicalizedOutputFolderPath.makeUserOnlyWritableFolder()
	{
		fatal(format!("Could not make --output {:?} writable because '{}'", outputFolderPath, error), 2);
	}
	
	let cacheFolderPath = match canonicalizedOutputFolderPath.createSubFolder("cache")
	{
		Err(error) => fatal(format!("Could not create 'cache' folder inside --output {:?} because '{}'", outputFolderPath, error), 2),
		Ok(path) => path,
	};
	
	(
		cacheFolderPath,
		recreateFolder(&canonicalizedOutputFolderPath, "temporary"),
		recreateFolder(&canonicalizedOutputFolderPath, "site"),
		recreateFolder(&canonicalizedOutputFolderPath, "root"),
		recreateFolder(&canonicalizedOutputFolderPath, "errors"),
		recreateFolder(&canonicalizedOutputFolderPath, "pjax")
	)
}

fn recreateFolder(canonicalizedOutputFolderPath: &Path, subFolderName: &str) -> PathBuf
{
	match canonicalizedOutputFolderPath.recreateSubFolder(subFolderName)
	{
		Err(error) => fatal(format!("Could not recreate '{}' folder inside --output {:?} because '{}'", subFolderName, canonicalizedOutputFolderPath, error), 2),
		Ok(path) => path,
	}
}
