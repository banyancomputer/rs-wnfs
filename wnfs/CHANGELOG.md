# Changelog

## [0.1.21](https://github.com/banyancomputer/rs-wnfs/compare/wnfs-v0.1.21...wnfs-v0.1.21) (2023-11-20)


### ⚠ BREAKING CHANGES

* get_node should return null on missing path ([#253](https://github.com/banyancomputer/rs-wnfs/issues/253))
* **exports:** make re-exports more flexible ([#167](https://github.com/banyancomputer/rs-wnfs/issues/167))

### Features

* `open_file_mut` function for getting `&mut PrivateFile` references ([#218](https://github.com/banyancomputer/rs-wnfs/issues/218)) ([f80dbb1](https://github.com/banyancomputer/rs-wnfs/commit/f80dbb19cee471447145245b8c0285608a25ebcc))
* Add `PrivateDirectory::entires`, `PrivateFile::read_at` and make `PrivateFile::get_content_size_upper_bound` public ([#237](https://github.com/banyancomputer/rs-wnfs/issues/237)) ([1572f43](https://github.com/banyancomputer/rs-wnfs/commit/1572f432b6ae5366436cdefda7defd71c23b0ca7))
* added prints ([c4098cc](https://github.com/banyancomputer/rs-wnfs/commit/c4098ccd8051e015b9ceff5ce21531d4a52dfff2))
* **api:** add privateforest merge and diff bindings ([#181](https://github.com/banyancomputer/rs-wnfs/issues/181)) ([231ece4](https://github.com/banyancomputer/rs-wnfs/commit/231ece4309cab86d4682693e8e31f8ed99478a1f))
* **api:** adds missing metadata functions for the private side ([#144](https://github.com/banyancomputer/rs-wnfs/issues/144)) ([7588f69](https://github.com/banyancomputer/rs-wnfs/commit/7588f69440bfec14b8959f6aecd35eb5f848dacc))
* **api:** adds missing metadata functions for the private side ([#146](https://github.com/banyancomputer/rs-wnfs/issues/146)) ([88e9f19](https://github.com/banyancomputer/rs-wnfs/commit/88e9f19a69fbbb99e3ee78c831eeb520a33f0b46))
* **api:** self lookup & store at construction ([#138](https://github.com/banyancomputer/rs-wnfs/issues/138)) ([228d326](https://github.com/banyancomputer/rs-wnfs/commit/228d326291926c7e4b593ef66ebb089ce220dacb))
* final version ([0fd84bf](https://github.com/banyancomputer/rs-wnfs/commit/0fd84bfd306cd4cd6c2834cce3ae25ebeeb7b7f7))
* get_cids method implemented ([1fa1666](https://github.com/banyancomputer/rs-wnfs/commit/1fa16665aa130c783f97d15ffa2e043e02a7e58a))
* **hamt:** diff and merge implementation ([#94](https://github.com/banyancomputer/rs-wnfs/issues/94)) ([883b3ab](https://github.com/banyancomputer/rs-wnfs/commit/883b3ab7f9c0ec4c086e83afe7f0510c448f6bbb))
* implemented SnapshotKey decryption on PrivateFiles ([4795d01](https://github.com/banyancomputer/rs-wnfs/commit/4795d019be60b6cc76fc498a34c69b349b73ab97))
* Make log optional ([#189](https://github.com/banyancomputer/rs-wnfs/issues/189)) ([12cd842](https://github.com/banyancomputer/rs-wnfs/commit/12cd8428514d7c145b443a78e279dc468fa01a91))
* private backpointer ([#90](https://github.com/banyancomputer/rs-wnfs/issues/90)) ([e38d039](https://github.com/banyancomputer/rs-wnfs/commit/e38d039d3886f8590e00c7f87a530ca207f8a713))
* PrivateLink abstraction ([#172](https://github.com/banyancomputer/rs-wnfs/issues/172)) ([f04fa89](https://github.com/banyancomputer/rs-wnfs/commit/f04fa89738e19a095d177e18b35d7e153c380833))
* **private:** shared private data ([#148](https://github.com/banyancomputer/rs-wnfs/issues/148)) ([c210067](https://github.com/banyancomputer/rs-wnfs/commit/c2100679acb1d16d98cb9a2e6aa6e9abc5a8eff2))
* Redundant sha2 ([#191](https://github.com/banyancomputer/rs-wnfs/issues/191)) ([231f4e9](https://github.com/banyancomputer/rs-wnfs/commit/231f4e929378d7a02c9f7f8b095f1c2b1175ec2e))
* reimplmented temporary symlink & double packing solutions ([ffc7863](https://github.com/banyancomputer/rs-wnfs/commit/ffc78631bbf57a2d2effe8316969631eeafef5db))
* Remove `base_history_on` and auto-track history instead ([#174](https://github.com/banyancomputer/rs-wnfs/issues/174)) ([806bbb9](https://github.com/banyancomputer/rs-wnfs/commit/806bbb93b1f03983165375005e14a9b63ebe67c2))
* Streaming write for PrivateFile ([#163](https://github.com/banyancomputer/rs-wnfs/issues/163)) ([1bfe89b](https://github.com/banyancomputer/rs-wnfs/commit/1bfe89bcaabdf679a5338a2c9aa97b76deb00b03))
* working on Directory reconstruction ([9c8eea8](https://github.com/banyancomputer/rs-wnfs/commit/9c8eea8901ec4f6714bfa2ffb6a3463917c19d0f))


### Bug Fixes

* `find_latest_share_counter` finds the last share count ([#197](https://github.com/banyancomputer/rs-wnfs/issues/197)) ([69ffeec](https://github.com/banyancomputer/rs-wnfs/commit/69ffeeca20cc3106e6d733e2d5adf5f87987630c))
* checks ([354cda1](https://github.com/banyancomputer/rs-wnfs/commit/354cda12d6100429bf8c676d4e93d703c69c0db7))
* docs tests ([66be8fc](https://github.com/banyancomputer/rs-wnfs/commit/66be8fc0c9fb495f78b195c4cbc8a1f3d198515e))
* expose metdata ([b6955f2](https://github.com/banyancomputer/rs-wnfs/commit/b6955f284343d418e6ae229209a2d07760c50bd7))
* get_node should return null on missing path ([#253](https://github.com/banyancomputer/rs-wnfs/issues/253)) ([5ed87fe](https://github.com/banyancomputer/rs-wnfs/commit/5ed87fe6359a19abdea5f34dd0537fd5d62c98a8))
* kept separate loading functions but unified store ([fa536d8](https://github.com/banyancomputer/rs-wnfs/commit/fa536d8fab98c8c87c6a171d4b3ffa717c996710))
* more cleanup ([fa33a8f](https://github.com/banyancomputer/rs-wnfs/commit/fa33a8fdadc7437380fb2cc52c0f95fec15bd60c))
* open_file_mut required mut BlockStore, no longer ([9464836](https://github.com/banyancomputer/rs-wnfs/commit/9464836cd6c67db9316667818a73ab4af89337f4))
* propagate missing chunk error ([#252](https://github.com/banyancomputer/rs-wnfs/issues/252)) ([4c16dcd](https://github.com/banyancomputer/rs-wnfs/commit/4c16dcd4725c8b499a01184530e0e95ed8f4a9d5))
* re-disabled Dir snapshot decryption ([829629a](https://github.com/banyancomputer/rs-wnfs/commit/829629a25d9a7b4f255b4a0f00618e0e16ccbe2d))
* removing old prints ([9159c2d](https://github.com/banyancomputer/rs-wnfs/commit/9159c2d27e98a9230a6f9e329c72ddb5cb64ddf8))
* reverted silly changes ([2385c5c](https://github.com/banyancomputer/rs-wnfs/commit/2385c5c2e3b5a63637f1d0404340ddf25d9d77bf))
* snapshot blocks are encrypted now ([b895c03](https://github.com/banyancomputer/rs-wnfs/commit/b895c037aec99253503b0c2f207e48acfed50da7))
* wasm test break undo ([1243411](https://github.com/banyancomputer/rs-wnfs/commit/124341195c90e7e5adbefd899886e9c263670a20))


### Miscellaneous Chores

* **exports:** make re-exports more flexible ([#167](https://github.com/banyancomputer/rs-wnfs/issues/167)) ([d7870bc](https://github.com/banyancomputer/rs-wnfs/commit/d7870bc78660458fe9c5252c551a474dcdd045f2))
* release 0.1.10 ([#114](https://github.com/banyancomputer/rs-wnfs/issues/114)) ([9cbc320](https://github.com/banyancomputer/rs-wnfs/commit/9cbc32076d80a5b7d3138ea891180c689411123f))
* release 0.1.16 ([#178](https://github.com/banyancomputer/rs-wnfs/issues/178)) ([89e4d36](https://github.com/banyancomputer/rs-wnfs/commit/89e4d36dc9b27ec1ab67db6fc214670efe768f32))
* release 0.1.19 ([1f37ec4](https://github.com/banyancomputer/rs-wnfs/commit/1f37ec4d706b9bcb4305128451cc77063b4f211d))
* release 0.1.21 ([#255](https://github.com/banyancomputer/rs-wnfs/issues/255)) ([2be9f49](https://github.com/banyancomputer/rs-wnfs/commit/2be9f4999d279acccfcda3b690d69dcbcdf8e60b))
* rename to wnfs-wasm and actions fix *maybe* ([#116](https://github.com/banyancomputer/rs-wnfs/issues/116)) ([9ffad56](https://github.com/banyancomputer/rs-wnfs/commit/9ffad56e6ab402c8636b13563a5bf516fb962037))

## [0.1.21](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.20...wnfs-v0.1.21) (2023-05-22)


### ⚠ BREAKING CHANGES

* get_node should return null on missing path ([#253](https://github.com/wnfs-wg/rs-wnfs/issues/253))

### Features

* Add `PrivateDirectory::entires`, `PrivateFile::read_at` and make `PrivateFile::get_content_size_upper_bound` public ([#237](https://github.com/wnfs-wg/rs-wnfs/issues/237)) ([1572f43](https://github.com/wnfs-wg/rs-wnfs/commit/1572f432b6ae5366436cdefda7defd71c23b0ca7))


### Bug Fixes

* get_node should return null on missing path ([#253](https://github.com/wnfs-wg/rs-wnfs/issues/253)) ([5ed87fe](https://github.com/wnfs-wg/rs-wnfs/commit/5ed87fe6359a19abdea5f34dd0537fd5d62c98a8))
* propagate missing chunk error ([#252](https://github.com/wnfs-wg/rs-wnfs/issues/252)) ([4c16dcd](https://github.com/wnfs-wg/rs-wnfs/commit/4c16dcd4725c8b499a01184530e0e95ed8f4a9d5))


### Miscellaneous Chores

* release 0.1.21 ([#255](https://github.com/wnfs-wg/rs-wnfs/issues/255)) ([2be9f49](https://github.com/wnfs-wg/rs-wnfs/commit/2be9f4999d279acccfcda3b690d69dcbcdf8e60b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.20 to 0.1.21
    * wnfs-hamt bumped from 0.1.20 to 0.1.21
    * wnfs-namefilter bumped from 0.1.20 to 0.1.21

## [0.1.20](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.19...wnfs-v0.1.20) (2023-03-30)


### Features

* `open_file_mut` function for getting `&mut PrivateFile` references ([#218](https://github.com/wnfs-wg/rs-wnfs/issues/218)) ([f80dbb1](https://github.com/wnfs-wg/rs-wnfs/commit/f80dbb19cee471447145245b8c0285608a25ebcc))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.19 to 0.1.20
    * wnfs-hamt bumped from 0.1.19 to 0.1.20
    * wnfs-namefilter bumped from 0.1.19 to 0.1.20

## [0.1.19](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.18...wnfs-v0.1.19) (2023-03-23)


### Miscellaneous Chores

* release 0.1.19 ([1f37ec4](https://github.com/wnfs-wg/rs-wnfs/commit/1f37ec4d706b9bcb4305128451cc77063b4f211d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.18 to 0.1.19
    * wnfs-hamt bumped from 0.1.18 to 0.1.19
    * wnfs-namefilter bumped from 0.1.18 to 0.1.19

## [0.1.18](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.17...wnfs-v0.1.18) (2023-03-23)


### Features

* Make log optional ([#189](https://github.com/wnfs-wg/rs-wnfs/issues/189)) ([12cd842](https://github.com/wnfs-wg/rs-wnfs/commit/12cd8428514d7c145b443a78e279dc468fa01a91))
* Redundant sha2 ([#191](https://github.com/wnfs-wg/rs-wnfs/issues/191)) ([231f4e9](https://github.com/wnfs-wg/rs-wnfs/commit/231f4e929378d7a02c9f7f8b095f1c2b1175ec2e))


### Bug Fixes

* `find_latest_share_counter` finds the last share count ([#197](https://github.com/wnfs-wg/rs-wnfs/issues/197)) ([69ffeec](https://github.com/wnfs-wg/rs-wnfs/commit/69ffeeca20cc3106e6d733e2d5adf5f87987630c))

## [0.1.17](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.16...wnfs-v0.1.17) (2023-02-24)


### Features

* **api:** add privateforest merge and diff bindings ([#181](https://github.com/wnfs-wg/rs-wnfs/issues/181)) ([231ece4](https://github.com/wnfs-wg/rs-wnfs/commit/231ece4309cab86d4682693e8e31f8ed99478a1f))
* PrivateLink abstraction ([#172](https://github.com/wnfs-wg/rs-wnfs/issues/172)) ([f04fa89](https://github.com/wnfs-wg/rs-wnfs/commit/f04fa89738e19a095d177e18b35d7e153c380833))
* Remove `base_history_on` and auto-track history instead ([#174](https://github.com/wnfs-wg/rs-wnfs/issues/174)) ([806bbb9](https://github.com/wnfs-wg/rs-wnfs/commit/806bbb93b1f03983165375005e14a9b63ebe67c2))

## [0.1.16](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.15...wnfs-v0.1.16) (2023-02-22)


### ⚠ BREAKING CHANGES

* **exports:** make re-exports more flexible ([#167](https://github.com/wnfs-wg/rs-wnfs/issues/167))

### Features

* Streaming write for PrivateFile ([#163](https://github.com/wnfs-wg/rs-wnfs/issues/163)) ([1bfe89b](https://github.com/wnfs-wg/rs-wnfs/commit/1bfe89bcaabdf679a5338a2c9aa97b76deb00b03))


### Miscellaneous Chores

* **exports:** make re-exports more flexible ([#167](https://github.com/wnfs-wg/rs-wnfs/issues/167)) ([d7870bc](https://github.com/wnfs-wg/rs-wnfs/commit/d7870bc78660458fe9c5252c551a474dcdd045f2))
* release 0.1.16 ([#178](https://github.com/wnfs-wg/rs-wnfs/issues/178)) ([89e4d36](https://github.com/wnfs-wg/rs-wnfs/commit/89e4d36dc9b27ec1ab67db6fc214670efe768f32))

## [0.1.15](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.14...wnfs-v0.1.15) (2023-02-16)


### Features

* **private:** shared private data ([#148](https://github.com/wnfs-wg/rs-wnfs/issues/148)) ([c210067](https://github.com/wnfs-wg/rs-wnfs/commit/c2100679acb1d16d98cb9a2e6aa6e9abc5a8eff2))

## [0.1.14](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.13...wnfs-v0.1.14) (2023-01-16)


### Features

* **api:** adds missing metadata functions for the private side ([#144](https://github.com/wnfs-wg/rs-wnfs/issues/144)) ([7588f69](https://github.com/wnfs-wg/rs-wnfs/commit/7588f69440bfec14b8959f6aecd35eb5f848dacc))

## [0.1.13](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.12...wnfs-v0.1.13) (2023-01-12)


### Features

* **api:** self lookup & store at construction ([#138](https://github.com/wnfs-wg/rs-wnfs/issues/138)) ([228d326](https://github.com/wnfs-wg/rs-wnfs/commit/228d326291926c7e4b593ef66ebb089ce220dacb))

## [0.1.12](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.11...wnfs-v0.1.12) (2023-01-11)


### Features

* private backpointer ([#90](https://github.com/wnfs-wg/rs-wnfs/issues/90)) ([e38d039](https://github.com/wnfs-wg/rs-wnfs/commit/e38d039d3886f8590e00c7f87a530ca207f8a713))

## [0.1.11](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.10...wnfs-v0.1.11) (2023-01-06)


### Features

* **hamt:** diff and merge implementation ([#94](https://github.com/wnfs-wg/rs-wnfs/issues/94)) ([883b3ab](https://github.com/wnfs-wg/rs-wnfs/commit/883b3ab7f9c0ec4c086e83afe7f0510c448f6bbb))

## [0.1.10](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.9...wnfs-v0.1.10) (2022-12-06)


### Miscellaneous Chores

* release 0.1.10 ([#114](https://github.com/wnfs-wg/rs-wnfs/issues/114)) ([9cbc320](https://github.com/wnfs-wg/rs-wnfs/commit/9cbc32076d80a5b7d3138ea891180c689411123f))
* rename to wnfs-wasm and actions fix *maybe* ([#116](https://github.com/wnfs-wg/rs-wnfs/issues/116)) ([9ffad56](https://github.com/wnfs-wg/rs-wnfs/commit/9ffad56e6ab402c8636b13563a5bf516fb962037))

## [0.1.10](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.9...wnfs-v0.1.10) (2022-12-06)


### Miscellaneous Chores

* release 0.1.10 ([#114](https://github.com/wnfs-wg/rs-wnfs/issues/114)) ([9cbc320](https://github.com/wnfs-wg/rs-wnfs/commit/9cbc32076d80a5b7d3138ea891180c689411123f))
