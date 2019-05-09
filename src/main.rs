/// This is just a small test program that won't be part of any official releases.

use bodhi::{
    BodhiService,
    BuildNVRQuery, BuildQuery,
    CommentIDQuery, CommentQuery,
    OverrideNVRQuery, OverrideQuery,
    PackageQuery,
    ReleaseNameQuery,
};

// TODO: make this configurable
const SERVER_URL: &str = "https://bodhi.fedoraproject.org";

fn main() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    /*
    let build = BuildNVRQuery::new(String::from("rust-1.34.1-1.fc29")).query(&bodhi);

    match build {
        Ok(build) => println!("Build: {:#?}", build),
        Err(error) => println!("Error: {:#?}", error),
    }

    let builds = BuildQuery::new()
        .nvr(String::from("rust-1.34.1-1.fc29"))
        .query(&bodhi);

    match builds {
        Ok(builds) => println!("Builds: {:#?}", builds),
        Err(error) => println!("Error: {:#?}", error),
    }

    let builds = BuildQuery::new()
        .package(String::from("rust"))
        .release(String::from("F29"))
        .query(&bodhi);

    match builds {
        Ok(builds) => println!("Builds: {:#?}", builds),
        Err(error) => println!("Error: {:#?}", error),
    }

    let comment = CommentIDQuery::new(19999).query(&bodhi);

    match comment {
        Ok(comment) => println!("Comment: {:#?}", comment),
        Err(error) => println!("Error: {:#?}", error),
    }

    let comments = CommentQuery::new()
        .user(String::from("decathorpe"))
        .package(String::from("kernel"))
        .query(&bodhi);

    match comments {
        Ok(comment) => println!("Comment: {:#?}", comment),
        Err(error) => println!("Error: {:#?}", error),
    }

    let r#override = OverrideNVRQuery::new(String::from("wingpanel-2.2.1-1.fc28")).query(&bodhi);

    match r#override {
        Ok(r#override) => println!("Override: {:#?}", r#override),
        Err(error) => println!("Error: {:#?}", error),
    }

    let overrides = OverrideQuery::new()
        .user(String::from("decathorpe"))
        .query(&bodhi);

    match overrides {
        Ok(overrides) => println!("Overrides: {:#?}", overrides),
        Err(error) => println!("Error: {:#?}", error),
    }

    let packages = PackageQuery::new().name(String::from("rust")).query(&bodhi);

    match packages {
        Ok(packages) => println!("Packages: {:#?}", packages),
        Err(error) => println!("Error: {:#?}", error),
    }
    */

    let release = ReleaseNameQuery::new(String::from("F30")).query(&bodhi);

    match release {
        Ok(release) => println!("Release: {:#?}", release),
        Err(error) => println!("Error: {:#?}", error),
    }
}
