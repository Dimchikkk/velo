This is deployement branch for the project. To keep size of the repo small it is advised to clear history for big blobs (e.g. `velo_bg.wasm`) from time to time:

### Useful commands:

To see the size of the blobs in the repo:
```sh
git rev-list --objects --all \
                                         | git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' \
                                         | awk '/^blob/ {print substr($0,6)}' \
                                         | sort -r --numeric-sort --key=2 \
                                         | gcut --complement --characters=13-40 \
                                         | gnumfmt --field=2 --to=iec-i --suffix=B --padding=7 --round=nearest
```

To remove all blobs bigger than 1M from the history (not including files from HEAD):
```sh
brew install bfg
bfg -b 1M
```

Update repo:
```sh
git push origin --force --all
```