// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use self::AudioCodec::*;
use self::VideoCodec::*;
use ::hyper::header::ContentType;
use ::mime::*;


include!("AudioCodec.rs");
include!("audioMp4TwitterContentType.rs");
include!("content_type_application_font_sfnt.rs");
include!("content_type_application_manifest_json_utf8.rs");
include!("content_type_application_xml_utf8.rs");
include!("content_type_audio_mp4.rs");
include!("content_type_font_sfnt.rs");
include!("content_type_font_ttf.rs");
include!("content_type_font_woff.rs");
include!("content_type_font_woff2.rs");
include!("content_type_image_gif.rs");
include!("content_type_image_jpeg.rs");
include!("content_type_image_png.rs");
include!("content_type_image_svg_xml_utf8.rs");
include!("content_type_text_css_utf8.rs");
include!("content_type_text_html_utf8.rs");
include!("content_type_text_plain_utf8.rs");
include!("content_type_text_vtt_utf8.rs");
include!("content_type_text_xml_utf8.rs");
include!("content_type_video_mp4.rs");
include!("content_type_video_webm.rs");
include!("mimeType.rs");
include!("VideoCodec.rs");
include!("videoMp4TwitterContentType.rs");
include!("webm8ContentType.rs");
