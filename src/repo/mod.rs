pub fn connect() {
    use mongodb::{Client, options::ClientOptions};

// Parse a connection string into an options struct.
    let mut client_options =
        ClientOptions::parse("mongodb+srv://rustrest:rustrestpwd@mongocluster-oshww.gcp.mongodb.net/test?retryWrites=true&w=majority").expect("Could not connect");

// Manually set an option.
    client_options.app_name = Some("My App".to_string());

// Get a handle to the deployment.
    let client = Client::with_options(client_options).expect(">Could not create client");

// List the names of the databases in that deployment.
    for db_name in client.list_database_names(None).expect("Name not found") {
        println!("{}", db_name);
    }
}