use fluent::{FluentBundle, FluentResource};
use unic_langid::langid;

pub fn create() {
    let ftl_string = "hello-world = Hello, world!".to_owned();
    let res = FluentResource::try_new(ftl_string).expect("Could not parse an FTL string.");

    
    let langid_en = langid!("en");
    let mut bundle = FluentBundle::new(vec![langid_en]);

    bundle
        .add_resource(&res)
        .expect("Failed to add FTL resources to the bundle.");

    let msg = bundle
        .get_message("hello-world")
        .expect("Failed to retrieve a message.");
    let val = msg.value().expect("Message has no value.");

    let mut errors = vec![];
    let value = bundle.format_pattern(val, None, &mut errors);
    
    
}
