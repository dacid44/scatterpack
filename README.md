# Scatterpack
Scatterpack is a packing list app designed to take some of the stress out of packing. It's built to solve the parts of packing for a trip which stress me out every time, no matter if it's a weekend camping trip or a semester abroad.


## Features (eventually)
- Customizable packing list templates, which scale to the length of the trip
- Keeping track of unique, individual items (e.g. your favorite shirt) with thumbnail images from your camera
- Recording where items are, even if they aren't packed yet (e.g. laundry in the dryer, toothbrush which you still need to brush teeth the evening before you leave) and compiling reminders of these items
- (maybe) gamification for ADHD people who need those dopamine hits for motivation


## Architecture
This project is built using [Tauri V2](https://v2.tauri.app/). Tauri is a framework which bundles a Rust backend with your platform's native web view for the UI. The frontend is built in [Svelte](https://svelte.dev/), with components from [Flowbite](https://flowbite-svelte.com/) and styling with [Tailwind](https://tailwindcss.com/).

## Building
Node v20 or greater is required.

Install the Tauri V2 CLI ([instructions](https://v2.tauri.app/start/create-project/#manual-setup-tauri-cli), step 2), then run:
```bash
git clone https://github.com/dacid44/scatterpack.git
cd scatterpack
npm install
npm tauri dev # or: npm tauri build
```


## Platforms
Android is the eventual plan for this app, but Windows and Linux should be fully supported. I don't own the hardware to build for or test on Apple platforms, but I may eventually incorporate GitHub or other CI to build for iOS and macOS once the project is nearer to completion.


## License
This project is licensed under the [Mozilla Public License, version 2.0](https://www.mozilla.org/en-US/MPL/2.0/).

https://www.tldrlegal.com/license/mozilla-public-license-2-0-mpl-2


## Contributing
This is intended as a solo project for now. Feel free to submit issues or PRs if you wish, but don't expect a quick response.


## Tools
I'll share some of the tools I've been using for this project here, just in case it helps anyone else. They shouldn't be required to build or run the project, though.
- [jj](https://github.com/martinvonz/jj), a git-compatible version system that I find far more intuitive than git itself
- [Bun](https://bun.sh/), a replacement for NodeJS and NPM. I've been using it as a lightning-fast package manager without the need to manage npm versions across Windows and Linux, without replacing the Node runtime.
- [Node Version Manager](https://github.com/nvm-sh/nvm), a script to handle having multiple versions of Node (and npm) installed on a Linux (or Mac) system.
- [NVM for Windows](https://github.com/coreybutler/nvm-windows), same deal but for Windows.