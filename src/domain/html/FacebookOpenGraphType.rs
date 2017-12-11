// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum FacebookOpenGraphType
{
	music_song
	{
		musicians: Vec<ResourceUrl>,

		albums: Vec<FacebookOpenGraphAlbum>,

		duration_in_seconds: u64,

		isrc: Option<String>,

		previews: Vec<ResourceUrl>,

		release_date: DateTime<Utc>,

		release_type: FacebookOpenGraphAlbumReleaseType,
	},

	music_album
	{
		musicians: Vec<ResourceUrl>,

		release_date: DateTime<Utc>,

		release_type: FacebookOpenGraphAlbumReleaseType,

		songs: Vec<FacebookOpenGraphSong>,
	},

	music_playlist
	{
		creator: ResourceUrl,

		songs: Vec<FacebookOpenGraphSong>,
	},

	music_radio_station
	{
		creator: ResourceUrl,
	},

	video_movie
	{
		movie: FacebookOpenGraphVideo,
	},

	video_episode
	{
		episode: FacebookOpenGraphVideo,

		series: ResourceUrl,
	},

	video_tv_show
	{
		tv_show: FacebookOpenGraphVideo,
	},

	video_other
	{
		other: FacebookOpenGraphVideo,
	},

	article
	{
		authors: Vec<ResourceUrl>,

		content_tier: FacebookOpenGraphArticleContentType,

		publisher: Option<ResourceUrl>,

		section: String,

		#[serde(default)] tags: HashSet<String>,
	},

	book
	{
		authors: Vec<ResourceUrl>,

		isbn: Option<String>,

		release_date: DateTime<Utc>,

		#[serde(default)] tags: HashSet<String>,
	},

	profile
	{
		first_name: Option<String>,

		last_name: Option<String>,

		#[serde(default)] username: Option<String>,

		#[serde(default)] gender: Option<FacebookOpenGraphProfileGender>,
	},

	website,

	business
	{
		head_offices: HashMap<Iso639Dash1Alpha2Language, FacebookOpenGraphBusiness>,
	}

	// https://developers.facebook.com/docs/reference/opengraph/object-type/books.author/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/books.book/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/books.genre/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/fitness.course/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/game.achievement/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/place/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/product/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/product.group/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/product.item/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/restaurant.menu/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/restaurant.menu_item/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/restaurant.menu_section/
	// https://developers.facebook.com/docs/reference/opengraph/object-type/restaurant.restaurant/
}

impl Default for FacebookOpenGraphType
{
	#[inline(always)]
	fn default() -> Self
	{
		FacebookOpenGraphType::website
	}
}

