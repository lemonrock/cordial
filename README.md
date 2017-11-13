# cordial

[cordial] is a static webserver that atomically rebuilds an entire website before serving it. It also keeps a previous copy of the website, so that (re-) deployments don't cause any downtime; clients that were served a web page just before deployment carry on working. It does this by using versioned URLs for the sub-resources (assets) that a typical HTML page references (images, stylesheets and the like). 

It also does a lot more to create a great experience for your users:-

* Zero downtime for website deployments;
	* When a new deployment is ready, it is atomically switched to
	* Meanwhile the old deployment is served
	* Versioned assets for the old deployment are available even when the new deployment is switched to, in cases where clients got a web page with versioned asset references just before the point of switch-over
* Serving web pages in more than one language is very easy
	* Even images can be localized
	* All language content is spell-checked and grammar-checked
	* Internationally aware site maps and RSS feeds are supported, too
	* Localization can be by relative URL or by alternative server
* Support for PJAX is built-in, so everyone gets fast web pages;
* Support for AMP is built-in, so mobile users get a great experience;
* Support for Feedly;
* Security and robustness are priorities
	* HTTP is always redirected to HTTPS, no ifs, no buts;
	* HTTPS in production can have HSTS preloading enabled for maximum protection of your users;
	* Only the latest TLSv1.2 cipher suites are used;
	* The server runs with minimal permissions and file system access
* HTTP headers are considered part of the web page, so you don't need horrible hacks
* Modern, correct caching rules are generated;
	* ETags are used
	* ETags are based on both content and headers, so only change when they have to
	* Unlike other servers, ETags don't change on every deployment
	* ETag changes cascade, so a web page that uses a stylesheets that uses sprites will have its ETag change if the sprites change
	* Last Modified dates based on deployment times are used as a fallback for less capable clients
