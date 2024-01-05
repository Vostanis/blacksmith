# THIS BUILD IS UNSAFE

get_vec() is a useful fn that allows for async get requests of urls in a Vec<&str> form, e.g.

    let urls = vec![
        "www.endpoint_1.xml",
        "www.endpoint_2.json"
    ];
    get_vec(urls, "./src", 2).await;

This would retrieve both files and download them to the "src/" directory.

The unsafety comes from the macros; using string as an initial intro to how they're built.
Syn parsing to do.
