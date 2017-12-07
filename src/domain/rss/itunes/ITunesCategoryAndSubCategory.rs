// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


// From https://help.apple.com/itc/podcasts_connect/?lang=en#/itc9267a2f12
#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum ITunesCategoryAndSubCategory
{
	Arts,
		Design,
		#[serde(rename = "Fashion & Beauty")] Fashion_and_Beauty,
		Food,
		Literature,
		#[serde(rename = "Performing Arts")] Performing_Arts,
		#[serde(rename = "Visual Arts")] Visual_Arts,
	
	Business,
		#[serde(rename = "Business News")] Business_News,
		Careers,
		Investing,
		#[serde(rename = "Management & Marketing")] Management_and_Marketing,
		Shopping,
	
	Comedy,
	
	Education,
		#[serde(rename = "Educational Technology")] Educational_Technology,
		#[serde(rename = "Higher Education")] Higher_Education,
		#[serde(rename = "K-12")] K_12,
		#[serde(rename = "Language Courses")] Language_Courses,
		Training,
	
	#[serde(rename = "Games & Hobbies")] Games_and_Hobbies,
		Automotive,
		Aviation,
		Hobbies,
		#[serde(rename = "Other Games")] Other_Games,
		#[serde(rename = "Video Games")] Video_Games,
	
	#[serde(rename = "Government & Organizations")] Government_and_Organizations,
		Local,
		National,
		#[serde(rename = "Non-Profit")] Non_Profit,
		Regional,
	
	Health,
		#[serde(rename = "Alternative Health")] Alternative_Health,
		#[serde(rename = "Fitness and Nutrition")] Fitness_and_Nutrition,
		#[serde(rename = "Self-Help")] Self_Help,
		Sexuality,
	
	#[serde(rename = "Kids & Family")] Kids_and_Family,
	
	Music,
	
	#[serde(rename = "News & Politics")] News_and_Politics,
	
	#[serde(rename = "Religion & Spirituality")] Religion_and_Spirituality,
		Buddhism,
		Christianity,
		Hinduism,
		Islam,
		Judaism,
		Other,
		Spirituality,
	
	#[serde(rename = "Science & Medicine")] Science_and_Medicine,
		Medicine,
		#[serde(rename = "Natural Sciences")] Natural_Sciences,
		#[serde(rename = "Social Sciences")] Social_Sciences,
	
	#[serde(rename = "Society & Culture")] Society_and_Culture,
		History,
		#[serde(rename = "Personal Journals")] Personal_Journals,
		Philosophy,
		#[serde(rename = "Places & Travel")] Places_and_Travel,
	
	#[serde(rename = "Sports & Recreation")] Sports_and_Recreation,
		Amateur,
		#[serde(rename = "College & High School")] College_and_High_School,
		Outdoor,
		Professional,
	
	Technology,
		Gadgets,
		#[serde(rename = "Tech News")] Tech_News,
		Podcasting,
		#[serde(rename = "Software How-To")] Software_How_To,
	
	#[serde(rename = "TV & Film")] TV_and_Film,
}

impl Default for ITunesCategoryAndSubCategory
{
	#[inline(always)]
	fn default() -> Self
	{
		ITunesCategoryAndSubCategory::Technology
	}
}

impl ITunesCategoryAndSubCategory
{
	#[inline(always)]
	pub(crate) fn toCategoryStr(&self) -> &'static str
	{
		self.toCategory().toCategoryStr()
	}
	
