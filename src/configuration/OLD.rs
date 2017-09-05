// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.



#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Resource
{
	hierarchy: Vec<String>,
	name: String,
	parentAbsoluteFolderPath: PathBuf,
	configuration: Config,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OutputForm
{
	HTML,
	AMP,
	PJAX,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Compression
{
	None,
	Gzip,
	Brotli,
}

#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LanguageCode
{
	iso_639_1_alpha_2_language_code: String,
	iso_3166_1_alpha_2_country_code: String,
}

/*
	Resources are output as follows
	
	site/
		<LANG_CODE>/
			amp/
				uncompressed/
					path/to/resource.html
				gzip/
				brotli/
			pjax/
			regular/
				HTML files
				Assets, eg JS, JSON, PNG, CSS, etc
			errors/
	
*/

impl Resource
{
	pub fn urlRelativeToBase(&self, outputForm: OutputForm, compression: Compression, languageCode: &LanguageCode) -> String
	{
		let languages: languages = self.configuration.get("languages").expect("Could not obtain [languages] from configuration");
		
		
		let languages = self.configuration.get_table("languages").expect("languages not defined");
		let language = languages.get(&languageCode.iso_639_1_alpha_2_language_code).expect(&format!("language '{}' not available for resource '{:?}/{}'", hierarchy, &self.name));
		
		
	}
	
	pub fn loadResources(verbosity: Verbosity, inputSiteFolderPath: &Path, defaults: &Defaults) -> HashMap<Vec<String>, Resource>
	{
		let mut resources = HashMap::with_capacity(1024);
		
		let fileProcessor = |relativeEntryPath: &Path, fromPath: &Path|
		{
			match relativeEntryPath.file_name().unwrap().to_str()
			{
				None => Err(format!("file {:?} does not have a UTF-8 file name", fromPath)),
				Some(fileName) =>
					{
						const ending: &'static str = ".resource.toml";
						
						if fileName.ends_with(ending)
						{
							let name = fileName[0 .. fileName.len() - ending.len()];
							if name.is_empty()
							{
								return Err(format!("Empty resource names are not permitted, ie {:?} is not valid because it has a file name of '{}'", fromPath, ending))
							}
							
							let hierarchy = Self::hierarchy(relativeEntryPath);
							
							let mut configuration = defaults.findDefaultConfiguration(hierarchy);
							
							if let Err(error) = configuration.merge(relativeEntryPath.into())
							{
								return Err(format!("Could not load resource configuration from {:?} because '{:?}'", relativeEntryPath, error));
							}
							
							let mut key = hierarchy.clone();
							key.push(name.to_owned());
							
							let resource = Self
							{
								hierarchy,
								name: name.to_owned(),
								parentAbsoluteFolderPath: fromPath.parent().unwrap().to_path_buf(),
								configuration,
							};
							
							
							resources.insert(key, resource);
						}
					}
			}
		};
		
		verbosity.dieIfErrors(inputSiteFolderPath.iterateOverFoldersAndFiles(inputSiteFolderPath, fileProcessor), "Could not load resources");
		
		resources
	}
}
