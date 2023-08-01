use quickbooks_types::VendorBuilder;

fn main() {
    let ven = VendorBuilder::default().title("Nana").build().unwrap();
    println!("{ven}");
    println!("{:?}", ven);
}
