use serde_json;

// This is just used for testing
fn main() {
    let client = pubmed::Client::new();
    /*
        let ids = client
            .work_ids_from_query(&"\"10.1016/j.bpj.2008.12.3951\"".to_string(), 1000)
            .unwrap();
        let works = client.works(&ids);
    */
    let articles = client
        .articles(&vec![22595786]) //[22722859, 19348744, 25081398]
        .unwrap(); // 22722859,19348744,25081398
    if true {
        println!("{}", serde_json::to_string(&articles[0]).unwrap());
    }
}
