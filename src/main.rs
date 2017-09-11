// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of cordial, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(unboxed_closures)]
#![recursion_limit="128"]


extern crate base64;
extern crate brotli2;
extern crate clap;
extern crate daemonize;
extern crate futures;
extern crate handlebars;
extern crate hyper;
extern crate image;
#[macro_use] extern crate log;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate maplit;
extern crate mktemp;
extern crate net2;
#[cfg(unix)] extern crate nix;
extern crate num_cpus;
extern crate ordermap;
extern crate oxipng;
extern crate radix_trie;
extern crate ring;
extern crate rustls;
extern crate sass_rs;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde_hjson;
extern crate stderr_logging;
extern crate svgcleaner;
extern crate svgdom;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_rustls;
extern crate tokio_signal;
extern crate url;
extern crate url_serde;
extern crate zero85;
extern crate zopfli;


use self::domain::*;
use ::clap::App;
use ::clap::Arg;
use ::clap::ArgMatches;
use ::rustls::Certificate;
use ::rustls::PrivateKey;
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
use ::std::io::BufWriter;
use ::std::io::Read;
use ::std::io::Write;
use ::std::os::unix::fs::PermissionsExt;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::Command;
use ::std::process::Stdio;
use ::std::process::exit;
use ::stderr_logging::StandardErrorAnsiLog;
use ::quick_error::ResultExt;


pub(crate) mod domain;
pub(crate) mod hjson;
pub(crate) mod webserver;


include!("ArgMatchesExt.rs");
include!("CordialError.rs");
include!("fatal.rs");
include!("PathExt.rs");


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
	
	setUMaskToUserOnly();
	
	let (inputFolderPath, outputFolderPath) = canonicalizeInputAndOutputFolderPaths(uncanonicalizedInputFolderPath, uncanonicalizedOutputFolderPath);
	
	let settings = Settings::new(environment, inputFolderPath, outputFolderPath, isDaemon);
	
	
	if let Err(error) = settings.startWebserver()
	{
		fatal(format!("Could not start webserver '{}'", error), 1);
	}
}

fn setUMaskToUserOnly()
{
	#[cfg(unix)]
	{
		let mode = ::nix::sys::stat::Mode::from_bits(0o7077).unwrap();
		::nix::sys::stat::umask(mode);
	}
}

fn canonicalizeInputAndOutputFolderPaths(uncanonicalizedInputFolderPath: PathBuf, uncanonicalizedOutputFolderPath: PathBuf) -> (PathBuf, PathBuf)
{
	let canonicalizedInputFolderPath = match uncanonicalizedInputFolderPath.metadata()
	{
		Err(error) =>
		{
			fatal(format!("Could not read from --input {:?} because '{}'", uncanonicalizedInputFolderPath, error), 2);
		}
		Ok(metadata) =>
		{
			if !metadata.is_dir()
			{
				fatal(format!("--input {:?} is not a folder path", uncanonicalizedInputFolderPath), 2);
			}
			match uncanonicalizedInputFolderPath.canonicalize()
			{
				Err(error) => fatal(format!("Could not canonicalize --input {:?} because '{}'", uncanonicalizedInputFolderPath, error), 2),
				Ok(canonicalizedInputFolderPath) => canonicalizedInputFolderPath,
			}
		}
	};
	
	if !canonicalizedInputFolderPath.is_dir()
	{
		fatal(format!("Canonicalized input path {:?} is a not a folder", canonicalizedInputFolderPath), 1);
	}
	
	if let Err(error) = create_dir_all(&uncanonicalizedOutputFolderPath)
	{
		fatal(format!("Could not create --output {:?} because '{}'", canonicalizedInputFolderPath, error), 2);
	}
	
	let canonicalizedOutputFolderPath = match uncanonicalizedOutputFolderPath.canonicalize()
	{
		Err(error) => fatal(format!("Could not canonicalize --output {:?} because '{}'", canonicalizedInputFolderPath, error), 2),
		Ok(canonicalizedOutputFolderPath) => canonicalizedOutputFolderPath,
	};
	
	if let Err(error) = canonicalizedOutputFolderPath.makeUserOnlyWritableFolder()
	{
		fatal(format!("Could not make --output {:?} writable because '{}'", canonicalizedInputFolderPath, error), 2);
	}
	
	(canonicalizedInputFolderPath, canonicalizedOutputFolderPath)
}
