# Rust Project template

## Initialize

### Enter nix shell

#### Add direnv support (Optional)

```console
$ echo "use flake" > .envrc
$ direnv allow
```

#### (OR) Manually enter shell

```console
nix develop
```

### Initialize

```console
$ cargo init (--lib)
```

> It is recommended to gitignore `.envrc` and its `.direnv` directory, as it can be used to set personal environment settings.
> Also it is prefered to not add it to the remote `.gitignore` file but to the local `.git/info/exclude` one.

## Environment settings

### Rust version

This template uses [fenix](https://github.com/nix-community/fenix) to manage the rust toolchain.

To change the rust toolchain, edit the `latest` version tag in the `rust'` variable.
