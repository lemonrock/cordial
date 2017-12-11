// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AudioVideoTrack
{
	pub(crate) kind: AudioVideoTrackKind,
	labels: HashMap<Iso639Dash1Alpha2Language, String>,
}

impl AudioVideoTrack
{
	pub(crate) fn bodyAndUrl(&self, languageData: &LanguageData, inputContentFilePath: &Path, resourceUrl: &ResourceUrl) -> Result<Option<(Vec<u8>, Url)>, CordialError>
	{
		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
		
		let webVttFilePath = inputContentFilePath.with_extension(format!("{:?}.{:?}.vtt", iso639Dash1Alpha2Language, self.kind));
		if let Some(webVttBody) = webVttFilePath.fileContentsAsBytesIfExtant().context(&webVttFilePath)?
		{
			let webVttUrl = resourceUrl.replaceFileNameExtension(&format!(".{:?}.vtt", self.kind)).url(languageData)?;
			Ok(Some((webVttBody, webVttUrl)))
		}
		else
		{
			Ok(None)
		}
	}
	
	//noinspection SpellCheckingInspection
	pub(crate) fn asNode(&self, isDefaultTrack: bool, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, webVttUrl: &Url) -> Result<UnattachedNode, CordialError>
	{
		let label = match self.labels.get(&iso639Dash1Alpha2Language)
		{
			None => return Err(CordialError::Configuration(format!("Missing language '{:?}' for track label", iso639Dash1Alpha2Language))),
			Some(label) => label,
		};
		
		let mut node = "track"
			.with_src_attribute(webVttUrl.as_ref())
			.with_attribute("srclang".str_attribute(iso639Dash1Alpha2Language.to_iso_639_1_alpha_2_language_code()))
			.with_attribute("label".str_attribute(label));
		
		if isDefaultTrack
		{
			node = node.with_empty_attribute("default");
		}
		
		use self::AudioVideoTrackKind::*;
		
		let trackNode = match self.kind
		{
			subtitles => node,
			captions => node.with_attribute("kind".str_attribute("captions")),
			descriptions => node.with_attribute("kind".str_attribute("descriptions")),
			chapters => node.with_attribute("kind".str_attribute("chapters")),
			metadata => node.with_attribute("kind".str_attribute("metadata")),
		};
		
		Ok(trackNode)
	}
}
