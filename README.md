# starsearch

A very simple web application to better search your starred GitHub repositories and find stuff blazingly fast! 💫🚀

This project actually emerged out of pure frustration with the starred repository search functionality built in to GitHub. [It is nearly impossible to find projects when the searched term is not exactly in the name or description of the repository.](https://github.com/zekroTJA/starsearch/assets/16734205/9752eb6d-d7df-442c-a315-2bd6b6c5d3bd) To fix this, I had the idea to simply index all starred repositories in a full text search friendly database (in this case [Meilisearch](https://www.meilisearch.com/)) to be able to search through them via a very simple web application.

## Web App

starsearch can easily be queried directly via a simple, statically generated web interface.

https://github.com/zekroTJA/starsearch/assets/16734205/e236b40d-46ea-4008-90e9-d79a085c9e30

## CLI

You can also use the app directly from your terminal using the provided CLI!
Simply download the latest builds from the [releases page](https://github.com/zekroTJA/starsearch/releases).

Alternatively, if you have the Rust toolchain installed, you can use `cargo install` to build
and install the CLI locally.
```
cargo install --git https://github.com/zekroTJA/starsearch starsearch-cli
```

This will install the CLI under the executable name `starsearch-cli`. If you want
to change this, simply add an `alias` to your profile or use the following command
to rename the binary.
```
EXEC_PATH="$(which starsearch-cli)" mv $EXEC_PATH $(dirname $EXEC_PATH)/starsearch
```

![](.github/media/cli-demo.gif)

### Config Reference

You can create a `starsearch.toml` in your local users config directory to configure the behaviour of the CLI.

```toml
# The starsearch API endpoint
endpoint = "https://starsearch.exmaple.com"
# The default limit for results shown.
limit = 5
# The default view mode. Can be either "condensed"
# or "detailed".
display_mode = "detailed"
```

## Setup the Server

Currently, starsearch is statically configured to your GitHub account and does not provide OAUth login
or any user management. So, if you want to use it yourself, you need to set it up on your infrastructure.

> **Information**  
> Because it is built in rust, the service does not need a lot of resources to run. So a cheap VPS or
> Raspberry Pi should do the trick.

You can simply use the provided [`docker-compose.yml`](docker-compose.yml) to set up your instance.

Just make sure to enter your configuration in the environment variables of the `starsearch` service.

After that, just spin up the stack using the following command.
```
docker compose up -d
```

---

©2023 zekro Development (Ringo Hoffmann).
Covered by the [MIT License](LICENSE).