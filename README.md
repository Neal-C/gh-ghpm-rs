# A gh-extension to manage privacy on github

> [!WARNING]
> Turning a starred repository into a private repository will lose all the stars  
> Current forks will remain public and will be detached from the repository.

> [!IMPORTANT]
> if it has >= 1 stars or is a fork, ghpm does not turn a repository into a private repository.  
> It does not turn your README repository (username/username) private because it's a special repository meant for public display

> [!NOTE]
> I am not sponsored by github, nor affiliated, but you can change that by pinging them on social media. And ask for this functionality to be integrated directly into the gh CLI

## Requirements 

- The Github CLI https://cli.github.com/

## Installation

### supported operating systems

- linux-amd64
- linux-arm64
- darwin-arm64
- darwin-amd64

### not supported operating systems

- freebsd-amd64
- freebsd-arm64
- linux-386
- windows-386.exe
- windows-amd64.exe
- windows-arm64.exe

```bash
gh extension install Neal-C/gh-ghpm-rs
```

List your extensions

```bash
gh extension list
```

Upgrade

```bash
gh extension upgrade ghpm-rs
```

Uninstall

```bash
gh extension remove ghpm-rs
```

## Usage

```bash
# prints help message
gh ghpm-rs --help
```

```bash
# turns all your repositories private (except starred repos)
gh ghpm-rs thanos_snap
```

## Only turn 1 repository private

The github cli already supports turning 1 repository private: https://cli.github.com/manual/gh_repo_edit

```bash
gh repo edit myusername/myrepository --visibility private
```

## Roadmap

- [x] switch every repositories to private (excluding repos with >= 1 stars)
- [ ] Lobby github to provide a batch request endpoint, so that it's only 1 HTTPS request and not O(n) HTTPS requests
- [ ] Lobby github to add this functionality to the gh CLI

## Contributing

I am open to random pull requests that do at least 1 of the following :
- cross items off the roadmap
- fix typos
- add tooling
- add tests
- add/improve documentation
- improve CI/CD

### Local setup

0. if gh-ghpm-rs is already installed, uninstall it with 
```bash
gh extension remove ghpm-rs
```
1. Clone the repo
2. Run `cargo build --release`
```bash
cargo build --release
```
3. ```cp target/release/gh-ghpm-rs .```
```bash
cp target/release/gh-ghpm-rs .
```
4. Run `gh extension install .`
```bash
gh extension install .
```
5. Run `gh ghpm-rs --help`
```bash
gh ghpm-rs --help
```

## How to permanently delete or hide data from a repository ?

Only sure way is to contact github support : https://support.github.com/

When in doubt, revoke and rotate your keys. Or better yet, automate it.

---

Made with 💞 love 💞 for developers by a developer ❤️