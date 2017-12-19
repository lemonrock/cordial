# cordial

[cordial] is a static webserver that atomically rebuilds an entire website before serving it. It also keeps a previous copy of the website, so that (re-) deployments don't cause any downtime; clients that were served a web page just before deployment carry on working. It does this by using versioned URLs for the sub-resources (assets) that a typical HTML page references (images, stylesheets and the like). And, unlike other generators, like Hugo, templates can use the full power of Lua, in markdown, SASS plugins and handlebars.

It also does a lot more to create a great experience for your users:-

* Zero downtime for website deployments;
	* When a new deployment is ready, it is atomically switched to
	* Meanwhile the old deployment is served
	* Versioned assets for the old deployment are available even when the new deployment is switched to, in cases where clients got a web page with versioned asset references just before the point of switch-over
* Serving web pages in more than one language is very easy
	* Even images and videos can be localized
	* All language content is spell-checked and grammar-checked
	* Internationally aware site maps and RSS feeds are supported, too
	* Localization can be by relative URL or by alternative server
* Support for PJAX is built-in, so everyone gets fast web pages;
* Support for AMP is built-in, so mobile users get a great experience;
* Support for Feedly, iTunes Podcasts and Google Play;
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
* Videos are assumed to be served as both MP4 and WebM, with full support for language-aware subtitles (WebVTT), video site maps and Twitter Player cards;
* Unnecessary CSS rules are stripped, CSS is auto-prefixed and invalid CSS detected before it is served;
* Support for SEO is trivial with Image & Video Site Maps, schema.org, Twitter Cards, Facebook OpenGraph, iTunes & Google Play Podcasts, Safari, BrowserConfig, Web App manifests, RSS and Bing mRSS support built-in;
* Templates can be extended using Lua and their output is always minified and validated;
* Text is always spell-checked and grammar-checked;
* HTTP OPTIONS support is built-in;
* If a resource is an index URL, and the user specifies a leaf, or vice versa, an automatic redirect is generated.
* Prevents development websites leaking into production:-
	* We disallow robot indexing using the 'X-Robots-Tag' header to prevent accidental exposure to search engines
	* We only turn on HSTS preloading in production
	* Additionally, all configuration is environment-aware

[cordial] also makes it easy for you to manage large amounts of content:-

