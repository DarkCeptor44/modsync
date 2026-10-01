# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.2.0](https://github.com/DarkCeptor44/modsync/compare/9e0871ae930eb6d9b8ad8303141370cdfc0d1230..v0.2.0) - 2026-10-01
#### Performance
- (**backend**) use buffer_unordered instead of unbounded semaphore for sync logic - ([5090f34](https://github.com/DarkCeptor44/modsync/commit/5090f34b9bff7832eca2d6af95d92e9e447ae4d1)) - DarkCeptor44
#### Refactors
- (**backend**) add glob support for exclusions - ([9e0871a](https://github.com/DarkCeptor44/modsync/commit/9e0871ae930eb6d9b8ad8303141370cdfc0d1230)) - DarkCeptor44

- - -

## [v0.1.0](https://github.com/DarkCeptor44/modsync/compare/3f8fa7742d79df79b59c226978a149cee7da6b37..v0.1.0) - 2026-09-30
#### Features
- (**frontend**) add toast component - ([bca1d55](https://github.com/DarkCeptor44/modsync/commit/bca1d55e03bf1dc71d5fc05364f5391cd53e4312)) - DarkCeptor44
- initial commit - ([3f8fa77](https://github.com/DarkCeptor44/modsync/commit/3f8fa7742d79df79b59c226978a149cee7da6b37)) - DarkCeptor44
#### Bug Fixes
- (**backend**) save config file after removing a profile - ([92bdf76](https://github.com/DarkCeptor44/modsync/commit/92bdf76ea42460b21bbd3f06890a3416934c49c8)) - DarkCeptor44
- (**backend**) use dry_run from frontend in sync instead of hardcoded true - ([0e63d45](https://github.com/DarkCeptor44/modsync/commit/0e63d458b23eca5cb01f82a6ea34145be8c68101)) - DarkCeptor44
- (**backend**) require Send in calculate_quick/full_hash functions to allow spawning a task with them - ([e961632](https://github.com/DarkCeptor44/modsync/commit/e9616320eba170d796bc8283626cb2474de3fa0f)) - DarkCeptor44
- (**frontend**) sanitize input from InputField component so it's not always string - ([aefb582](https://github.com/DarkCeptor44/modsync/commit/aefb582198562813cb7fb778f4ae3bd5e225183d)) - DarkCeptor44
- (**frontend**) button hover coloring wasnt working due to dynamic classes - ([6bf5e61](https://github.com/DarkCeptor44/modsync/commit/6bf5e6161478af91f34369116ffca989fecd3d94)) - DarkCeptor44
#### Performance
- (**backend**) erase types in calculate_full/quick_hash to reduce binary bloat - ([8263cf1](https://github.com/DarkCeptor44/modsync/commit/8263cf1222ea794fa194ef7897b98376b8fe78ce)) - DarkCeptor44
- return id from the backend add_profile so frontend doesnt have to call fetch again - ([e71e941](https://github.com/DarkCeptor44/modsync/commit/e71e941ff78d8f395d8e065aaa6fb9ca416d0be3)) - DarkCeptor44
#### Refactors
- (**backend**) add final necessary functions for the syncing - ([8b72495](https://github.com/DarkCeptor44/modsync/commit/8b724953d1636a1eac205c04e6b63bb94caa8d50)) - DarkCeptor44
- (**backend**) add handle_removal function to properly get rid of unsynced files - ([5422820](https://github.com/DarkCeptor44/modsync/commit/5422820233584d3f098170ee32887ef1e3afa552)) - DarkCeptor44
- (**backend**) rename collect_source_files to collect_files to be general - ([b82c3e3](https://github.com/DarkCeptor44/modsync/commit/b82c3e35d7bf5bc0e22a67bf80d2c1ef1d0e22b1)) - DarkCeptor44
- (**backend**) make sync_file return SyncOutcome instead of a Result, error is in SyncOutcome - ([f3c5824](https://github.com/DarkCeptor44/modsync/commit/f3c5824b98c8d3f3d34ceb646bc493180e36fe89)) - DarkCeptor44
- (**backend**) add error variant to SyncAction; get rid of utils - ([a2dc3f9](https://github.com/DarkCeptor44/modsync/commit/a2dc3f9e3186bcedbb5bdf03579a6b940288c3ec)) - DarkCeptor44
- (**backend**) let hashing functions take async reader instead of path - ([f7437bf](https://github.com/DarkCeptor44/modsync/commit/f7437bf09d92d1d3b88f634439a11a823ccef39c)) - DarkCeptor44
- (**backend**) add override flag in link_or_copy to allow forcing a copy if on the same filesystem - ([62099a4](https://github.com/DarkCeptor44/modsync/commit/62099a4630971e938cda1cbb8ae828d99e4680fe)) - DarkCeptor44
- (**backend**) expose sync::fs module and create a divan benchmark for collect_source_files - ([c8fb0aa](https://github.com/DarkCeptor44/modsync/commit/c8fb0aaf387e3f3a84c6d3847aa480d3822cb6e5)) - DarkCeptor44
- (**backend**) add very performant function to collect source files fast - ([342bd35](https://github.com/DarkCeptor44/modsync/commit/342bd35187f7880f9894516e5ada676bafad8041)) - DarkCeptor44
- (**backend**) more work on sync logic - ([da372bd](https://github.com/DarkCeptor44/modsync/commit/da372bddc66f9c38c18f4cc8cf67d9b3032f80e8)) - DarkCeptor44
- (**backend**) prepare for sync - ([03c7a82](https://github.com/DarkCeptor44/modsync/commit/03c7a82b22f8921f52d7628765be1edbf7ff4191)) - DarkCeptor44
- (**backend**) initial backend - ([20a47f9](https://github.com/DarkCeptor44/modsync/commit/20a47f995a2d0b099e4ac2dc5cc2dd0177ce0c1b)) - DarkCeptor44
- (**frontend**) add humanBytes function to print filesizes in KiB/MiB - ([386b0d1](https://github.com/DarkCeptor44/modsync/commit/386b0d1f4107d9dbf3954ad3f7cef334a0ea2ca4)) - DarkCeptor44
- (**frontend**) sort profiles in home page - ([d1d69ed](https://github.com/DarkCeptor44/modsync/commit/d1d69ed7ad127c67e61b0bb624c16a95cdf30516)) - DarkCeptor44
- (**frontend**) add i18n - ([86f03d0](https://github.com/DarkCeptor44/modsync/commit/86f03d0a35f79ec8fa87f0326d47f100ff31aa41)) - DarkCeptor44
- (**frontend**) make dev-mode frontend logging better - ([b21e197](https://github.com/DarkCeptor44/modsync/commit/b21e19750d9fa812e3d77cbe581c17195d2e0c65)) - DarkCeptor44
- (**frontend**) initial work on syncing frontend - ([4187119](https://github.com/DarkCeptor44/modsync/commit/41871196f966fb52303b57cf3291452fdfb062dc)) - DarkCeptor44
- (**frontend**) implement API to add and list profiles - ([6d4ae18](https://github.com/DarkCeptor44/modsync/commit/6d4ae18c3bea3f7f5e37af3558bc92d006e0043a)) - DarkCeptor44
- (**frontend**) finish the profile form - ([a468c6f](https://github.com/DarkCeptor44/modsync/commit/a468c6f80f65218856447440df38a7a9d7d9bdf4)) - DarkCeptor44
- implement profile delete - ([b2acb3d](https://github.com/DarkCeptor44/modsync/commit/b2acb3df01cb20a157be825a6ea112ea89cb877c)) - DarkCeptor44
- make sync logic use settings - ([c10ca3a](https://github.com/DarkCeptor44/modsync/commit/c10ca3a5e85696db3e97cd2735f82d28116f0113)) - DarkCeptor44
- implement settings - ([f36f5e2](https://github.com/DarkCeptor44/modsync/commit/f36f5e219ae92f54199a62279d87bb398193c785)) - DarkCeptor44
- implement dry run toggle for sync - ([3980d4a](https://github.com/DarkCeptor44/modsync/commit/3980d4a9426526412232776e8f791cf21b9d8a78)) - DarkCeptor44
- finish sync logic and match in ui - ([df87b6e](https://github.com/DarkCeptor44/modsync/commit/df87b6ee8f266a74d61cc2b49069bf7d204e0c9c)) - DarkCeptor44
- use two exclusion lists, one for sync and one for delete - ([0da5bf8](https://github.com/DarkCeptor44/modsync/commit/0da5bf897b29734d0000228e6e58c2838b9d4af3)) - DarkCeptor44
- allow profile editing - ([c82f1a8](https://github.com/DarkCeptor44/modsync/commit/c82f1a8567bf9087469ccb1fe58d26e8a9535349)) - DarkCeptor44
- make Svelte template into SvelteKit; add a navbar - ([f006b02](https://github.com/DarkCeptor44/modsync/commit/f006b02e3459e994c2f1f9fc190fb13c8b1b6886)) - DarkCeptor44
#### Style
- (**frontend**) replace Textarea with a fancier box - ([dfaf3d1](https://github.com/DarkCeptor44/modsync/commit/dfaf3d16ee488ee6ae9484567d2df35340188b20)) - DarkCeptor44
- (**frontend**) make disable on Toggle actually disable cursor - ([ea3760a](https://github.com/DarkCeptor44/modsync/commit/ea3760aa396d485bcc2ec7b3fc39fcd57ad262ed)) - DarkCeptor44
- (**frontend**) adjust some things; cleanup some things; fix some things - ([4bce386](https://github.com/DarkCeptor44/modsync/commit/4bce38638460c03fffd10e53f6c64efd3f9567ef)) - DarkCeptor44
- (**frontend**) adjust input field component to allow optional horizontal mode and prefix - ([2dcc3fb](https://github.com/DarkCeptor44/modsync/commit/2dcc3fb5e4812d4188be46cd3772bf533514f260)) - DarkCeptor44
- (**frontend**) Button component takes textSize into account - ([4dc14ee](https://github.com/DarkCeptor44/modsync/commit/4dc14eefe34d1603ecaed7c6070602b5aeb3531c)) - DarkCeptor44
- clean things up in the frontend - ([706da1b](https://github.com/DarkCeptor44/modsync/commit/706da1bee117c840c32b81251e5692252fbb5b63)) - DarkCeptor44

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).