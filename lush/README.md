<div align="center">

<img src="https://emoji2svg.deno.dev/api/🦊" alt="eyecatch" height="100">

# lush

The Lua scripting shell.

<br>
<br>


</div>

<table>
  <thead>
    <tr>
      <th style="text-align:center">🍔English</th>
    </tr>
  </thead>
</table>

<div align="center">

</div>

## ⛏️   Development

> **Note**
> require Lua5.4 library.
> I develop in Manjaro Linux. So, other enviromets may get some errors.
> If you get error, Please raise an issue. Thankyou!

```sh
# clone this repository
cargo run
```

### Devcontainer

This project support Devcontainer.
If you want to use on CLI, please follow this command.

> **Note**
> Requires [Devcontainer CLI](https://github.com/devcontainers/cli)
> If you use npm, run `npm install -g @devcontainers/cli`

```sh
# setup devcontainer
devcontainer up --workspace-folder .

# run debug build and execute
devcontainer exec --workspace-folder . cargo run
```

## 📝 Todo

- [ ] error handling
- [ ] ~~transplant UI to TUI~~ Use prompt library
- [ ] change to prompt library to rustyline

## 📜 License

MIT

### 🧩 Modules

- rlua

## 👏 Affected projects

- xonsh
