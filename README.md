# cordial

[cordial] is a rust crate.


## Licensing

The license for this project is MIT.

[cordial]: https://github.com/lemonrock/cordial "cordial GitHub page"


## TODO

* Modify zopfli crate to allow specifying options
* Order of processing
	* SVG, Favicon, Image
	* Then HTML
	* Then sitemap(s), robots, etc, sitemap_index.xml, etc
		* As this will allow use to have image dimensions ready, etc
* Sitemaps
	* 50,000 URL / size limit respected
	* Translations
* SEO
* Passing configuration objects to handlebars
	* ? Handlebars in .resource.xml files for headers generation ?
	* Need a pure-ASCII solution - encoded URL
* Pipeline to generate img src sets
	* ie multiple resources from one input
* Loop over all at end
	* Make all files 0400
	* Make all folders 0500
* Errors
	* 404 not found
	* 503 Service Unavailable (for maintenance)
	* Various 30x pages - probably should not have any content at all
	* A default error page (to say sorry)
* Web server
	* Set rlimit on start up
	* Use hyper

### Ideas

* [svgbob](https://crates.io/crates/svgbob) - ASCII to SVG
	* Introduces the problem of combined pipelines
* Caching
	* Allow specification of Cache-Control max-age, private for js, css, jpg, etc
	* Allow specification of Cache-Control no-cache for / and /file (HTML) pages
	* Create ETag values for all resources
	* Unique version URLs for all non-HTML resources (ie sub-resources)

### Notes

#### Test program for hyper URI parsing

```rust
extern crate hyper;

use hyper::Uri;
use std::str::FromStr;

fn main()
{
    // an asterisk-only URI can be distinguished by having a path of just '*'
    printUri("*");
    printUri("http://stormmq.com/*");
    printUri("http://stormmq.com*");
    
    printUri("http://stormmq.com");
    printUri("http://stormmq.com/");
    printUri("http://stormmq.com/about/");
    printUri("HTTP://stormmq.com/about/");
    printUri("http://stormmq.com/has%20a space in it/");
    printUri("/path/to/resource");
    
    // A path that begins with a double-slash is invalid
    printUri("//stormmq.com/no/scheme");
    
    
    
    printUri("/path/to/resource?query=10&other=15");
    
    // fragments are dropped
    printUri("/path/to/resource?query=10&other=15#fragment");
}

fn printUri(uri: &str)
{
    let uri = Uri::from_str(uri).expect("ok");
    
    println!("uri {}", uri);
    
    println!("absolute {}", uri.is_absolute());
    
    println!("scheme (case-insensitive) {:?}", uri.scheme());
    
    println!("authority {:?}", uri.authority());
    
    println!("authority/host {:?}", uri.host());
    
    println!("authority/port {:?}", uri.port());
    
    println!("path {}", uri.path());
    
    println!("query {:?}", uri.query());
    
    println!();
}
```
