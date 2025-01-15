# spore-dob-0

DOB0 协议旨在创建一个灵活的 DNA 字节渲染过程。作为 DOB 协议族的第一个实现，我们将其命名为 `DOB0`。

## 协议细节
DOB0 协议需要一个特征映射规则，用于定义 DNA 字节的哪一部分表示什么特征名称，以及从特征池中选择哪个特征值。

DOB0 协议要求 DOB 艺术家预先定义一个 DNA 特征池作为`pattern`，也即特征映射规则。DNA 中的每个单字节或一组字节将被解析为一个偏移指针，用于指向特征池中的具体特征项。所有选定的特征项的组合就是最终渲染的 DNA，例如：

```javascript
// Spore 中的 DNA 字节流
{
    contentType: "dob/0",
    content: {
        dna: "0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d"
    },
    // 或 content: "0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d",
    // 或 content: ["0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d"]
    content_id: "0x3b0e340b6c77d7b6e4f1fb2946d526ba65bfd196a27d9a7e5b6f06b82af5d07e"
}

// Cluster 中的 pattern 实例
{
    name: "DOBs collection",
    description: {
        description: "Unicorn Collection",
        dobs: {
            decoder: {
                type: "code_hash"// 或 "type_id" 或 "type_script",
                // (在使用 `code_hash` 或 "type_id" 的情况下)
                hash: "0x13cac78ad8482202f18f9df4ea707611c35f994375fa03ae79121312dda9925c",
                // (在使用 `type_script` 的情况下)
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

`0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d` 是 DNA 字节流，DOB0 解码器将逐个解析。`pattern: ...` 是由 Cluster 艺术家创建的特征映射规则，也将在 DOB0 解码器中同时解析。此外，该特征映射规则是一个 JSON 数组，参考链接：https://docs.spore.pro/dob0/protocol#pattern-definition。

解码器定位方法：
* `type: "code_hash"` 表示下面的 `hash` 是 DOB0 解码器 RISC-V 二进制文件的 CKB 个性化 blake2b 哈希。
* `type: "type_id"` 表示下面的 `hash` 是一个 `type_id` args 值，指向链上某个 cell，该 cell 的 data 字段存储 DOB0 解码器 RISC-V 二进制文件。
* `type: "type_script"` 使用下面的 `script` 作为指示器，通过其 `type_script` 指向链上解码器。

## 流程图
![plot](./assets/DOB0.jpg)

## 运行
安装 `ckb-vm-runner`：
```sh
$ git clone https://github.com/nervosnetwork/ckb-vm
$ cargo install --path . --example ckb-vm-runner
```

快速运行：

```sh
$ cargo run-riscv -- ac7b88 "[[\"Name\",\"String\",0,1,\"options\",[\"Alice\",\"Bob\",\"Charlie\",\"David\",\"Ethan\",\"Florence\",\"Grace\",\"Helen\"]],[\"Age\",\"Number\",1,1,\"range\",[0,100]],[\"Score\",\"Number\",2,1,\"rawNumber\"]]"

或

$ cargo build-riscv --release
$ ckb-vm-runner target/riscv64imac-unknown-none-elf/release/spore-dobs-decoder ac7b88 "[[\"Name\",\"String\",0,1,\"options\",[\"Alice\",\"Bob\",\"Charlie\",\"David\",\"Ethan\",\"Florence\",\"Grace\",\"Helen\"]],[\"Age\",\"Number\",1,1,\"range\",[0,100]],[\"Score\",\"Number\",2,1,\"rawNumber\"]]"


"[{\"name\":\"Name\",\"traits\":[{\"String\":\"Ethan\"}]},{\"name\":\"Age\",\"traits\":[{\"Number\":23}]},{\"name\":\"Score\",\"traits\":[{\"Number\":136}]}]"
```

如何集成：
1. 在后端服务器上安装 `ckb-vm-runner`
2. 在服务器代码中使用 `spore-dob-0` 二进制文件路径、DNA 和 pattern 参数调用 `ckb-vm-runner`（参考上面的快速运行）
3. 解析 JSON 特征结果

## 相关仓库
1. dob-standalone-decoder-server: https://github.com/sporeprotocol/dob-decoder-standalone-server
2. spore-dob-1: https://github.com/sporeprotocol/spore-dob-1

## 最新链上信息

`code_hash`: 0x13cac78ad8482202f18f9df4ea707611c35f994375fa03ae79121312dda9925c

`tx_hash`:
* 测试网: `0x4a8a0d079f8438bed89e0ece1b14e67ab68e2aa7688a5f4917a59a185e0f8fd5`
* 主网: `0x71023885a2178648be6a7f138ee49379000a82cda98dd8adabee99eaaca42fde`

`type_id`: None

如果文档未及时更新，可以到下面的文件查看最新的链上信息：
* 测试网： https://github.com/sporeprotocol/dob-decoder-standalone-server/blob/master/settings.toml#L65
* 主网： https://github.com/sporeprotocol/dob-decoder-standalone-server/blob/master/settings.mainnet.toml#L52

