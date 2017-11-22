// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[serde(deny_unknown_fields, default)]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct CleaningSettings
{
	remove_unused_defs: bool,
	convert_shapes: bool,
	remove_title: bool,
	remove_desc: bool,
	remove_metadata: bool,
	remove_dupl_linear_gradients: bool,
	remove_dupl_radial_gradients: bool,
	remove_dupl_fe_gaussian_blur: bool,
	ungroup_groups: bool,
	ungroup_defs: bool,
	group_by_style: bool,
	merge_gradients: bool,
	regroup_gradient_stops: bool,
	remove_invalid_stops: bool,
	remove_invisible_elements: bool,
	resolve_use: bool,
	
	remove_version: bool,
	remove_unreferenced_ids: bool,
	trim_ids: bool,
	remove_text_attributes: bool,
	remove_unused_coordinates: bool,
	remove_default_attributes: bool,
	remove_xmlns_xlink_attribute: bool,
	remove_needless_attributes: bool,
	remove_gradient_attributes: bool,
	join_style_attributes: CleaningSettingsStyleJoinMode,
	apply_transform_to_gradients: bool,
	apply_transform_to_shapes: bool,
	
	paths_to_relative: bool,
	remove_unused_segments: bool,
	convert_segments: bool,
	apply_transform_to_paths: bool,
	
	coordinates_precision: CleaningSettingsPrecision,
	properties_precision: CleaningSettingsPrecision,
	paths_coordinates_precision: CleaningSettingsPrecision,
	transforms_precision: CleaningSettingsPrecision,
}

impl Default for CleaningSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			remove_unused_defs: true,
			convert_shapes: true,
			remove_title: true,
			remove_desc: true,
			remove_metadata: true,
			remove_dupl_linear_gradients: true,
			remove_dupl_radial_gradients: true,
			remove_dupl_fe_gaussian_blur: true,
			ungroup_groups: true,
			ungroup_defs: true,
			group_by_style: true,
			merge_gradients: true,
			regroup_gradient_stops: true,
			remove_invalid_stops: true,
			remove_invisible_elements: true,
			resolve_use: true,
			
			remove_version: true,
			remove_unreferenced_ids: true,
			trim_ids: true,
			remove_text_attributes: true,
			remove_unused_coordinates: true,
			remove_default_attributes: true,
			remove_xmlns_xlink_attribute: true,
			remove_needless_attributes: true,
			remove_gradient_attributes: true,
			join_style_attributes: CleaningSettingsStyleJoinMode::default(),
			apply_transform_to_gradients: true,
			apply_transform_to_shapes: true,
			
			paths_to_relative: true,
			remove_unused_segments: true,
			convert_segments: true,
			apply_transform_to_paths: true,
			
			coordinates_precision: CleaningSettingsPrecision::default(),
			properties_precision: CleaningSettingsPrecision::default(),
			paths_coordinates_precision: CleaningSettingsPrecision::default(),
			transforms_precision: CleaningSettingsPrecision::default(),
		}
	}
}

impl CleaningSettings
{
	#[inline(always)]
	pub(crate) fn toSvgCleanOptions(&self) -> SvgCleanOptions
	{
		let mut options = SvgCleanOptions::default();
		
		options.remove_unused_defs = self.remove_unused_defs;
		options.convert_shapes = self.convert_shapes;
		options.remove_title = self.remove_title;
		options.remove_desc = self.remove_desc;
		options.remove_metadata = self.remove_metadata;
		options.remove_dupl_linear_gradients = self.remove_dupl_linear_gradients;
		options.remove_dupl_radial_gradients = self.remove_dupl_radial_gradients;
		options.remove_dupl_fe_gaussian_blur = self.remove_dupl_fe_gaussian_blur;
		options.ungroup_groups = self.ungroup_groups;
		options.ungroup_defs = self.ungroup_defs;
		options.group_by_style = self.group_by_style;
		options.merge_gradients = self.merge_gradients;
		options.regroup_gradient_stops = self.regroup_gradient_stops;
		options.remove_invalid_stops = self.remove_invalid_stops;
		options.remove_invisible_elements = self.remove_invisible_elements;
		options.resolve_use = self.resolve_use;
		
		options.remove_version = self.remove_version;
		options.remove_unreferenced_ids = self.remove_unreferenced_ids;
		options.trim_ids = self.trim_ids;
		options.remove_text_attributes = self.remove_text_attributes;
		options.remove_unused_coordinates = self.remove_unused_coordinates;
		options.remove_default_attributes = self.remove_default_attributes;
		options.remove_xmlns_xlink_attribute = self.remove_xmlns_xlink_attribute;
		options.remove_needless_attributes = self.remove_needless_attributes;
		options.remove_gradient_attributes = self.remove_gradient_attributes;
		//options.join_style_attributes = self.join_style_attributes.toStyleJoinMode;
		options.apply_transform_to_gradients = self.apply_transform_to_gradients;
		options.apply_transform_to_shapes = self.apply_transform_to_shapes;
		
		options.paths_to_relative = self.paths_to_relative;
		options.remove_unused_segments = self.remove_unused_segments;
		options.convert_segments = self.convert_segments;
		options.apply_transform_to_paths = self.apply_transform_to_paths;
		
		options.coordinates_precision = self.coordinates_precision.to_u8();
		options.properties_precision = self.properties_precision.to_u8();
		options.paths_coordinates_precision = self.paths_coordinates_precision.to_u8();
		options.transforms_precision = self.transforms_precision.to_u8();
		
		options
	}
}
