# starsearch

A very simple web application to better search your starred GitHub repositories and find stuff quick!

This project actually emerged out of pure frustration with the starred repository search functionality built in to GitHub. It is nearly impossible to find projects when the searched term is not exactly in the name or description of the repository. To fix this, I had the idea to simply index all starred repositories in a full text search friendly database (in this case [Meilisearch](https://www.meilisearch.com/)) to be able to search through them via a very simple webb application.