<div align="center">

# @dataconstruct/faking (cli)

Generate massive amounts of fake (but realistic) data for testing and development.

[![Latest Version](https://img.shields.io/crates/v/faking.svg)](https://crates.io/crates/faking)

If you want to use the faking library in either rust or javascript head over to [faking-lib](https://github.com/Data-Construct/faking)

</div>

## Playground

Schema files for the cli can be created through our webapp at [playground](https://www.dataconstruct.io/organizations/playground/schemas). 

Once you draw up your schema, click the cli button for the schema file output format.

## Usage

Single generator
```bash
> faking "I32" -n 10

-1858314450
-222718204
1350368207
1761043599
534217560
806407691
1799913151
-210038962
-857619727
259086971
```

Custom schema generation (JSON)
```bash
> faking -s cust_schema.json -n 5

[
        {
                "field-1-bool": bool,
                "filed-2-i8": -62,
                "field-3-obj-or-null": null,
                "field-4-obj": {
                        "inner-field-1-uuid": "886c6e12-3f5c-466a-8da3-b416caeffa79",
                        "inner-field-2-uuid": "a0946c18-24b0-471e-b996-6f1459de981e"
                }
        },
        ....
]
```

For a list of generators and their categories you can use the following command
```bash
faking list
```

## Getting Started Development

```bash
cargo run -- --help

cargo run -- -s .\schemas\testcase.json
```
