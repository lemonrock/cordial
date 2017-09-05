// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of cordial, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub fn validateInputFiles(inputFolderPath: PathBuf) -> PathBuf
{
	let canonicalizedInputFolderPath = match inputFolderPath.metadata()
	{
		Err(error) =>
		{
			fatal(format!("Could not read from --input {:?} because '{}'", inputFolderPath, error), 2);
		}
		Ok(metadata) =>
		{
			if !metadata.is_dir()
			{
				fatal(format!("--input {:?} is not a folder path", inputFolderPath), 2);
			}
			match inputFolderPath.canonicalize()
			{
				Err(error) => fatal(format!("Could not canonicalize --input {:?} because '{}'", inputFolderPath, error), 2),
				Ok(canonicalizedInputFolderPath) => canonicalizedInputFolderPath,
			}
		}
	};
	
	if !canonicalizedInputFolderPath.is_dir()
	{
		fatal(format!("Canonicalized input path {:?} is a not a folder", canonicalizedInputFolderPath), 1);
	}
	
	let mut errors = Vec::with_capacity(256);
	isFileValid(&mut errors, &canonicalizedInputFolderPath, &canonicalizedInputFolderPath);
	if !errors.is_empty()
	{
		for error in errors
		{
			error!("{}", error);
		}
		fatal("Input files are not valid", 1);
	}
	
	canonicalizedInputFolderPath
}

fn isFileValid(errors: &mut Vec<String>, canonicalizedInputFolderPath: &Path, path: &Path)
{
	let metadata = match path.symlink_metadata()
	{
		Err(_) =>
		{
			errors.push(format!("{:?} is unreadable", path));
			return;
		},
		Ok(metadata) => metadata,
	};
	
	let fileType = metadata.file_type();

	if fileType.is_symlink()
	{
		// is this an absolute symlink?
		let pointsTo = path.read_link().unwrap();
		if !pointsTo.is_relative()
		{
			errors.push(format!("{:?} is an absolute symlink (is not relative)", path));
			return;
		}
		if pointsTo.has_root()
		{
			errors.push(format!("{:?} is an absolute symlink (has a root)", path));
			return;
		}
		
		match path.canonicalize()
		{
			Err(_) =>
			{
				errors.push(format!("{:?} is a broken symlink", path));
			},
			Ok(canonicalPath) =>
			{
				if canonicalPath.strip_prefix(canonicalizedInputFolderPath).is_err()
				{
					errors.push(format!("{:?} is a symlink that points outside of input {:?} to {:?}", path, canonicalizedInputFolderPath, canonicalPath));
				}
			}
		}
	}
	else if fileType.is_dir()
	{
		match path.read_dir()
		{
			Err(error) => errors.push(format!("Could not read contents of folder {:?} because {}", path, error)),
			Ok(readDir) =>
			{
				for entry in readDir
				{
					match entry
					{
						Err(error) => errors.push(format!("Could not read entry in folder {:?} because {}", path, error)),
						Ok(entry) =>
						{
							let path = entry.path();
							isFileValid(errors, canonicalizedInputFolderPath, &path);
						}
					}
				}
			}
		};
		
	}
	else if fileType.is_file()
	{
	}
	else if fileType.is_block_device()
	{
		errors.push(format!("{:?} is a block device", path));
	}
	else if fileType.is_char_device()
	{
		errors.push(format!("{:?} is a char device", path));
	}
	else if fileType.is_char_device()
	{
		errors.push(format!("{:?} is a char device", path));
	}
	else if fileType.is_fifo()
	{
		errors.push(format!("{:?} is a FIFO", path));
	}
	else if fileType.is_socket()
	{
		errors.push(format!("{:?} is a socket", path));
	}
	else
	{
		errors.push(format!("{:?} is unknown (?Solaris Door?)", path));
	}
}
