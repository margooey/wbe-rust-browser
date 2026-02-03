pub mod url;

use crate::url::*;

fn main() {
    let url = Url::new(HOMEPAGE);
    url.request();
}
