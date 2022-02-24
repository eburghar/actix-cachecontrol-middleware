Define 1 middleware to control cache-control header under actix-web

`CacheHeader` takes a `CacheControl` structure (that can be deserialized with serde) that defines 2 maps of
paths to cache-control header values. One is used for matching prefixes, and the other for suffixes.

The example below defines a max-age of one year for every path starting with `/_app/`, and the other one, sets
no cache on every path ending with `.html` and `.json`.

```rust
use actix_cachecontrol_middleware::{data::CacheControl, middleware::CacheHeaders};
use actix_files::{Files, NamedFile};
use actix_web::{get, HttpResponse, HttpServer};

async fn serve() -> Result<()> {
    // Structure to drive the CacheHeadersMiddleware instanciated by CacheHeaders factory (can be deserialized with serde)
    let cache_control = CacheControl {
        prefixes: vec![("/_app/", "max-age=2678400")],
        suffixes: vec![(".html", "no-cache",".json":"no-cache")],
    };
    let server = HttpServer::new(move || {
        App::new()
            .wrap(CacheHeaders::new(cache_control.clone()))
            .service(Files::new("/", "/var/www").index_file("index.html"));
    // serve
    server.await?;
    Ok(())
}
```

Note that the deserialization from a dictionnary maintains order of the keys, and the same `CacheControl` structure
than the example above could be deserialized from :

```yaml
cache:
  prefixes:
    "_app": "max-age=2678400"
  suffices:
    ".html": "no-cache"
    ".json": "no-cache"
```

Suffixes are matched before prefixes, and the first match wins and sets the cache header.