* Images are first-class resources
	* Pipelines to resize, dither, optimize, crop, etc
	* Source sets (`srcset`) are automatically generated
		* See [this blog post](https://mattwilcox.net/web-development/keeping-srcset-and-sizes-under-control) to work out how many you'll need
	* Full integration for schema.org, image site maps, Feedly, Twitter Cards and Open Graph
	* Images can vary by language
	* Images are always converted and compressed to the best possible format using the latest optimizations (eg guetzli)
* All assets are minified and compressed using Brotli and Zopfli;
* Unnecessary CSS rules are stripped, CSS is auto-prefixed and invalid CSS detected before it is served;
* Support for SEO is trivial;
* Text is always spell-checked and grammar-checked
* HTTP OPTIONS support is built-in
* If a resource is an index URL, and the user specifies a leaf, or vice versa, an automatic redirect is generated.

[cordial] also makes it easy for you to manage large amounts of content:-

* it deliberately separates front matter (configuration) for actual content, so you can write markdown with a markdown editor
* configuration is a hierarchical set of human-readable JSON (HJSON), mirroring your website, so you can 
* configuration can have production (and other) environment overrides, and separate secrets from data
* everything can go in source control
* HTTP headers can be generated using templates
* CSS is auto-prefixed, stripped and validated

[cordial] is intended to be secure:-

* Permissions are dropped once ports are bound - the server can run as `nobody`
* No generated content is written to disk by default;
* Temporary file creation is secure;
* Capabilities (include Ambient ones) are dropped once sockets are bound (Linux & Android only);
* Lock bits are set set once sockets are bound (Linux & Android only);
* Process dumping is prevented (Linux & Android only);
* Assigning new privileges is prevented (Linux & Android only);
* `SECCOMP-BPF` is not used on Linux because it is too difficult currently to know which sycalls are in use.
	* See <https://github.com/seccomp/libseccomp>
* Folder and files, if created, are only usable by the current user, ie umask is `0o7077`.

[cordial] is intended to be robust when used as the only process on a server:-

* If a website deployment fails, [cordial] continues to serve the old content;
* The number of TLS sessions can be restricted;
* The number of connections (`rlimit`) is set as high as possible (Linux & Android only);
* HTTPS OCSP stapling is supported;


## Restrictions

* [cordial] does not support `TRACE`, but this can be easily added if desired;
* When atomically upgrading to a new deployment, it is not possible to serve old versions if a host name has been retired or changed (this is because we will no longer have any HTTPS information)
* URL path segments containing `/` (technically possible because of percent-encoding) are not supported; this can be supported if required but the work is substantial and there is no simple, visually meaningful way (ie look at this file; that must be this URL) to map these to file system resources
* It is impossible to have both an index and a leaf resource for an URL end path segment (eg '' for `https://example.com/about/` and 'about' for `https://example.com/about`). In practice this is rarely an issue. [cordial] supports automatic redirects for one to the other (one can specify which way).
* It is impossible to have empty non-terminal path segments, eg `https://example.com/hello//about/` has an empty path segment in '//'. Leading empty path segments, eg `https://example.com//hello` (`//` before `hello`) are invalid anyway.
* `robots.txt` generation adds in whitespace that isn't strictly required but does so to try to keep consistency with human-edited files
* Generated GIF animations that have alternate sources (for image source sets) lack the `smallest_image` and `largest_image` UrlTags. These could be added but the code complexity may not be worthwhile.


## Licensing

The license for this project is AGPL-3.0.



## TODO
* Redirect for primary language pages (if primary language is 'en', redirect '/en/' to '/')
* Redirect for leaf URLs to index URLs (eg '/en' to '/en/')
* Markdown / Handlebars / HTML minify
* Spellchecking using [languagetool](https://www.languagetool.org/)
* Error Templates / Content
	* 400 Bad Request - display a page very similar to 404 Not Found
	* 403 Forbidden
	* 404 Not Found
* Web server
	* Create output and cache folders so that they are readable/writable by webserver user after dropping permissions
	* Generate access logs
* combined pipelines, ie one pipeline feeds into another
	* eg [svgbob](https://crates.io/crates/svgbob) - ASCII to SVG
	* eg <https://crates.io/crates/mon-artist> - ASCII to SVG
	* eg [raster-retrace](https://crates.io/crates/raster-retrace) - images to SVG
	* eg [comic](https://crates.io/crates/comic)
	* eg [qrcode](https://crates.io/crates/qrcode)
	* eg [barcoders](https://github.com/buntine/barcoders) - generates barcode images
* Formats
	* SVG
		- adjust or set or remove width & height in document
		- support source set generation, also needed for Feedly
	* HTML
		* extract PJAX automatically with CSS selectors
		* Explore using [spongedown](https://ivanceras.github.io/spongedown/) because it allows creating charts and emoji faces; builds on [comrak](https://crates.io/crates/comrak), a commonmark and GitHub Flavoured Markdown renderer
	* GIF
		* engiffen
	* CSS
		* Embedding images into the stylesheet as data-uris
			* But nothing like cssembed for rust...
	* CSS extensions
		* something akin to css-embed for
			* images (particularly when used as sprites)
			* font-faces
				* webfont creation (may be problematic for AMP)
		* https://github.com/purifycss/purifycss
	* SVG to PNG - for organization-logo (feedly, google) and favicon
	* Favicon
		* Quick request library: `reqwest = "0.4"`
		* Svg2Png, then go from there, really. Multiple outputs.
	* [kuchiki](https://crates.io/crates/kuchiki) or [scraper](https://crates.io/crates/scraper) for manipulating HTML & XML with CSS selectors or [select](https://crates.io/crates/select)
* Modify zopfli crate to allow specifying options
* RSS
	* ?Register with Feedly, InoReader, Bazqux, The Older Reader and Feedbin?
	* Support itunes extensions for podcasts
	* <https://blog.feedly.com/10-ways-to-optimize-your-feed-for-feedly/>
* SEO
* Fonts
	* Use [ttfautohint](https://www.freetype.org/ttfautohint/); requires building FreeType ([eg](https://github.com/servo/libfreetype2/)) and HarfBuzz libraries ([wrapped for Rust](https://github.com/servo/rust-harfbuzz/blob/master/harfbuzz-sys/build.rs)), so tedious to add to [cordial]
	* Use [Open Type Sanitizer](https://github.com/khaledhosny/ots) to strip unnecessary metadata to make files smaller. Requires a bunch of dependencies, so tedious to add to [cordial]
	* Use Fontello's curl API to support Icon font creation
* SVG & Markdown extensions
	* [plotlib](https://github.com/milliams/plotlib) data sets to SVG graphs
	* svgbob
	* comic (ascii faces to SVG)
	* csv table from spongedown
	* [memenhancer](https://github.com/ivanceras/memenhancer)
* Image transformations need a variant of Borrow or Cow

### Ideas
* Styling <https://userstyles.org/categories/site> - indicative of the top sites on the internet that people use regularly
* UX ideas <https://www.nomensa.com/blog/2010/7-tips-for-multi-lingual-website-accessibility>

[cordial]: https://github.com/lemonrock/cordial "cordial GitHub page"
