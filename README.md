# file manager

批量修改 Windows UWP 版 Bilibili 客户端下载的视频。

#### 工作原理

Windows UWP 版 B 站客户端下载下来的视频无法直接播放，所以写了个批量处理工具，
移除文件起始 3 个 0xFF 字符，让普通播放器可以直接播放下载的视频。

目前使用笨办法实现，直接移除后再写入到新文件，再删除旧文件后重命名新文件到旧文件名，
等有空优化下。

## Usage

以下全部要在终端或者命令行工具内使用

### Windows 上如何使用

譬如在你的桌面，按住 Shift 键再对着桌面某个文件点鼠标右键可以复制路径，
把复制到的结尾文件名去掉就是可以使用的路径。

```bash
cargo run "C:\Users\改成你当前电脑的用户文件夹\Desktop"
```

### Linux/macOS/WSL2 使用方法

```bash
cargo run <some-path>
```
