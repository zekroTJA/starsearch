# starsearch

A very simple web application to better search your starred GitHub repositories and find stuff blazingly fast! 💫🚀

This project actually emerged out of pure frustration with the starred repository search functionality built in to GitHub. [It is nearly impossible to find projects when the searched term is not exactly in the name or description of the repository.](https://github.com/zekroTJA/starsearch/assets/16734205/9752eb6d-d7df-442c-a315-2bd6b6c5d3bd) To fix this, I had the idea to simply index all starred repositories in a full text search friendly database (in this case [Meilisearch](https://www.meilisearch.com/)) to be able to search through them via a very simple web application.

https://github.com/zekroTJA/starsearch/assets/16734205/e236b40d-46ea-4008-90e9-d79a085c9e30

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