* it deliberately separates front matter (configuration) for actual content, so you can write markdown with a markdown editor
* configuration is a hierarchical set of human-readable JSON (HJSON), mirroring your website, so you can 
* configuration can have production (and other) environment overrides, and separate secrets from data
* everything can go in source control
* HTTP headers can be generated using templates
* CSS is auto-prefixed, stripped and validated
* SVG can be generated markdown-like using [svgbob](https://github.com/ivanceras/svgbobrus), [barcode](), [mon-artist](https://github.com/pnkfelix/mon-artist/blob/master/src/lit/src/test_data.md), [qrcode](https://docs.rs/qrcode/0.5.0/qrcode/), [memes](https://docs.rs/memenhancer/0.1.0/memenhancer/) and [plotlib](); just change the input_format for the SVG pipeline.
* Custom XML, JSON, CSS, JSON-SEQ and HTML resources can be templated and minified
* Any raw data can be passed through

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

[cordial] tries hard to minify data:-

* Data URIs are encoded as either percent encoded or base64-encoded, and the MIME type omitted if possible (lossless coercion from UTF-8 and other to US-ASCII is not done).
* SVGs are cleaned and minified using the best techniques possible
* All PNGs are crushed and recompressed using zopfli
* JPEGs can be perceptually encoded using guetzli
* Fonts are encoded using brotli
* Maximal zopfli and brotli compression is applied to all resources
* PJAX is supported to serve only changed content
* CSS is stripped of unused content (purified) when served under AMP


## Restrictions

* Videos with a soundtrack in more than one language (dubbing), or locale-differing content, are not supported. However, videos can have alternative language subtitle tracks, artwork and placeholders (posters). Content restrictions are not enforced.
* [cordial] does not support `TRACE`, but this can be easily added if desired;
* When atomically upgrading to a new deployment, it is not possible to serve old versions if a host name has been retired or changed (this is because we will no longer have any HTTPS information)
* URL path segments containing `/` (technically possible because of percent-encoding) are not supported; this can be supported if required but the work is substantial and there is no simple, visually meaningful way (ie look at this file; that must be this URL) to map these to file system resources
* It is impossible to have both an index and a leaf resource for an URL end path segment (eg '' for `https://example.com/about/` and 'about' for `https://example.com/about`). In practice this is rarely an issue. [cordial] supports automatic redirects for one to the other (one can specify which way).
* It is impossible to have empty non-terminal path segments, eg `https://example.com/hello//about/` has an empty path segment in '//'. Leading empty path segments, eg `https://example.com//hello` (`//` before `hello`) are invalid anyway.
* `robots.txt` generation adds in whitespace that isn't strictly required but does so to try to keep consistency with human-edited files
* Generated GIF animations that have alternate sources (for image source sets) lack the `smallest_image` and `largest_image` UrlTags. These could be added but the code complexity may not be worthwhile.
* Not all URLs are validated for existence. This is because they are external (shortcodes, pingbacks).


## Notes on (fav) icons

* Android Chrome Web App Manifest icons:-
	* Should be for sizes 36x36, 48x48, 72x72, 96x96, 144x144, 192x192, 256x256, 384x384 and 512x512.
	* Should be PNG.
* MS Tile icons:-
	* Should be for sizes 70x70, 150x150, 310x310 and 310x150 (used by browserconfig.xml).
	* Should also be 144x144 for IE 10.
	* Should be PNG.
* Apple Safari icons, both precomposed and regular:-
	* Should be for sizes 57x57, 60x60, 72x72, 76x76, 114x114, 120x120, 144x144, 152x152 and 180x180.
	* Should be PNG.
	* Should have a default icon which is 180x180, ie largest.
	* There should also be a SVG mask for pinned tabs.
* Apple Touch icons:-
	* Should be for sizes 120x120, 180x180, 152x152 and 167x167.
	* Should be PNG.
* If favicon PNGs are supplied, then:-
	* Should be in sizes 16x16, 32x32, 192x192 and 194x194.
* If legacy ICOs are supplied, then:-
	* Should be one file containing 16x16, 32x32 and 48x48.

## Licensing

The license for this project is AGPL-3.0.



## TODO

* Redirect for primary language pages (if primary language is 'en', redirect '/en/' to '/')
* Error Templates / Content
	* 400 Bad Request - display a page very similar to 404 Not Found
	* 403 Forbidden
	* 404 Not Found
* Web server
	* Support start URLs in Web App Manifests that are visited with query strings such as `?utm_source=homescreen`
	* Create output and cache folders so that they are readable/writable by webserver user after dropping permissions
	* Generate access logs
* Additional SVG plugins
	* [octicons](https://docs.rs/octicons/0.1.1/octicons/)
	* [raster-retrace](https://crates.io/crates/raster-retrace)
* HTML Minification
	* Identify specific attributes that can be additionally optimised
		* Eg global attributes where some values can be just an empty attribute
		* Eg class and srcset, where some spacing can be eliminated
		* Remove leading and trailing space from some attributes (eg id, href, class, src, srcset, sizes, etc)
* schema.org (using JSON+LD)
	* breadcrumbs
	* We need to move to 3 article images
* AMP
	* amp-app-banner
		* Implement a markdown plugin
		* ? link rel="amp-manifest" ?
	* amp-analytics
	* amp-call-tracking (needs simple JSON responses)
		* Implement as a markdown plugin for telephone numbers
	* amp-gist
	* amp-user-notification / cookies
	* amp-accordion
	* amp-carousel
	* amp-sidebar
	* amp-iframe
		* eg for Google maps, see <https://www.ampproject.org/docs/reference/components/amp-iframe>
	* ?amp-lightbox? (not the say as amp-image-lightbox)
	* amp-youtube, amp-vimeo, amp-playbuzz etc
	* amp-timeago
	* amp-viz-vega
	* amp-social-share
	* amp-twitter
	* amp-gycat
	
* Formats
	* URLs: Minify to relative URLs
	* Non-HTML, language-aware resources should use link headers eg
		* Indicate the alternative links in HTTP headers using `Link:<http://es.example.com/document.pdf>;rel="alternate";hreflang="es",<http://en.example.com/document.pdf>;rel="alternate";hreflang="en",<http://de.example.com/document.pdf>;rel="alternate";hreflang="de"` as it is likely to be more efficient.
	* SVG
		* Specify width & height when used as img src to avoid a flash of unstyled content.
		* Do we want to support source set generation?
	* CSS
		* Tokenize CSS class names
			* Probably best done with a whitelist
		* Embedding images into the stylesheet as data-uris like `cssembed` does.
			* But nothing like cssembed for rust...
	* SVG to PNG 
		* for organization-logo (feedly, google) and favicon
		* Use rust crate `librsvg`
	* Raster Images
		* Generate images suitable for Google VR View
	* HTML
		* Spellchecking using [languagetool](https://www.languagetool.org/)
		* Make sure there is at least 300 words of content on every page.
		* Validate title and description length
			* Do we know what facebook prefers?
		* Use `preconnect prefetch preload prerender dns-prefetch` link rel hints for non-AMP pages.
		* Generate `manifest`, `amp-manifest` and `icon` link rel tags.
		* Take HTML and run it through languagetool
		* Validate HtmlAbstract properties
		* Consider switching from description to title for anchor titles.
		* Embedded SVG
			* need to append / replace id, classes
			* strip xmlns="http://www.w3.org/2000/svg" namespace
		* Embedded images (data-uri)
			* Is it worthwhile?
* Modify zopfli crate to allow specifying options
* Videos / Podcasts
	* Support for opengraph
* Fonts
	* Use [ttfautohint](https://www.freetype.org/ttfautohint/); requires building FreeType ([eg](https://github.com/servo/libfreetype2/)) and HarfBuzz libraries ([wrapped for Rust](https://github.com/servo/rust-harfbuzz/blob/master/harfbuzz-sys/build.rs)), so tedious to add to [cordial]
	* Use [Open Type Sanitizer](https://github.com/khaledhosny/ots) to strip unnecessary metadata to make files smaller. Requires a bunch of dependencies, so tedious to add to [cordial]
	* Use Fontello's curl API to support Icon font creation
Other
	* Basic XSL stylesheet for podcast RSS feeds: <https://github.com/TheCraigHewitt/Seriously-Simple-Podcasting/blob/master/templates/feed-stylesheet.xsl>
	* [Google Cloud Podcasts](https://www.gcppodcast.com/)
	* Check <http://www.accessify.com/r/play.rust-lang.org> for page load speed analysis
	* <https://www.freepik.com/>
	* [BBC engineers on multi-lingual website design](http://responsivenews.co.uk/post/123104512468/13-tips-for-making-responsive-web-design)
	* Make sure [PureCSS](https://purecss.io/layouts/) will work
	* And also <https://duckduckgo.com/?q=language+menu+pure+css&t=ffab&ia=web>
	* [Google Web Designer](https://www.google.com/webdesigner/)
	* [Vega](https://vega.github.io/vega/)
	* [podgallery](http://podgallery.com/); also has a rather nice details view for podcasts
	
### Ideas
* Styling <https://userstyles.org/categories/site> - indicative of the top sites on the internet that people use regularly
* UX ideas <https://www.nomensa.com/blog/2010/7-tips-for-multi-lingual-website-accessibility>

[cordial]: https://github.com/lemonrock/cordial "cordial GitHub page"
