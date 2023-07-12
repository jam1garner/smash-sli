# smash-sli

A Rust library for reading and writing `soundlabelinfo.sli` files from Super Smash Bros. Ultimate.

```
cargo add smash-sli
```

## sli_yaml

A command-line program for creating and editing `soundlabelinfo.sli` files using YAML. Drag and drop a `soundlabelinfo.sli` file onto the executable to create a YAML file. Drag and drop a properly structured YAML file onto the executable to create a `soundlabelinfo.sli` file. YAML files are text files, so they can be viewed and edited in any text editor.

Sample output from a `soundlabelinfo.sli` file:

```yaml
entries:
- tone_name: se_silent
  nus3bank_id: 2000
  tone_id: 474
- tone_name: vc_popo_001
  nus3bank_id: 4014
  tone_id: 0
```

### Usage

The latest prebuilt binary for Windows is available in [Releases](https://github.com/jam1garner/smash-sli/releases/latest).

Download the latest set of [labels](https://github.com/ultimate-research/param-labels/blob/master/soundlabelinfo/Labels.txt) and have them placed beside the executable when dragging and dropping or include them in the command when converting to YAML. Missing labels will result in all `tone_name` values appearing as hashes.

`sli_yaml <input> [output]`<br>
`sli_yaml <input> [output] [label]`<br>
`sli_yaml soundlabelinfo.sli soundlabelinfo.yaml`<br>
`sli_yaml soundlabelinfo.sli soundlabelinfo.yaml Labels.txt`<br>
`sli_yaml soundlabelinfo.yaml soundlabelinfo.sli`<br>
