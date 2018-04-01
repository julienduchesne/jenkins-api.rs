# jenkins-api.rs [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Build Status](https://travis-ci.org/mockersf/jenkins-api.rs.svg?branch=master)](https://travis-ci.org/mockersf/jenkins-api.rs) [![Realease Doc](https://docs.rs/jenkins_api/badge.svg)](https://docs.rs/jenkins_api) [![Crate](https://img.shields.io/crates/v/jenkins_api.svg)](https://crates.io/crates/jenkins_api)

Bindings to [Jenkins JSON API](https://wiki.jenkins.io/display/JENKINS/Remote+access+API)

## Example

```rust
extern crate jenkins_api;

use jenkins_api::{JenkinsBuilder, BuildStatus};

fn main() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080")
        .with_user("user", Some("password"))
        .build()
        .unwrap();

    let job = jenkins.get_job("job name").unwrap();
    let build = job.last_build.unwrap().get_full_build(&jenkins).unwrap();

    println!(
        "last build for job {} at {} was {:?}",
        job.name, build.timestamp, build.result
    );

    if build.result == BuildStatus::Success {
        println!("triggering a new build");
        jenkins.build_job("job name").unwrap();
    }
}
```
