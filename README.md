# GET_VEC

The get_vec(...) method allows for async get requests of urls in a Vec<&str> form, e.g.    

    let api = API::new();

    #[threads(2)]
    api.get_vec(urls, DATA_PATH).await;

This would retrieve both files in the urls vec, and download them to the "src/" directory, using 2 async threads.

Included are the "header" and "threads" macro; header adding custom headers, and threads specifying the number of threads to open.
By default, threads is set to 1.

Below is an example:

    #[tokio::main]
    fn main() {
        let mut api = API::new(); // runner required to be mut for header changes
        let SAVE_PATH: &str = "./data";

        // Visit the first api, needing only the User-Agent (with only 1 thread)
        //

        let api_1 = vec![
            "www.endpoint_1.xml",
            "www.endpoint_2.json"
        ];

        #[header("User-Agent", "example@email_domain.com")]
        api.get_vec(api_1, ).await;
        


        // Visit the second api, now needing an API key; referred to as "API-Token"
        // (Opening 2 threads at once)
        //

        let api_2 = vec![
            "www.endpoint_3.xml",
            "www.endpoint_4.csv"
        ];

        #[threads(2)]
        #[header("User-Agent", "example@email_domain.com")]
        #[header("API-Token", "XXXXXXXXX")]
        api.get_vec(api_2, ).await;
    }

# TODO!
- [ ] request_per_second wrapper + proc_macro
- [ ] add logger (with boolean option)
- [ ] add thiserror
- [ ] determine if get_vec can be defined with generic types
