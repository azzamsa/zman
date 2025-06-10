# Changelog

All notable changes to this project will be documented in this file.

## [3.1.3] - 2025-06-10

### 💼 Other

- Upgrade `jiff` from v0.1 to v0.2 ([43fad2c](https://github.com/azzamsa/zman/commit/43fad2c1bbde873c94e53222c0808df85114a2af))

  It is safe to assume all days are 24 hours.
  As this tool just to show the progress bar.

- Upgrade dependencies ([e438b48](https://github.com/azzamsa/zman/commit/e438b48a8eb982c25a348a87b7d970d2f8713121))
- Upgrade MSRV ([05514e0](https://github.com/azzamsa/zman/commit/05514e0660bed0616f2451f745fb4003f557b3b9))

### 📚 Documentation

- Decision regarding the JSON generation ([079da98](https://github.com/azzamsa/zman/commit/079da982d02477a58b23d0c0facbfc47832e527e))

### ⚙️ Miscellaneous Tasks

- Update configs ([4cd7373](https://github.com/azzamsa/zman/commit/4cd73739d3530301cc87aa746ba96ca9175ea241))

## [3.1.2] - 2024-08-07

### 🚜 Refactor

- Group imports ([02e32ae](https://github.com/azzamsa/zman/commit/02e32aee6f0435913c3001142f58182ae79d828d))

## [3.1.1] - 2024-08-07

### 💼 Other

- Put formatter and linter configs in the root directory ([60898be](https://github.com/azzamsa/zman/commit/60898bed4baa189b2050714766f8fdce51976272))

  Otherwise, we would have to deal with a custom `--config-path` for every possible formatter on Earth.
  It's okay to let them clutter the directory (check out other popular repos!).
  Even if they are visible on the GitHub repo, locally they are hidden by your favorite editor.

### 🚜 Refactor

- Use cleaner approach for defining `today` ([cbca1b2](https://github.com/azzamsa/zman/commit/cbca1b2169889defa66d856f1ba882ee0e3e6a41))

### 📚 Documentation

- Outdated CI badge ([f403c07](https://github.com/azzamsa/zman/commit/f403c07e1934d1957236d5011ff7df743ea34491))

## [3.1.0] - 2024-08-06

### 💼 Other

- Adopt `just` new features ([c166b77](https://github.com/azzamsa/zman/commit/c166b7706341e4929325c9938df670cdda9e451e))

  It simplifies my recipes.

- Upgrade dependencies ([5e823fe](https://github.com/azzamsa/zman/commit/5e823feb04c3b29cfa40327e1a3e01383c2e0e32))
- Upgrade dependencies ([4dfee37](https://github.com/azzamsa/zman/commit/4dfee3790fdfaa785382a1504003341c28376d04))
- Put formatter and linter configs in the root directory ([a1c8e66](https://github.com/azzamsa/zman/commit/a1c8e66eb33286706e020bd4b3634c947c9ec60a))

  Otherwise, we would have to deal with a custom `--config-path` for every possible formatter on Earth.
  It's okay to let them clutter the directory (check out other popular repos!).
  Even if they are visible on the GitHub repo, locally they are hidden by your favorite editor.

- Upgrade dependencies ([80ac947](https://github.com/azzamsa/zman/commit/80ac9474e5d10d9eb23163ede520f0ecb93b205e))
- Upgrade MSRV ([cf8b583](https://github.com/azzamsa/zman/commit/cf8b583d1270d1fd6a4fd6ce4c76df4873438191))
- Add new workflow ([d46252d](https://github.com/azzamsa/zman/commit/d46252db95c3c59cdc7094b7eb424ad1e37202f9))
- Upgrade dependencies ([a25c898](https://github.com/azzamsa/zman/commit/a25c898ec091c93c7ab50eb48860d31ad0ac7cdd))

### 🚜 Refactor

- Port chrono to jiff ([cf3b798](https://github.com/azzamsa/zman/commit/cf3b798fcfc9d455853b4328bb09ea824f370cae))

  jiff API is much more cleaner.

- Don't pass too many args to a function ([304afd5](https://github.com/azzamsa/zman/commit/304afd5af6878adc876dadc63adc49c96cfdc60a))

### 📚 Documentation

- Enable github sponsor ([7b5d23c](https://github.com/azzamsa/zman/commit/7b5d23cb7d8390e2882148edb94deac5d88796a6))

### ⚙️ Miscellaneous Tasks

- Update ([94cb107](https://github.com/azzamsa/zman/commit/94cb10791593a2f8be6be0c0c08304cf9962726a))
- Remove unused tool ([9ab919b](https://github.com/azzamsa/zman/commit/9ab919b894641b5c990cde8a8f279cf57de726c6))
- Use regex for os name ([05946eb](https://github.com/azzamsa/zman/commit/05946eb751ce4de2827f189525e1fe8c34078ada))

  I don't want to change it every year

- `APP_NAME` is suitable for both library and binary ([7900b18](https://github.com/azzamsa/zman/commit/7900b18105677698c292c764cd95f2058a1d2372))

## [3.0.0] - 2023-07-22

### 🚀 Features

- Multi threading with `chrono` ([5a94c01](https://github.com/azzamsa/zman/commit/5a94c01b702500d4c269f58dd9fe0afcae1f3a37))

### 💼 Other

- Upgrade dependencies ([12c706e](https://github.com/azzamsa/zman/commit/12c706edcbf3f3e0efe98ca81d412d26bcccf67d))
- Support binstall ([50c7a2f](https://github.com/azzamsa/zman/commit/50c7a2f8624e2b9ae44517417cfd2b9f67b98eb6))

### 🚜 Refactor

- New workflow ([df53b9d](https://github.com/azzamsa/zman/commit/df53b9dd7e8c38dc8782e0bc068b6475000c4737))

### 📚 Documentation

- Update copyrights year. ([b88ef4a](https://github.com/azzamsa/zman/commit/b88ef4a2eb95aa40519d53121afad589518e3643))
- Binstall installation guide ([452104b](https://github.com/azzamsa/zman/commit/452104b9ca725bc95dd4f3337156391c59251b6d))

### ⚙️ Miscellaneous Tasks

- `build` matrix is not needed ([65ec38d](https://github.com/azzamsa/zman/commit/65ec38d5321740b055b7c737b6b609f082989be0))
- Remove debug step ([42e0ab3](https://github.com/azzamsa/zman/commit/42e0ab3087ef6bedf92bdf283923d8f001d69640))

## [2.0.2] - 2023-01-03

### 🧪 Testing

- Fails during the start of the year ([395997c](https://github.com/azzamsa/zman/commit/395997c877372b1eddcbe2bceb2084deb57bcbb8))

### ⚙️ Miscellaneous Tasks

- Better workflow ([5b8c5b0](https://github.com/azzamsa/zman/commit/5b8c5b08adb732b44b876f0ba9944d846c217bab))

## [2.0.1] - 2022-12-28

### 💼 Other

- Remove unused git-cliff arg ([8bdbe4e](https://github.com/azzamsa/zman/commit/8bdbe4ee033148b0061333a72bf51a1b2e96bf05))
- Add git hook ([7a4db9a](https://github.com/azzamsa/zman/commit/7a4db9a6120a1a7f6510ae20e8753e27f575c47b))
- `content` parser is not used here ([3bc84b2](https://github.com/azzamsa/zman/commit/3bc84b2a9a7cca67104c4589fa0c654a2be13353))

### 🚜 Refactor

- Reorder functions ([8661500](https://github.com/azzamsa/zman/commit/8661500ab2e10cf76716a9ebfe9cb575a1a5d28d))
- Remove unused `derive` traits ([af795c7](https://github.com/azzamsa/zman/commit/af795c73c9c9af51d4511ee725d416f036ff349f))

### 📚 Documentation

- Refactor ([40b62ca](https://github.com/azzamsa/zman/commit/40b62cac70020798a2aa07bb4b6dd9ecd851ca9d))
- Add commit message format ([9c1263f](https://github.com/azzamsa/zman/commit/9c1263f00917cd71c1644f5fb8b47b1b5aedef2e))
- Update guide ([1bb1639](https://github.com/azzamsa/zman/commit/1bb16397242cc27756ff90c5fa7c65083112a401))

### ⚡ Performance

- Migrate to `owo-color` ([c88a60a](https://github.com/azzamsa/zman/commit/c88a60aea3d5cd09c77656a02b821a08c3ce86b5))

  It has fewer dependencies

### 🧪 Testing

- Fails during the end of the month ([f910ade](https://github.com/azzamsa/zman/commit/f910ade3077a131b372021a2a3c6519371198fee))
- Don't hardcode crate name ([2b83dec](https://github.com/azzamsa/zman/commit/2b83decd51e6c88c2c26ebd695318a39cf9b4bca))

### ⚙️ Miscellaneous Tasks

- Simplify release workflow ([095825e](https://github.com/azzamsa/zman/commit/095825ea4d850d5f21f46e10777c31fcbc8b075d))
- `dprint` not found ([f08223a](https://github.com/azzamsa/zman/commit/f08223a63d1ad6800350d1e658a6f85432c99b7c))

## [2.0.0] - 2022-12-20

### 🐛 Bug Fixes

- `period` is more self-describable ([2430dde](https://github.com/azzamsa/zman/commit/2430dde16f3697963ed5692a5ad80479148d61b3))

## [1.0.7] - 2022-12-20

### 💼 Other

- Bump rust edition ([4975600](https://github.com/azzamsa/zman/commit/49756001a9cd210ccf07529fdf9403a82f400a1a))
- Update release step ([a16d7be](https://github.com/azzamsa/zman/commit/a16d7beb9a6095ad52f1c79bae40954ba50130e2))
- Migrate from `make` to `just` ([4b5f491](https://github.com/azzamsa/zman/commit/4b5f49136a6a7e6927cd692f77724d904967f510))

### 🚜 Refactor

- Remove any `unrwap` ([bf09e1b](https://github.com/azzamsa/zman/commit/bf09e1b71dbe7453989308a54d265fd6c6170f5e))
- Use `thiserror` internally ([5bfdba1](https://github.com/azzamsa/zman/commit/5bfdba1885e27ac6adc93bacdd680bfd1e25f0e2))
- Remove unused `clippy` rules ([4154042](https://github.com/azzamsa/zman/commit/4154042f6f97418164d817bc8dc45578675f0e0f))
- Remove any `unwrap` ([104f853](https://github.com/azzamsa/zman/commit/104f8535b8fb789616087d958464b62078f62680))
- Migrate to `time` from `chrono` ([b3dad64](https://github.com/azzamsa/zman/commit/b3dad64218e7ab3b17789d00cefa03043c377172))

  Its API looks easier to use.

- Migrate to `clap derive` ([974f5a7](https://github.com/azzamsa/zman/commit/974f5a756430ce6a1d6fc7240dfef3c58f07b4b7))
- Introduce `dprint` ([3d8c4e0](https://github.com/azzamsa/zman/commit/3d8c4e0295b3f3c9a8af24b42df517b27ddfc98d))

### ⚙️ Miscellaneous Tasks

- Use cross for all the release build ([a7aabe3](https://github.com/azzamsa/zman/commit/a7aabe3b35866b58766a349afe295b3b460a766d))
- Update release task ([35f7e4e](https://github.com/azzamsa/zman/commit/35f7e4e7409509350143d14a54fe1ee6cc15b0fd))
- Bump `windows` version ([52072d4](https://github.com/azzamsa/zman/commit/52072d430ac94b42a984544c5c44f034610e15e2))
- Use `taiki-e/install-action` for `just` ([e60a22d](https://github.com/azzamsa/zman/commit/e60a22db4c81b317d903e87643aefacd38279c69))
- Update ([3ef11ec](https://github.com/azzamsa/zman/commit/3ef11eceaed3017c14ae1693a0418483b5698002))

## [1.0.6] - 2022-05-18

### 🚜 Refactor

- Update dependencies ([929d342](https://github.com/azzamsa/zman/commit/929d3425a389ba9358ce8952f1ed494fa0b529a2))

### ⚙️ Miscellaneous Tasks

- Disable `fail-fast` on release ([31ecc91](https://github.com/azzamsa/zman/commit/31ecc91af9ba5901711cc1bd19b6211df0c1db6c))
- Fix sed expression ([15b6548](https://github.com/azzamsa/zman/commit/15b6548fd503c011e7e04032058f797f224693d3))
- Don't skip tag in git cliff ([6348b1c](https://github.com/azzamsa/zman/commit/6348b1cc7dde10b1784d8f97180683b3e21fa6fa))

## [1.0.5] - 2022-05-18

### 📚 Documentation

- Update the release checklist ([21f5238](https://github.com/azzamsa/zman/commit/21f52388cf8c1fcbb647fcf6a5b8b0860e8c239c))
- Update old documentation ([1bacc80](https://github.com/azzamsa/zman/commit/1bacc8069c4e60ca127407d971a20a6c0d15d520))

### ⚙️ Miscellaneous Tasks

- Fix missing step in makefile ([6a827a1](https://github.com/azzamsa/zman/commit/6a827a1be00456a2609d0b89d1a0535062927588))
- Add release scripts ([68c8a56](https://github.com/azzamsa/zman/commit/68c8a56a87b168ea8ec15d8cc814ca82973a002e))
- Add makefile ([8d0eb55](https://github.com/azzamsa/zman/commit/8d0eb55437d14dfec5a74ee48fc44915e4549a41))
- Update CI workflows ([3c27d55](https://github.com/azzamsa/zman/commit/3c27d55c0366283a245eedf242a14680a85a5a72))
- Remove SourceHut config ([45a1d5b](https://github.com/azzamsa/zman/commit/45a1d5ba7dbcc5fd641be9c4cd3856782ab74207))

## [1.0.4] - 2021-11-14

### 🚜 Refactor

- Port to clap v2 ([4ac0794](https://github.com/azzamsa/zman/commit/4ac07944893403d4a417077d49d77971235b3de8))

  The v3 API is always changing.

## [1.0.2] - 2021-11-14

### 🚜 Refactor

- Clippy ([a4b6f44](https://github.com/azzamsa/zman/commit/a4b6f44ea3edf2986bc0ee2e9b91305d7012f9e1))
- Hide private functions ([4044710](https://github.com/azzamsa/zman/commit/40447102713f6c75cbe1db69bfc968a60a8cdbd5))
- Don't use glob import for `chrono` ([dfa857d](https://github.com/azzamsa/zman/commit/dfa857d525b9f7f832b76cd7c7e408d15cbb2e2c))

  Just for readability.

### 📚 Documentation

- Update contributing ([f6b4416](https://github.com/azzamsa/zman/commit/f6b4416d7c57c1e23a6ada0249cdf37896a29d3e))
- Update contributions steps ([2888ba7](https://github.com/azzamsa/zman/commit/2888ba73c60ac0547e5bf091a5235b06943183ca))
- It's i3status-rust ([aea4c0b](https://github.com/azzamsa/zman/commit/aea4c0bf04dcfae6b0ffec54c090efab366f4839))

  Not i3status-rs.

- Don't show `help` in demo ([551cea5](https://github.com/azzamsa/zman/commit/551cea5a3ca652251a9155fe8d5a2d507cb17042))

  The `--help` contains the app version.
  While the API stays the same, the version number makes users think
  the repo was not updated correctly.

- Update release script path ([bcdf80d](https://github.com/azzamsa/zman/commit/bcdf80d3fa89543009c6293a3c5671b83ed2eb33))

### ⚙️ Miscellaneous Tasks

- Migrate to GitHub ([c9a5c52](https://github.com/azzamsa/zman/commit/c9a5c5220090b9d346dddfd7389bce43d43cc740))

## [1.0.0] - 2021-04-20

### 🚀 Features

- Ability to set app icon ([890c0fa](https://github.com/azzamsa/zman/commit/890c0fa3bcdf0e4e80f68b79a59f0ce53e107907))
- Show a char to distinguish a time in JSON mode ([9c3c428](https://github.com/azzamsa/zman/commit/9c3c428a29b76cf30730495c4fcf8916ccb7895d))

  If the status bar supports cycling command, it will be hard to
  distinguish a difference between year, month, and week.
  A char prefix such "y", "m", "w" would be helpful.

### 🐛 Bug Fixes

- Wrong month and week ratio computation ([e1f79b7](https://github.com/azzamsa/zman/commit/e1f79b7ae3d29a0772daae001edcb67f84d756f8))
- I3status.rs can't accept unicode icon ([21a0af2](https://github.com/azzamsa/zman/commit/21a0af2dbe9fa7290300fffda0d8eccc677fba36))

### 🚜 Refactor

- Duplicate code ([ebb9935](https://github.com/azzamsa/zman/commit/ebb9935dc801bcced6abbe61d58b23b62696ec03))
- Rename function name for readability ([2057427](https://github.com/azzamsa/zman/commit/205742738d27001fb7829ac03d3e6358b60bfbf6))

  rename `get` to `compute`

### 📚 Documentation

- Master branch is not protected ([83c6ee3](https://github.com/azzamsa/zman/commit/83c6ee3577e31d0f217ecb2814be8084cba8e818))

  We host zman on sr.ht, so the master branch is not protected

- Improve docs ([a2447c7](https://github.com/azzamsa/zman/commit/a2447c73ca2d4ec8ccccc3a90fd8e56bb916f655))
- Zman now support time char prefix and icon ([5d945e3](https://github.com/azzamsa/zman/commit/5d945e323a81ea7689bae099b60584aeba2c4f85))
- I3status.rs example configs ([6d35fe0](https://github.com/azzamsa/zman/commit/6d35fe090bed0f3654130f3666c97d05b507282a))

### 🎨 Styling

- Change app icon ([36e1254](https://github.com/azzamsa/zman/commit/36e125440237525684cace7a4be2032414f434a4))

### 🧪 Testing

- Add more unit tests ([353a1e9](https://github.com/azzamsa/zman/commit/353a1e9222fa9acb7077b4ed0ba8f0e5e2c44fca))
- Add unit tests ([d78c651](https://github.com/azzamsa/zman/commit/d78c651bf92a4e2207f1da6aace3c3d707b7a88f))
- More integration tests ([0847f45](https://github.com/azzamsa/zman/commit/0847f457f5d33d3bc0848e2a91fab10fc272434c))

## [0.1.3] - 2021-04-19

### 🐛 Bug Fixes

- Don't colorize JSON output ([bf5f186](https://github.com/azzamsa/zman/commit/bf5f1868d3d8d958e2f60353eaee5e292ad0046c))

  It's useless.

- Hide possible values ([ef1ccd4](https://github.com/azzamsa/zman/commit/ef1ccd4c1407f87e0dbeeb51e7389fe5be96ccb5))

  Add too many info in terminal.

- Use time as an argument instead of an option ([9e8d683](https://github.com/azzamsa/zman/commit/9e8d68323d073bb113c9c7bc84d00c11ffe35002))

  Using time as an argument is somewhat wrong

### 💼 Other

- Restructure directory ([e788574](https://github.com/azzamsa/zman/commit/e7885743cd0a4de2162797f56ad989df2c7f15b6))

### 🚜 Refactor

- Make clippy happy ([3740d56](https://github.com/azzamsa/zman/commit/3740d561d6ca27ba44d1b4309d179b6bf4450e9a))
- Use printer module ([78cbe90](https://github.com/azzamsa/zman/commit/78cbe9092da46f5f1dfc27cd7dc2b01979d7b848))

### 📚 Documentation

- Add window dimension to record demo ([089c2d3](https://github.com/azzamsa/zman/commit/089c2d3bb5c9dd7cd08591af34693390cc3f6f9f))

## [0.1.2] - 2020-09-29

### 🐛 Bug Fixes

- Wrong name in argument doc ([a80f8f9](https://github.com/azzamsa/zman/commit/a80f8f96067681d15b18a7e77dba9befa6138a36))

### 💼 Other

- Bump version to v0.1.2 ([bb62899](https://github.com/azzamsa/zman/commit/bb62899b91cc744d04957632ba42c1ec7d4175d8))
- Add warning to update related release file ([d4b2300](https://github.com/azzamsa/zman/commit/d4b23005a8a4aea6ddc3e186c409f9331b836ee7))

### 📚 Documentation

- Update cotribution how-to ([df9609a](https://github.com/azzamsa/zman/commit/df9609a7421794f3b18a51156408761901c6dba5))
- Add binaries info ([984be74](https://github.com/azzamsa/zman/commit/984be741be2395b46b311774bbf08b5abc4ee1b5))
- Fix wrong link in the docs ([3b8a8e1](https://github.com/azzamsa/zman/commit/3b8a8e1c4a0c12beec01b20e02a85177299b4c0d))

## [0.1.1] - 2020-09-28

### 🚀 Features

- Support full bar and rest bar config ([ab4ebfb](https://github.com/azzamsa/zman/commit/ab4ebfbd79439c6d73eec5dcfa0d86ab31aa9614))
- Add initial support for `config` ([ab0db5f](https://github.com/azzamsa/zman/commit/ab0db5f8954b9f97d549c4915385e178919b6023))

### 💼 Other

- Bump version to v0.1.1 ([65ae00b](https://github.com/azzamsa/zman/commit/65ae00b1e3e51ec7a0cbbf5fa8f4bb598acdaff6))

### 📚 Documentation

- Add doc regarding demo ([9caafb3](https://github.com/azzamsa/zman/commit/9caafb35a7c9c6fd83c185add000ef2443fa88e0))
- Add default value in argument doc ([5fb2b9a](https://github.com/azzamsa/zman/commit/5fb2b9aa8849e26ccc255d5f85aa82bf2912bfd1))
- Host demo in its own repo ([74df238](https://github.com/azzamsa/zman/commit/74df238c2d4354557fd71e41cd1af89cded5c9ae))
- Add inspiration section ([9c4c42b](https://github.com/azzamsa/zman/commit/9c4c42b61153f90d661ef2d2d856261c24d3da04))

## [0.1.0] - 2020-09-28

### 🚀 Features

- Support colored output ([7cdcf30](https://github.com/azzamsa/zman/commit/7cdcf300304b9304ad631e3fe2d52aed245008f0))
- Support JSON ouput ([1dc35a6](https://github.com/azzamsa/zman/commit/1dc35a669b32659ff0cecd5c9c889b235b6781f5))
- Support week progress bar ([1ac0a8f](https://github.com/azzamsa/zman/commit/1ac0a8f23921afe15580025bcc545303d4d84d62))
- Support month progress bar ([27e801e](https://github.com/azzamsa/zman/commit/27e801e4506b27070a48a51985f3b8fbf212a82d))
- Add CLI support ([c978454](https://github.com/azzamsa/zman/commit/c978454e8521026c008af0b656895d96714319dc))

### 💼 Other

- Bump version to v0.1.0 ([64b1fef](https://github.com/azzamsa/zman/commit/64b1fef65bf2f759db5c67768bd3266a8bff1edf))
- Add build scripts ([f11e5e7](https://github.com/azzamsa/zman/commit/f11e5e7d35d52122325bf66a3760b5a9ff67f6b1))

### 🚜 Refactor

- Remove unused reference implementation ([f812495](https://github.com/azzamsa/zman/commit/f81249558ffa51c4eb30fa5b7839988a1cd815fe))

### 📚 Documentation

- Add LICENSE ([972986e](https://github.com/azzamsa/zman/commit/972986e5b688457a0359dc41ec3c7f11cc8bbd5a))
- Add initial docs ([f0faceb](https://github.com/azzamsa/zman/commit/f0faceb6f2ed6e1170c4091959fe6fb9d4ce7ce3))
