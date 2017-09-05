// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of cordial, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]


extern crate brotli2;
extern crate clap;
extern crate image;
#[macro_use] extern crate log;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate maplit;
extern crate mktemp;
#[cfg(unix)] extern crate nix;
extern crate num_cpus;
extern crate oxipng;
extern crate sass_rs;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde_hjson;
extern crate stderr_logging;
extern crate svgcleaner;
extern crate svgdom;
extern crate url;
extern crate url_serde;
extern crate zopfli;

use self::configuration::*;
use self::configuration::domain::*;
use ::clap::App;
use ::clap::Arg;
use ::clap::ArgMatches;
use ::std::collections::BTreeSet;
use ::std::ffi::OsStr;
use ::std::ffi::OsString;
use ::std::fs;
use ::std::fs::create_dir_all;
use ::std::fs::File;
use ::std::fs::remove_dir;
use ::std::fs::remove_file;
use ::std::fs::set_permissions;
use ::std::io;
use ::std::io::BufReader;
use ::std::io::Read;
use ::std::io::Write;
use ::std::os::unix::fs::FileTypeExt;
use ::std::os::unix::fs::PermissionsExt;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::Command;
use ::std::process::Stdio;
use ::std::process::exit;
use ::stderr_logging::StandardErrorAnsiLog;
use ::quick_error::ResultExt;


pub(crate) mod configuration;
pub(crate) mod hjson;


include!("ArgMatchesExt.rs");
include!("CordialError.rs");
include!("createOutputStructure.rs");
include!("fatal.rs");
include!("PathExt.rs");
include!("validateInputFiles.rs");


#[cfg(unix)]
fn setUMaskToUserOnly()
{
	let mode = ::nix::sys::stat::Mode::from_bits(0o7077).unwrap();
	::nix::sys::stat::umask(mode);
}

fn main()
{
	#[cfg(unix)]
	{
		setUMaskToUserOnly();
	}
	
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
		.get_matches();
	
	matches.configureStandardErrorLogging();
	
	let environment = matches.value_of("environment").unwrap_or("development");
	
	let inputFolderPath = matches.defaultPathForCommandLineOption("input", "./input");
	let canonicalizedInputFolderPath = validateInputFiles(inputFolderPath);
	
	let outputFolderPath = matches.defaultPathForCommandLineOption("output", "./output");
	let (cacheFolderPath, temporaryFolderPath, siteFolderPath, rootFolderPath, errorsFolderPath, pjaxFolderPath) = createOutputStructure(&outputFolderPath);
	
	let configuration = match Configuration::loadBaselineConfiguration(&canonicalizedInputFolderPath, environment)
	{
		Err(error) => fatal(format!("Could not load baseline configuration '{}'", error), 1),
		Ok(configuration) => configuration,
	};
	
	let resources = match DiscoverResources::discoverResources(&configuration, &canonicalizedInputFolderPath)
	{
		Err(error) => fatal(format!("Could not load resources '{}'", error), 1),
		Ok(configuration) => configuration,
	};
	
	//compileSassFiles(&canonicalizedInputFolderPath, &rootFolderPath);
	
	//extern crate handlebars;
	//#[macro_use] extern crate serde_json;
	//use handlebars::Handlebars;
	// let mut reg = Handlebars::new();
    // // render without register
    // println!(
    //     "{}",
    //     reg.template_render("Hello {{name}}", &json!({"name": "foo"}))
    //         .unwrap()
    // );
    //
    // // register template using given name
    // reg.register_template_string("tpl_1", "Good afternoon, {{name}}").unwrap();
    // println!("{}", reg.render("tpl_1", &json!({"name": "foo"})).unwrap());
}
