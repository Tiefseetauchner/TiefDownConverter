# Smart Clean {#smart-clean}

Smart clean automatically removes old conversion folders. When enabled via
`smart_clean` in `manifest.toml`, TiefDown keeps only a given number of
recent folders. The number is specified with `smart_clean_threshold` and
defaults to `5`.

During conversion the library checks the amount of existing conversion
folders and deletes the oldest ones once the threshold is exceeded.
