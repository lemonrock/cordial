// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


use super::*;
use ::mon_artist::render::svg::SvgRender;
use ::mon_artist::render::RenderS;
use ::mon_artist::grid::Grid;
use ::mon_artist::SceneOpts;
use ::mon_artist::format::Table;
use ::qrcode::QrCode;
use ::qrcode::render::svg::Color as SvgColor;
use ::qrcode::types::EcLevel;
use ::qrcode::types::Version;


include!("Meme.rs");
include!("MonArtist.rs");
include!("QrCodeData.rs");
include!("QrVersion.rs");
include!("QrErrorCorrectionLevel.rs");
