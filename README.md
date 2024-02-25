# New-to-me web experiments

This is just me experimenting with some new-to-me web tools, including

- shuttle (a rust foucsed solution for hosting rust apps),
- htmx ([https://htmx.org](https://htmx.org)),
- askama templating (I usually use Tera),
- turso (distributed edge SQLite for the new age, [https://turso.tech](https://turso.tech))

## Set up

You need to install shuttle and turso. Look at the websites for the latest
but at the moment this is done by

```bash
bash
curl -sSfL https://get.tur.so/install.sh | bash
exit
cargo install cargo-shuttle
```

## Running locally

```bash
cargo shuttle run
```

or if you have bacon installed (`cargo install bacon`) you can do

```bash
bacon
```

## Deployment

```bash
cargo shuttle deploy
```

If there are dirty changes in the repo

```bash
cargo shuttle deploy --ad
```
