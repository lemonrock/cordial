// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


pub(crate) struct DiscoverResources
{
	prefix: PathBuf,
	resourceTemplates: ResourceTemplates,
	resources: BTreeMap<String, Resource>,
}

impl DiscoverResources
{
	pub(crate) fn discover(configuration: &Configuration, inputFolderPath: &Path) -> Result<BTreeMap<String, Resource>, CordialError>
	{
		let prefix = inputFolderPath.join("root");
		let mut this = Self
		{
			prefix: prefix.clone(),
			resourceTemplates: ResourceTemplates::new(configuration),
			resources: BTreeMap::new(),
		};
		this.processRootFile(inputFolderPath)?;
		this.processFolder(&prefix)?;
		Ok(this.resources)
	}
	
	#[inline(always)]
	fn insertResource(&mut self, resource: Resource)
	{
		self.resources.insert(resource.resourceRelativeUrl().to_owned(), resource);
	}
	
	#[inline(always)]
	fn processFolder(&mut self, folderPath: &Path) -> Result<(), CordialError>
	{
		let mut hierarchy =
		{
			let relativeEntryPath = folderPath.strip_prefix(&self.prefix).unwrap();
			Self::hierarchy(relativeEntryPath.parent().unwrap())?
		};
		
		let overridesFolderPath = folderPath.join("overrides.hjson");
		let hjsonConfiguration = loadHjsonIfExtantAndMerge(&overridesFolderPath, self.resourceTemplates.find(hierarchy.as_slice()).clone())?;
		
		hierarchy.push(folderPath.utf8FileName()?);
		self.resourceTemplates.store(hierarchy, hjsonConfiguration);
		
		for entry in folderPath.read_dir().context(folderPath)?
		{
			let entry = entry.context(folderPath)?;
			
			let fromPath = entry.path();
			
			let fileType = entry.file_type().context(&fromPath)?;
			if fileType.is_dir()
			{
				self.processFolder(&fromPath)?;
			}
			else
			{
				self.processFile(&fromPath)?
			}
		}
		
		Ok(())
	}
	
	fn processRootFile(&mut self, inputFolderPath: &Path) -> Result<(), CordialError>
	{
		let rootResourceFilePath = inputFolderPath.join("root.resource.hjson");
		let configurationHjson = loadHjsonIfExtantAndMerge(&rootResourceFilePath, self.resourceTemplates.resourceTemplate.clone())?;
		let mut resource: Resource = deserializeHjson(configurationHjson)?;
		resource.finishInitialization(Vec::new(), "root", inputFolderPath.to_path_buf());
		self.insertResource(resource);
		Ok(())
	}
	
	#[inline(always)]
	fn processFile(&mut self, filePath: &Path) -> Result<(), CordialError>
	{
		const ResourceFileEnding: &'static str = ".resource.hjson";
		
		let fileName = filePath.utf8FileName()?;
		if fileName.ends_with(ResourceFileEnding)
		{
			let resourceInputName = &fileName[0 .. fileName.len() - ResourceFileEnding.len()];
			if resourceInputName.is_empty()
			{
				return Err(CordialError::InvalidFile(filePath.to_path_buf(), "it has a file name which resolves to an empty resource input name".to_owned()));
			}
			
			let parentHierarchy =
			{
				let relativeEntryPath = filePath.strip_prefix(&self.prefix).unwrap();
				Self::hierarchy(relativeEntryPath.parent().unwrap())?
			};
			
			let hjsonConfiguration =
			{
				let parentHjsonConfiguration = self.resourceTemplates.find(parentHierarchy.as_slice());
				loadHjsonIfExtantAndMerge(filePath, parentHjsonConfiguration.clone())?
			};
			
			let mut resource: Resource = deserializeHjson(hjsonConfiguration)?;
			
			resource.finishInitialization(parentHierarchy.clone(), resourceInputName, filePath.parent().unwrap().to_path_buf());
			
			self.insertResource(resource);
		}
		
		Ok(())
	}
	
	fn hierarchy(relativeEntryPath: &Path) -> Result<Vec<String>, CordialError>
	{
		let mut hierarchy = Vec::with_capacity(8);
		for component in relativeEntryPath.components()
		{
			match component
			{
				Normal(name) => match name.to_str()
				{
					None => return Err(CordialError::InvalidFile(relativeEntryPath.to_path_buf(), "it contains a non UTF-8 component".to_owned())),
					Some(utf8Name) => hierarchy.push(utf8Name.to_owned()),
				},
				_ => panic!("Should not be possible"),
			}
		}
		Ok(hierarchy)
	}
}
