# Týr

Tyr is a simple runtime for JavaScript and TypeScript that uses JavaScriptCore and is built in Rust.

[![JavaScriptCore Version](https://img.shields.io/badge/JavaScriptCore-wpewebkit%2F2.36.4-hotpink)](https://github.com/WebKit/WebKit/releases/tag/wpewebkit-2.36.4)

## Support Matrix

| Operating System | Architectures | Versions                   | Notes                                 | Status      |
| ---------------- | ------------- | -------------------------- | ------------------------------------- | ----------- |
| Linux            | x86_64        | glibc >= 2.17              | e.g. Ubuntu 14.04, Debian 9, CentOS 7 | ✅          |
| Linux            | arm64         | glibc >= 2.17              | e.g. Ubuntu 14.04, Debian 9, CentOS 7 | coming soon |
| Linux            | x86_64        | musl >= 1.1.19             | e.g. Alpine 3.8                       | coming soon |
| Linux            | arm64         | musl >= 1.1.19             | e.g. Alpine 3.8                       | coming soon |
| Linux            | armv7         | glibc >= 2.28              | e.g. Ubuntu 18.04, Debian 10          | coming soon |
| macOS            | x64           | \>= 10.15                  |                                       | ✅          |
| macOS            | arm64         | \>= 11                     |                                       | ✅          |
| Windows          | x64           | \>= Windows 10/Server 2016 |                                       | ✅          |
