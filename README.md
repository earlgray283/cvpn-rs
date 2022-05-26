# cvpn-rs

a CLI application of VPN service at Faculty of Informatics, Shizuoka University (written in Rust 🦀)

## Install

TODO

## Features

1. list

Displays a list of files or directories in the specified path.

```shell
$ cvpn list /path/to/dir
         -  Wed Jul  7 00:00:00 2014   love
         -  Wed Aug 28 23:59:59 2013   poker
```

Also you can display by full path.

```shell
$ cvpn l /path/to/dir --name-only
/path/to/dir/love
/path/to/dir/poker
```

2. download

Download files from specified file paths.
Files are downloaded in parallel.

```shell
$ cvpn download /path/to/file1 /path/to/file2
```

The `--name-only` option of the `list` command allows you to download effortless.

```shell
$ cvpn l /path/to/dir --name-only | grep 'makabe' | xargs cvpn d
```