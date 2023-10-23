use fluent::{FluentResource, FluentBundle, FluentArgs, FluentValue};
use unic_langid::LanguageIdentifier;

fn main() {
    let ftl_string = String::from(r#"
just-url-dynamic = content before link<a href="{$url}">this is a url</a>content after link
link-dynamic = content before link{$url}content after link"#);
    let res = FluentResource::try_new(ftl_string)
        .expect("Failed to parse an FTL string.");
    
    let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
    let mut bundle = FluentBundle::new(vec![langid_en]);
    
    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    // Testing what should be two identical ftl outputs strings.

    // Starting with a sentence with a url passed in. 
    let mut just_url_dynamic_arg = FluentArgs::new();
    just_url_dynamic_arg.set("url", FluentValue::from("https://projectfluent.org/"));
    
    let msg = bundle.get_message("just-url-dynamic")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let just_url_dynamic_value = bundle.format_pattern(&pattern, Some(&just_url_dynamic_arg), &mut errors);

    // Then instead of passing in the url, pass in the entire html link.
    let mut link_dynamic_arg = FluentArgs::new();
    link_dynamic_arg.set("url", FluentValue::from(r#"<a href="https://projectfluent.org/">this is a url</a>"#));

    let msg = bundle.get_message("link-dynamic")
    .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let link_dynamic_value = bundle.format_pattern(&pattern, Some(&link_dynamic_arg), &mut errors);
    
    // both indeed look the same.
    println!("Compare these too, they are the same right?");
    println!("{link_dynamic_value}\n{just_url_dynamic_value}");
    println!("But are they? See for yourself: {}", link_dynamic_value == just_url_dynamic_value);
    println!("Just try to open both urls in your browser.");

    // They aren't the same. What does it actually look like?
    println!("\n\nByte arrays:\n{:?}\n{:?}\n\n", link_dynamic_value.as_bytes(), just_url_dynamic_value.as_bytes());

    // YUP they indeed are different!
}