	#[inline(always)]
	pub(crate) fn toCategory(&self) -> ITunesCategory
	{
		use self::ITunesCategoryAndSubCategory::*;
		
		match *self
		{
			Arts => ITunesCategory::Arts,
				Design => ITunesCategory::Arts,
				Fashion_and_Beauty => ITunesCategory::Arts,
				Food => ITunesCategory::Arts,
				Literature => ITunesCategory::Arts,
				Performing_Arts => ITunesCategory::Arts,
				Visual_Arts => ITunesCategory::Arts,
			
			Business => ITunesCategory::Business,
				Business_News => ITunesCategory::Business,
				Careers => ITunesCategory::Business,
				Investing => ITunesCategory::Business,
				Management_and_Marketing => ITunesCategory::Business,
				Shopping => ITunesCategory::Business,
			
			Comedy => ITunesCategory::Comedy,
			
			Education => ITunesCategory::Education,
				Educational_Technology => ITunesCategory::Education,
				Higher_Education => ITunesCategory::Education,
				K_12 => ITunesCategory::Education,
				Language_Courses => ITunesCategory::Education,
				Training => ITunesCategory::Education,
			
			Games_and_Hobbies => ITunesCategory::Games_and_Hobbies,
				Automotive => ITunesCategory::Games_and_Hobbies,
				Aviation => ITunesCategory::Games_and_Hobbies,
				Hobbies => ITunesCategory::Games_and_Hobbies,
				Other_Games => ITunesCategory::Games_and_Hobbies,
				Video_Games => ITunesCategory::Games_and_Hobbies,
			
			Government_and_Organizations => ITunesCategory::Government_and_Organizations,
				Local => ITunesCategory::Government_and_Organizations,
				National => ITunesCategory::Government_and_Organizations,
				Non_Profit => ITunesCategory::Government_and_Organizations,
				Regional => ITunesCategory::Government_and_Organizations,
			
			Health => ITunesCategory::Health,
				Alternative_Health => ITunesCategory::Health,
				Fitness_and_Nutrition => ITunesCategory::Health,
				Self_Help => ITunesCategory::Health,
				Sexuality => ITunesCategory::Health,
			
			Kids_and_Family => ITunesCategory::Kids_and_Family,
			
			Music => ITunesCategory::Music,
			
			News_and_Politics => ITunesCategory::News_and_Politics,
			
			Religion_and_Spirituality => ITunesCategory::Religion_and_Spirituality,
				Buddhism => ITunesCategory::Religion_and_Spirituality,
				Christianity => ITunesCategory::Religion_and_Spirituality,
				Hinduism => ITunesCategory::Religion_and_Spirituality,
				Islam => ITunesCategory::Religion_and_Spirituality,
				Judaism => ITunesCategory::Religion_and_Spirituality,
				Other => ITunesCategory::Religion_and_Spirituality,
				Spirituality => ITunesCategory::Religion_and_Spirituality,
			
			Science_and_Medicine => ITunesCategory::Science_and_Medicine,
				Medicine => ITunesCategory::Science_and_Medicine,
				Natural_Sciences => ITunesCategory::Science_and_Medicine,
				Social_Sciences => ITunesCategory::Science_and_Medicine,
			
			Society_and_Culture => ITunesCategory::Society_and_Culture,
				History => ITunesCategory::Society_and_Culture,
				Personal_Journals => ITunesCategory::Society_and_Culture,
				Philosophy => ITunesCategory::Society_and_Culture,
				Places_and_Travel => ITunesCategory::Society_and_Culture,
			
			Sports_and_Recreation => ITunesCategory::Sports_and_Recreation,
				Amateur => ITunesCategory::Sports_and_Recreation,
				College_and_High_School => ITunesCategory::Sports_and_Recreation,
				Outdoor => ITunesCategory::Sports_and_Recreation,
				Professional => ITunesCategory::Sports_and_Recreation,
			
			Technology => ITunesCategory::Technology,
				Gadgets => ITunesCategory::Technology,
				Tech_News => ITunesCategory::Technology,
				Podcasting => ITunesCategory::Technology,
				Software_How_To => ITunesCategory::Technology,
			
			TV_and_Film => ITunesCategory::TV_and_Film,
		}
	}
	
	#[inline(always)]
	pub(crate) fn toSubCategoryStr(&self) -> Option<&'static str>
	{
		use self::ITunesCategoryAndSubCategory::*;
		
		match *self
		{
			Design => Some("Design"),
			Fashion_and_Beauty => Some("Fashion & Beauty"),
			Food => Some("Food"),
			Literature => Some("Literature"),
			Performing_Arts => Some("Performing Arts"),
			Visual_Arts => Some("Visual Arts"),
			
			Business_News => Some("Business News"),
			Careers => Some("Careers"),
			Investing => Some("Investing"),
			Management_and_Marketing => Some("Management & Marketing"),
			Shopping => Some("Shopping"),
			
			Educational_Technology => Some("Educational Technology"),
			Higher_Education => Some("Higher Education"),
			K_12 => Some("K-12"),
			Language_Courses => Some("Language Courses"),
			Training => Some("Training"),
			
			Automotive => Some("Automotive"),
			Aviation => Some("Aviation"),
			Hobbies => Some("Hobbies"),
			Other_Games => Some("Other Games"),
			Video_Games => Some("Video Games"),
			
			Local => Some("Local"),
			National => Some("National"),
			Non_Profit => Some("Non-Profit"),
			Regional => Some("Regional"),
			
			Alternative_Health => Some("Alternative Health"),
			Fitness_and_Nutrition => Some("Fitness & Nutrition"),
			Self_Help => Some("Self-Help"),
			Sexuality => Some("Sexuality"),
			
			Buddhism => Some("Buddhism"),
			Christianity => Some("Christianity"),
			Hinduism => Some("Hinduism"),
			Islam => Some("Islam"),
			Judaism => Some("Judaism"),
			Other => Some("Other"),
			Spirituality => Some("Spirituality"),
			
			Medicine => Some("Medicine"),
			Natural_Sciences => Some("Natural Sciences"),
			Social_Sciences => Some("Social Sciences"),
			
			History => Some("History"),
			Personal_Journals => Some("Personal Journals"),
			Philosophy => Some("Philosophy"),
			Places_and_Travel => Some("Places & Travel"),
			
			Amateur => Some("Amateur"),
			College_and_High_School => Some("College & High School"),
			Outdoor => Some("Outdoor"),
			Professional => Some("Professional"),
			
			Gadgets => Some("Gadgets"),
			Tech_News => Some("Tech News"),
			Podcasting => Some("Podcasting"),
			Software_How_To => Some("Software How-To"),
			
			_ => None,
		}
	}
}
