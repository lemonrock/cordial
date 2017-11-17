// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of cordial, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(plugin)]
#![feature(unboxed_closures)]
#![plugin(phf_macros)]
#![recursion_limit="128"]


extern crate base64;
extern crate brotli2;
extern crate chardet;
extern crate chrono;
extern crate clap;
extern crate comrak;
extern crate css_autoprefix;
extern crate css_purify;
extern crate csv;
extern crate daemonize;
#[cfg(any(target_os = "android", target_os = "linux"))] extern crate dpdk_unix;
extern crate engiffen;
extern crate futures;
extern crate gif;
extern crate guetzli_sys;
extern crate handlebars;
extern crate hyper;
extern crate image;
extern crate libc;
#[macro_use] extern crate log;
#[macro_use] extern crate quick_error;
extern crate malloc_buf;
#[macro_use] extern crate maplit;
extern crate mktemp;
extern crate mime_guess;
extern crate mime_multipart;
extern crate mon_artist;
extern crate net2;
#[cfg(unix)] extern crate nix;
extern crate num_cpus;
extern crate ordermap;
extern crate oxipng;
extern crate phf;
extern crate qrcode;
extern crate radix_trie;
extern crate ring;
extern crate rustls;
extern crate sass_rs;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde_hjson;
extern crate sfnt2woff_zopfli_sys;
extern crate stderr_logging;
extern crate svgcleaner;
extern crate svgbob;
extern crate svgdom;
extern crate tendril;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_rustls;
extern crate tokio_signal;
extern crate typed_arena;
extern crate unicase;
extern crate url;
extern crate url_serde;
extern crate woff2_sys;
extern crate xml;
extern crate zero85;
extern crate zopfli;


use self::domain::*;
use self::domain::configure::*;
use self::domain::images::*;
use self::domain::inputFormats::*;
use self::domain::localization::*;
use ::clap::App;
use ::clap::Arg;
use ::clap::ArgMatches;
use ::handlebars::Handlebars;
use ::hyper::mime;
use ::hyper::mime::Mime;
use ::hyper::mime::Name;
use ::phf::Set as PhfSet;
use ::rustls::Certificate;
use ::rustls::PrivateKey;
use ::serde_json::Value as JsonValue;
use ::std::borrow::Cow;
use ::std::cmp::min;
use ::std::collections::BTreeMap;
use ::std::ffi::OsStr;
use ::std::ffi::OsString;
use ::std::fs;
use ::std::fs::create_dir_all;
use ::std::fs::File;
use ::std::fs::remove_dir;
use ::std::fs::remove_file;
use ::std::fs::set_permissions;
use ::std::io;
use ::std::io::BufRead;
use ::std::io::BufReader;
use ::std::io::BufWriter;
use ::std::io::Read;
use ::std::io::Write;
use ::std::os::unix::fs::PermissionsExt;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::exit;
use ::std::rc::Rc;
use ::stderr_logging::StandardErrorAnsiLog;
use ::quick_error::ResultExt;
use ::url::percent_encoding::percent_decode;
use ::url::Url;


pub(crate) mod domain;
pub(crate) mod hjson;
pub(crate) mod webserver;
pub(crate) mod woff;
pub(crate) mod xmlExtra;


include!("ArgMatchesExt.rs");
include!("CordialError.rs");
include!("fatal.rs");
include!("JsonValueExt.rs");
include!("MimeExt.rs");
include!("PathExt.rs");
include!("UrlExt.rs");


fn main()
{
	let matches = App::new("cordial")
		.version("0.0.0")
		.about("Creates static websites")
		.author("Raph Cohn (raphael.cohn@stormmq.com)")
		.arg
		(
			Arg::with_name("verbosity")
				.short("v")
				.long("verbose")
				.required(false)
				.help("Increase verbosity")
				.takes_value(false)
				.multiple(true)
		)
		.arg
		(
			Arg::with_name("quiet")
				.short("q")
				.long("quiet")
				.required(false)
				.help("Decrease verbosity")
				.takes_value(false)
				.multiple(false)
		)
		.arg
		(
			Arg::with_name("environment")
				.short("e")
				.long("environment")
				.required(false)
				.help("Environment to run in; defaults to development")
				.takes_value(true)
				.multiple(false)
				.value_name("ENVIRONMENT")
		)
		.arg
		(
			Arg::with_name("input")
				.short("i")
				.long("input")
				.required(false)
				.help("Folder containing website configuration, defaults to ./input")
				.takes_value(true)
				.multiple(false)
				.value_name("FOLDER_PATH")
		)
		.arg
		(
			Arg::with_name("output")
				.short("o")
				.long("output")
				.required(false)
				.help("Folder containing generate website, defaults to ./output")
				.takes_value(true)
				.multiple(false)
				.value_name("FOLDER_PATH")
		)
		.arg
		(
			Arg::with_name("daemon")
				.short("d")
				.long("daemon")
				.required(false)
				.help("Run in background as a daemon")
				.takes_value(false)
				.multiple(false)
		)
		.get_matches();
	
	matches.configureStandardErrorLogging();
	
	let isDaemon = matches.is_present("daemon");
	let environment = matches.value_of("environment").unwrap_or("development");
	let uncanonicalizedInputFolderPath = matches.defaultPathForCommandLineOption("input", "./input");
	let uncanonicalizedOutputFolderPath = matches.defaultPathForCommandLineOption("output", "./output");
	
	if let Err(error) = resourceLimits()
	{
		fatal(format!("Could not set resource limits '{}'", error), 1);
	}
	
	setUMaskToUserOnly();
	
	let settings = Settings::new(environment, uncanonicalizedInputFolderPath, uncanonicalizedOutputFolderPath, isDaemon);
	
	if let Err(error) = settings.startWebserver()
	{
		fatal(format!("Could not start webserver '{}'", error), 1);
	}
}


fn resourceLimits() -> Result<(), CordialError>
{
	#[cfg(any(target_os = "android", target_os = "linux"))]
	{
		use ::dpdk_unix::android_linux::resourceLimits::ResourceLimit;
		use ::dpdk_unix::android_linux::resourceLimits::ResourceLimitsSet;
		
		let procPath = PathBuf::from("/proc");
		let maximum_number_of_file_descriptors = ResourceLimit::maximumNumberOfFileDescriptors(&procPath).context(&procPath)?;
		let resourceLimits = ResourceLimitsSet::defaultish(maximum_number_of_file_descriptors);
		resourceLimits.change();
	}
	
	Ok(())
}

fn setUMaskToUserOnly()
{
	#[cfg(unix)]
	{
		use ::nix::sys::stat::*;
		let mode = Mode::from_bits(0o7077).unwrap();
		umask(mode);
	}
}
