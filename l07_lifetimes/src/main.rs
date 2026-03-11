fn main() {
    // let value;
    //
    // {
    //     let point = 2.25;
    //     value = &point;
    // }
    //
    // println!("{}", value);
    let name = String::from("John Doe");
    let john = Account {
        customer_name: &name,
        balance: 1000f32,
    };

    println!("{} has balance {}", john.customer_name, john.balance);
    let settings = get_app_settings();
    println!("Server address is: {}", settings.server_address);
    println!("Connection String is: {}", settings.connecting_string);
}

struct Account<'a> {
    customer_name: &'a str,
    balance: f32,
}

fn get_app_settings() -> ApplicationSettings<'static> {
    ApplicationSettings {
        server_address: "localhost",
        connecting_string: "data source=Northwind;server:127.0.01",
    }
}

struct ApplicationSettings<'a> {
    server_address: &'a str,
    connecting_string: &'a str,
}
