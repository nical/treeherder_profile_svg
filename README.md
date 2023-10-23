# treeherder_profile_svg
A quick and dirty tool to turn treeherder's resource-usage.json into an SVG

The tool takes the path to the json file as parameter and prints its output to stdout.

```
cargo run --release path/to/resource-usage.json > profile.svg
```

To grab the resource-usage.json file from treeherder:
 - Select a job
 - Go to `Artifact and debugging tools`
 - `resource-usage.json`
