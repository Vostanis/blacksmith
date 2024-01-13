# THIS BUILD IS UNSAFE

get_vec() allows for async get requests of urls in a Vec<&str> form, e.g.

    let urls = vec![
        "www.endpoint_1.xml",
        "www.endpoint_2.json"
    ];
    get_vec(urls, "./src", 2).await;
    
AIM FOR A STRUCT REMAKE WITH A MACRO:

    let runner = Runner::new();

    #[header("User-Agent", "example@email_domain.com")]
    runner.get_vec(urls, ...).await;


This would retrieve both files in the urls vec, and download them to the "src/" directory, using 2 async threads.

The unsafety comes from the macros; using string as an initial intro to how they're built.
Syn parsing to do.