impl FacebookOpenGraphType
{
	#[inline(always)]
	pub(crate) fn hasFacebookOpenGraphTypeDiscriminant(&self, facebookOpenGraphTypeDiscriminant: FacebookOpenGraphTypeDiscriminant) -> bool
	{
		use self::FacebookOpenGraphType::*;

		match *self
		{
			music_album { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::music_album,
			music_song { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::music_song,
			music_playlist { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::music_playlist,
			music_radio_station { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::music_radio_station,
			video_movie { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::video_movie,
			video_episode { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::video_episode,
			video_tv_show { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::video_tv_show,
			video_other { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::video_other,
			article { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::article,
			book { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::book,
			profile { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::profile,
			website => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::website,
			business { .. } => facebookOpenGraphTypeDiscriminant == FacebookOpenGraphTypeDiscriminant::business,
		}
	}

	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, languageData: &LanguageData, publicationDate: Option<DateTime<Utc>>, lastModificationDateOrPublicationDate: Option<DateTime<Utc>>, expirationDate: Option<DateTime<Utc>>) -> Result<(), CordialError>
	{
		fn addType(endHeadNodes: &mut Vec<UnattachedNode>, type_: &str)
		{
			endHeadNodes.push(meta_with_property_and_content("og:type", type_));
		}

		const MusicSongPreviewTag: ResourceTag = ResourceTag::audio_mp4;

		let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;

		use self::FacebookOpenGraphType::*;

		match *self
		{
			music_album { ref musicians, release_date, release_type, ref songs } =>
			{
				addType(endHeadNodes, "music.album");

				for musician in musicians.iter()
				{
					let url = musician.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::profile)?;
					endHeadNodes.push(meta_with_property_and_content("music:musician", url.as_str()));
				}

				endHeadNodes.push(meta_with_property_and_content("music:release_date", &release_date.to_rfc3339()));

				release_type.addTo(endHeadNodes);

				for song in songs
				{
					song.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
				}
			}

			music_song { ref musicians, ref albums, duration_in_seconds, ref isrc, ref previews, release_date, release_type } =>
			{
				addType(endHeadNodes, "music.song");

				for musician in musicians.iter()
				{
					let url = musician.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::profile)?;
					endHeadNodes.push(meta_with_property_and_content("music:musician", url.as_str()));
				}

				for album in albums.iter()
				{
					album.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
				}

				if duration_in_seconds != 0
				{
					endHeadNodes.push(meta_with_property_and_content("music:duration", &format!("{}", duration_in_seconds)));
				}

				if let &Some(ref isrc) = isrc
				{
					endHeadNodes.push(meta_with_property_and_content("music:isrc", isrc.as_str()));
				}

				for preview in previews.iter()
				{
					let previewResourceReference = ResourceReference
					{
						resource: preview.clone(),
						tag: MusicSongPreviewTag,
					};
					let urlData = previewResourceReference.urlDataMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;

					let url = urlData.url_str();
					endHeadNodes.push(meta_with_property_and_content("music:preview_url:url", url));
					endHeadNodes.push(meta_with_property_and_content("music:preview_url:secure_url", url));
					endHeadNodes.push(meta_with_property_and_content("music:preview_url:type", urlData.mimeType().as_ref()));
				}

				endHeadNodes.push(meta_with_property_and_content("music:release_date", &release_date.to_rfc3339()));

				release_type.addTo(endHeadNodes);
			}

			music_playlist { ref creator, ref songs } =>
			{
				addType(endHeadNodes, "music.playlist");

				let url = creator.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::profile)?;
				endHeadNodes.push(meta_with_property_and_content("music:creator", url.as_str()));

				for song in songs
				{
					song.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?;
				}

				endHeadNodes.push(meta_with_property_and_content("music:song_count", &format!("{}", songs.len())));
			}

			music_radio_station { ref creator } =>
			{
				addType(endHeadNodes, "music.radio_station");

				let url = creator.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::profile)?;
				endHeadNodes.push(meta_with_property_and_content("music:creator", url.as_str()));
			}

			video_movie { ref movie } =>
			{
				addType(endHeadNodes, "video.movie");

				movie.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, languageData)?;
			}

			video_episode { ref episode, ref series } =>
			{
				addType(endHeadNodes, "video.episode");

				episode.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, languageData)?;

				let url = series.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::video_tv_show)?;
				endHeadNodes.push(meta_with_property_and_content("video:series", url.as_str()));
			}

			video_tv_show { ref tv_show } =>
			{
				addType(endHeadNodes, "video.tv_show");

				tv_show.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, languageData)?;
			}

			video_other { ref other } =>
			{
				addType(endHeadNodes, "video.other");

				other.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, languageData)?;
			}

			article { ref authors, content_tier, ref publisher, ref section, ref tags } =>
			{
				addType(endHeadNodes, "article");

				for author in authors.iter()
				{
					let url = author.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::profile)?;
					endHeadNodes.push(meta_with_property_and_content("article:author", url.as_str()));
				}

				content_tier.addTo(endHeadNodes);

				if let Some(date) = publicationDate
				{
					endHeadNodes.push(meta_with_property_and_content("article:published_time", &date.to_rfc3339()));
				}

				if let Some(date) = lastModificationDateOrPublicationDate
				{
					endHeadNodes.push(meta_with_property_and_content("article:modified_time", &date.to_rfc3339()));
				}

				if let Some(date) = expirationDate
				{
					endHeadNodes.push(meta_with_property_and_content("article:expiration_time", &date.to_rfc3339()));
				}

				if let &Some(ref publisher) = publisher
				{
					let url = publisher.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::profile)?;
					endHeadNodes.push(meta_with_property_and_content("article:publisher", url.as_str()));
				}

				let translatedSection = languageData.facebookOpenGraphArticleSectionTranslation(section);
				endHeadNodes.push(meta_with_property_and_content("article:section", translatedSection));

				for tag in tags.iter()
				{
					let translatedTag = languageData.facebookOpenGraphArticleTagTranslation(tag);
					endHeadNodes.push(meta_with_property_and_content("article:tag", translatedTag));
				}
			}

			book { ref authors, ref isbn, ref release_date, ref tags } =>
			{
				addType(endHeadNodes, "book");

				for author in authors.iter()
				{
					let url = author.findUrlForFacebookOpenGraph(resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language, FacebookOpenGraphTypeDiscriminant::profile)?;
					endHeadNodes.push(meta_with_property_and_content("book:author", url.as_str()));
				}

				if let &Some(ref isbn) = isbn
				{
					endHeadNodes.push(meta_with_property_and_content("book:isbn", isbn.as_str()));
				}

				endHeadNodes.push(meta_with_property_and_content("book:release_date", &release_date.to_rfc3339()));

				for tag in tags.iter()
				{
					let translatedTag = languageData.facebookOpenGraphBookTagTranslation(tag);
					endHeadNodes.push(meta_with_property_and_content("book:tag", translatedTag));
				}
			}

			profile { ref first_name, ref last_name, ref username, gender } =>
			{
				addType(endHeadNodes, "profile");

				if let &Some(ref first_name) = first_name
				{
					endHeadNodes.push(meta_with_property_and_content("profile:first_name", first_name.as_str()));
				}

				if let &Some(ref last_name) = last_name
				{
					endHeadNodes.push(meta_with_property_and_content("profile:last_name", last_name.as_str()));
				}

				if let &Some(ref username) = username
				{
					endHeadNodes.push(meta_with_property_and_content("profile:username", username.as_str()));
				}

				if let Some(gender) = gender
				{
					endHeadNodes.push(meta_with_property_and_content("profile:gender", gender.to_str()));
				}
			}

			website => addType(endHeadNodes, "website"),

			business { ref head_offices } =>
			{
				let iso639Dash1Alpha2Language = languageData.iso639Dash1Alpha2Language;
				match head_offices.get(&iso639Dash1Alpha2Language)
				{
					Some(head_office) => head_office.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?,
					None => match head_offices.get(&fallbackIso639Dash1Alpha2Language)
					{
						Some(head_office) => head_office.addTo(endHeadNodes, resources, fallbackIso639Dash1Alpha2Language, iso639Dash1Alpha2Language)?,
						None => return Err(CordialError::Configuration("No fallback business defined".to_owned())),
					}
				}
			}
		}

