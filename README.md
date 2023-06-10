# starsearch

A very simple web application to better search your starred GitHub repositories and find stuff blazingly fast! 💫🚀

This project actually emerged out of pure frustration with the starred repository search functionality built in to GitHub. It is nearly impossible to find projects when the searched term is not exactly in the name or description of the repository. To fix this, I had the idea to simply index all starred repositories in a full text search friendly database (in this case [Meilisearch](https://www.meilisearch.com/)) to be able to search through them via a very simple web application.

<video src="https://github.com/zekroTJA/starsearch/assets/16734205/95571b26-a871-4aa1-a9d7-0bedcad6e0cd">

## Setup

You can simply use the provided [`docker-compose.yml`](docker-compose.yml) to set up your instance.

Just make sure to enter your configuration in the environment variables of the `starsearch` service.

After that, just spin up the stack using the following command.
```
docker compose up -d
```

---

©2023 zekro Development (Ringo Hoffmann).
Covered by the [MIT License](LICENSE).