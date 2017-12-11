// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct MonArtist
{
	#[serde(default)] text_infer_id: bool,
	#[serde(default = "MonArtist::x_scale_default")] x_scale: u32,
	#[serde(default = "MonArtist::y_scale_default")] y_scale: u32,
	#[serde(default = "MonArtist::font_family_default")] font_family: String,
	#[serde(default = "MonArtist::font_size_default")] font_size: u32,
	#[serde(default)] show_gridlines: bool,
	#[serde(default)] infer_rect_elements: bool,
	#[serde(default)] name: Option<String>,
	#[serde(default)] table: Option<String>,
}

impl Default for MonArtist
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			text_infer_id: false,
			x_scale: Self::x_scale_default(),
			y_scale: Self::y_scale_default(),
			font_family: Self::font_family_default(),
			font_size: Self::font_size_default(),
			show_gridlines: false,
			infer_rect_elements: false,
			name: None,
			table: None,
		}
	}
}

impl MonArtist
{
	#[inline(always)]
	pub(crate) fn svgString(&self, inputContentFilePath: &Path, resourceUrl: &ResourceUrl, configuration: &Configuration) -> Result<String, CordialError>
	{
		let table = match self.table
		{
			None => Table::default(),
			Some(ref tableName) => Self::findTable(configuration, tableName)?,
		};
		
		let content = inputContentFilePath.fileContentsAsString().context(inputContentFilePath)?;
		
		let scene = content.parse::<Grid>()?.into_scene(&table, Some(SceneOpts { text_infer_id: self.text_infer_id, .. Default::default()}));
		
		let svgRender = SvgRender
		{
			x_scale: self.x_scale,
			y_scale: self.y_scale,
			font_family: self.font_family.clone(),
			font_size: self.font_size,
			show_gridlines: self.show_gridlines,
			infer_rect_elements: self.infer_rect_elements,
			name: match self.name
			{
				None => resourceUrl.withoutFileNameExtension().to_owned(),
				Some(ref name) => name.as_str().to_owned(),
			},
			format_table: table,
		};
		
		let svg = svgRender.render_s(&scene);
		let mut output = String::new();
		use ::std::fmt::Write;
		write!(&mut output, "{}", svg).unwrap();
		Ok(output)
	}
	
	#[inline(always)]
	fn findTable(configuration: &Configuration, tableName: &str) -> Result<Table, CordialError>
	{
		match tableName
		{
			"" => Err(CordialError::Configuration("An empty mon-artist table name is not permitted".to_owned())),
			"default" => Ok(Table::default()),
			"demo" => Ok(Table::demo()),
			_ =>
			{
				let rawFolderPath = &configuration.inputFolderPath;
				let tableFilePath = rawFolderPath.join("mon-artist-tables").join(format!("{}.txt", tableName));
				let file = File::open(&tableFilePath).context(tableFilePath)?;
				Ok(Table::from_lines(BufReader::new(file).lines()))
			}
		}
	}
	
	#[inline(always)]
	fn x_scale_default() -> u32
	{
		1
	}

	#[inline(always)]
	fn y_scale_default() -> u32
	{
		1
	}

	#[inline(always)]
	fn font_family_default() -> String
	{
		"monospace".to_string()
	}

	#[inline(always)]
	fn font_size_default() -> u32
	{
		13
	}
}
