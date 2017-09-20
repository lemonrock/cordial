// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct RssImage
{
	width: u32,
	height: u32,
	url: Url,
	fileSize: u64,
	mimeType: Mime,
	alt: String,
	credit: FullName,
}

/*
	A width of 680px seems to be ideal for Chrome to render

	<img width="680" height="406" src="https://tctechcrunch2011.files.wordpress.com/2017/09/nikola-two-bosch-pr.png?w=680" class="attachment-large size-large wp-post-image" alt="" srcset="https://tctechcrunch2011.files.wordpress.com/2017/09/nikola-two-bosch-pr.png?w=680 680w, https://tctechcrunch2011.files.wordpress.com/2017/09/nikola-two-bosch-pr.png?w=1358 1358w, https://tctechcrunch2011.files.wordpress.com/2017/09/nikola-two-bosch-pr.png?w=150 150w, https://tctechcrunch2011.files.wordpress.com/2017/09/nikola-two-bosch-pr.png?w=300 300w, https://tctechcrunch2011.files.wordpress.com/2017/09/nikola-two-bosch-pr.png?w=768 768w" sizes="(max-width: 680px) 100vw, 680px">
*/

// Google recommends primary images are at least Images should be at least 696 pixels wide; multiple images are recommended for google to index
// TechCrunch feed embeds the primary image to the HTML of the synopsis... along with a Read More link
// We can do this by pre-pending an <img> element and post-pending a Read More link
/*
    For best results, provide multiple high-resolution images (minimum of 300k pixels when multiplying width and height) with the following aspect ratios: 16x9, 4x3, and 1x1.

For example:

{
  "@context": "http://schema.org",
  "@type": "NewsArticle",
  "image": [
    "https://example.com/photos/1x1/photo.jpg",
    "https://example.com/photos/4x3/photo.jpg",
    "https://example.com/photos/16x9/photo.jpg"
   ]
  }

*/