		Ok(())
	}
}

#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FacebookOpenGraphBusiness
{
	street_address: String,
	city: String,
	#[serde(default)] region: Option<String>,
	postal_code: String,
	country_name: String,
	#[serde(default)] email: Option<EMailAddress>,
	#[serde(default)] phone: Option<String>,
	#[serde(default)] fax: Option<String>,
	#[serde(default)] website: ResourceUrl,

	#[serde(default)] opening_hours: BTreeMap<FacebookOpenGraphDayOfWeek, FacebookOpenGraphBusinessHours>,
}

impl FacebookOpenGraphBusiness
{
	pub(crate) fn addTo(&self, endHeadNodes: &mut Vec<UnattachedNode>, resources: &Resources, fallbackIso639Dash1Alpha2Language: Iso639Dash1Alpha2Language, iso639Dash1Alpha2Language: Iso639Dash1Alpha2Language) -> Result<(), CordialError>
	{
		const WebsiteTag: ResourceTag = ResourceTag::default;

		endHeadNodes.push(meta_with_property_and_content("business:contact_data:street_address", &self.street_address));

		endHeadNodes.push(meta_with_property_and_content("business:contact_data:city", &self.city));

		if let Some(ref region) = self.region
		{
			endHeadNodes.push(meta_with_property_and_content("business:contact_data:region", region.as_str()));
		}

		endHeadNodes.push(meta_with_property_and_content("business:contact_data:postal_code", &self.postal_code));

		endHeadNodes.push(meta_with_property_and_content("business:contact_data:country_name", &self.country_name));

		if let Some(ref email) = self.email
		{
			endHeadNodes.push(meta_with_property_and_content("business:contact_data:email", &email.to_string()));
		}

		if let Some(ref phone) = self.phone
		{
			endHeadNodes.push(meta_with_property_and_content("business:contact_data:phone", phone.as_str()));
		}

		if let Some(ref fax) = self.fax
		{
			endHeadNodes.push(meta_with_property_and_content("business:contact_data:fax", fax.as_str()));
		}

		let websiteResourceReference = ResourceReference
		{
			resource: self.website.clone(),
			tag: WebsiteTag,
		};
		let url = websiteResourceReference.urlMandatory(resources, fallbackIso639Dash1Alpha2Language, Some(iso639Dash1Alpha2Language))?;
		endHeadNodes.push(meta_with_property_and_content("business:contact_data:website", url.as_str()));

		for (dayOfWeek, hours) in self.opening_hours.iter()
		{
			dayOfWeek.addTo(endHeadNodes);
			hours.addTo(endHeadNodes)?;
		}

		Ok(())
	}
}
