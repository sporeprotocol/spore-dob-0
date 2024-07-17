# spore-dob-0

DOB0 protocol aims to create a flexiable rendering process of the DNA bytes. It's the first implementation in DOB protocol family, so we name it with the number `ZERO`.

## Protocol detail
DOB0 protocol requires a parsing pattern that helps to define which part of DNA bytes indicates what trait name and which trait value to select from the traits pool.

DOB0 protocol requires DOB artist to pre-define a collection DNA traits pool, as the pattern, and each single or batch bytes in DNA will be recongnized an offset pointer that to indicate a specific trait item in the pool. The combination of all selected trait items is the final rendered DNA, for instance:

```javascript
// DNA bytes in Spore 
{
    contentType: "dob/0",
    content: {
        dna: "0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d"
    },
    // or content: "0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d",
    // or content: ["0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d"]
    content_id: "0x3b0e340b6c77d7b6e4f1fb2946d526ba65bfd196a27d9a7e5b6f06b82af5d07e"
}

// Pattern instance in Cluster
{
    name: "DOBs collection",
    description: {
        description: "Unicorn Collection",
        dobs: {
            decoder: {
                type: "code_hash"// or "type_id" or "type_script",
                // (in case of `code_hash` or "type_id")
                hash: "0x13cac78ad8482202f18f9df4ea707611c35f994375fa03ae79121312dda9925c",
                // (in case of `type_script`)
                // script: {
                //     code_hash: "0x00000000000000000000000000000000000000000000000000545950455f4944",
                //     hash_type: "type",
                //     args: "0xf0b942b593a33f91fcbb9ea27c5a76b54afc048520e157cd0d2ba39403ece024"
                // }
            },
            pattern: [
                [
                    "Face",
                    "String",
                    0,
                    1,
                    "options",
                    ["Laugh", "Smile", "Sad", "Angry"]
                ],
                [
                    "Age",
                    "Number",
                    1,
                    1,
                    "range",
                    [0, 100]
                ],
                [
                    "BirthMonth",
                    "Number",
                    2,
                    1,
                    "options",
                    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                ],
                [
                    "Score",
                    "Number",
                    3,
                    1,
                    "rawNumber"
                ],
                [
                    "Identity",
                    "String",
                    4,
                    8,
                    "rawString"
                ]
                ...
            ]
        }
    }
}
```

`0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d` is the DNA bytes, which DOB0 decoder will parse one by one. `pattern: ...` is the pattern created by Cluster artist, which will be also parsed in DOB0 decoder in the meantime. In addition, the pattern is a JSON array, reference is here: https://docs.spore.pro/dob0/protocol#pattern-definition.

For real-world use case, this DOB0 decoder program is referenced by [decoder-template-rust](https://github.com/sporeprotocol/decoder-template-rust) and compiled into RISC-V binary. Then, we have two different methods to put it on-chain:
1. record the hash of binary on-chain, which refers to `code_hash`
2. deploy the binary into an on-chain CKB cell with `type_id` enabled, using its `type_script.args`

`type: "code_hash"` means the below `hash` is a CKB personalizied blake2b hash of DOB0 decoder RISC-V binary. To be contrast, `type: "type_id"` means the below `hash` is a `type_id` args value that points to an on-chain cell which keeps the DOB0 decoder RISC-V binary in ins `data` field. In addition, we recently added a new type `type: "type_script"`, which directly indicates the decoder through its type script.

## Diagram
![plot](./assets/DOB0.jpg)

## Run
Install `ckb-vm-runner`:
```sh
$ git clone https://github.com/nervosnetwork/ckb-vm
$ cargo install --path . --example ckb-vm-runner
```

For quick run:

```sh
$ cargo run-riscv -- ac7b88 "[[\"Name\",\"String\",0,1,\"options\",[\"Alice\",\"Bob\",\"Charlie\",\"David\",\"Ethan\",\"Florence\",\"Grace\",\"Helen\"]],[\"Age\",\"Number\",1,1,\"range\",[0,100]],[\"Score\",\"Number\",2,1,\"rawNumber\"]]"

or

$ cargo build-riscv --release
$ ckb-vm-runner target/riscv64imac-unknown-none-elf/release/spore-dobs-decoder ac7b88 "[[\"Name\",\"String\",0,1,\"options\",[\"Alice\",\"Bob\",\"Charlie\",\"David\",\"Ethan\",\"Florence\",\"Grace\",\"Helen\"]],[\"Age\",\"Number\",1,1,\"range\",[0,100]],[\"Score\",\"Number\",2,1,\"rawNumber\"]]"


"[{\"name\":\"Name\",\"traits\":[{\"String\":\"Ethan\"}]},{\"name\":\"Age\",\"traits\":[{\"Number\":23}]},{\"name\":\"Score\",\"traits\":[{\"Number\":136}]}]"
```

How to integrate:
1. install `ckb-vm-runner` into your back server natively
2. call `ckb-vm-runner` with the path of `spore-dob-0` binary, DNA and Pattern parameters in your server code (refer to above quick run)
3. parse the JSON traits result

## Related Repo
1. dob-standalone-decoder-server: https://github.com/sporeprotocol/dob-decoder-standalone-server
2. spore-dob-1: https://github.com/sporeprotocol/spore-dob-svg

## Latest On-chain Information

`code_hash`: 0x13cac78ad8482202f18f9df4ea707611c35f994375fa03ae79121312dda9925c

`tx_hash`:
* testnet: 0x4a8a0d079f8438bed89e0ece1b14e67ab68e2aa7688a5f4917a59a185e0f8fd5
* mainnet: 0x71023885a2178648be6a7f138ee49379000a82cda98dd8adabee99eaaca42fde

`type_id`: None
