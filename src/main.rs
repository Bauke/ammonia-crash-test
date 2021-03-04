fn main() {
    let mut sanitizer = ammonia::Builder::default();
    let sanitizer = sanitizer.url_relative(ammonia::UrlRelative::RewriteWithBase(
        ammonia::url::Url::parse("https://troyhunt.com").unwrap(),
    ));

    let unclean = std::fs::read_to_string("ammonia-crash.html").unwrap();
    sanitizer.clean(&unclean);
}
