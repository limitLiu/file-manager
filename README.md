# file manager

A simple batch rename file command line.

#### files and directory structure like this

```text
$ tree -L 3
.
├── 1
│   ├── 80
│   │   ├── audio.m4s
│   │   ├── index.json
│   │   └── video.m4s
│   ├── danmaku.xml
│   └── entry.json
├── 2
│   ├── 80
│   │   ├── audio.m4s
│   │   ├── index.json
│   │   └── video.m4s
│   ├── danmaku.xml
│   └── entry.json
└── 3
    ├── 80
    │   ├── audio.m4s
    │   ├── index.json
    │   └── video.m4s
    ├── danmaku.xml
    └── entry.json

6 directories, 15 files
```

#### result like this

```text
$ tree -L 3
.
├── audio_Desktop_test-dir_1_80.m4s
├── audio_Desktop_test-dir_2_80.m4s
├── audio_Desktop_test-dir_3_80.m4s
├── danmaku_Desktop_test-dir_1.xml
├── danmaku_Desktop_test-dir_2.xml
├── danmaku_Desktop_test-dir_3.xml
├── entry_Desktop_test-dir_1.json
├── entry_Desktop_test-dir_2.json
├── entry_Desktop_test-dir_3.json
├── index_Desktop_test-dir_1_80.json
├── index_Desktop_test-dir_2_80.json
├── index_Desktop_test-dir_3_80.json
├── video_Desktop_test-dir_1_80.m4s
├── video_Desktop_test-dir_2_80.m4s
└── video_Desktop_test-dir_3_80.m4s

0 directories, 15 files

```

## Usage

```bash
cargo run <some-path>
```
