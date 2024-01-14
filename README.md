# GET_VEC

The get_vec(...) method allows for async get requests of urls in a Vec<&str> form, e.g.    

    let runner = Runner::new();
    runner.get_vec(urls, "./src", 2).await;

This would retrieve both files in the urls vec, and download them to the "src/" directory, using 2 async threads.

Included is the "header" macro, allowing for custom headers:

    let mut runner = Runner::new(); // runner required to be mut for header changes

    // Visit the first api, needing only the User-Agent
    let api_1 = vec![
        "www.endpoint_1.xml",
        "www.endpoint_2.json"
    ];
    #[header("User-Agent", "example@email_domain.com")]
    runner.get_vec(api_1, ...).await;
    
    // Visit the second api, now needing an API key; referred to as "API-Token"
    let api_2 = vec![
        "www.endpoint_3.xml",
        "www.endpoint_4.csv"
    ];
    #[header("User-Agent", "example@email_domain.com")]
    #[header("API-Token", "XXXXXXXXX")]
    runner.get_vec(api_1, ...).await;
