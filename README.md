# sora2-common
## Release process
- Create branch `release/X.Y.Z` from `develop`, use [semver](https://semver.org/)
- `cargo update`
- Merge `master` into `release/X.Y.Z` branch
- Merge `release/X.Y.Z` into `master` and `develop`
- Create Github Release on `master` branch, tag is `X.Y.Z`
