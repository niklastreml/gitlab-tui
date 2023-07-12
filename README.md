# Gitlab TUI 🚀
![Continuous Integration](https://github.com/NiklasTreml/gitlab-tui/workflows/Release/badge.svg?branch=main&event=push)
![](https://img.shields.io/badge/license-Apache%202-blueviolet.svg)
[![Crates.io](https://img.shields.io/crates/v/gitlab-tui.svg)](https://crates.io/crates/gitlab-tui)
![](https://img.shields.io/github/v/release/NiklasTreml/gitlab-tui?color=%23c694ff)

A TUI app for interacting with GitLab issues and merge requests.

![image](https://github.com/NiklasTreml/gitlab-tui/assets/27763017/e126e2cd-1e44-43ba-8e23-d124daa59621)

## Installation ⚙️

### Cargo 📦

You can use Cargo to download, compile, and install Gitlab TUI on your machine:

```bash
cargo install gitlab-tui
```

## Setup 🛠️

To authenticate the GitLab API, Gitlab TUI uses the same `.netrc` config that Git uses for HTTP cloning. If you have already set up Git with `.netrc`, there is a good chance it will work immediately. If not, you will need to create one:

1. Create an API token in GitLab with `read-api` scope.
2. Create a `.netrc` file:

   - On Linux and macOS:

     ```bash
     touch ~/.netrc
     ```

   - On Windows:
     ```powershell
     New-Item ~/.netrc
     ```

3. Create an entry for your GitLab instance:

   - For GitLab.com:

     ```netrc
     machine gitlab.com
       login __token__
       password <YOUR_TOKEN>
     ```

   - For self-hosted GitLab:
     ```netrc
     machine <YOUR_GITLAB_INSTANCE>
       login __token__
       password <YOUR_TOKEN>
     ```

That's it! Now just `cd` into your repo and run `gitlab-tui`. The program will parse your Git SSH or HTTP remote to figure out the GitLab API URL and fetch all issues and merge requests of your project.

## Usage 🚀

The binary is called `gitlab-tui`. You can bring up the UI by running it without any arguments. If you need to use a different remote than `origin`, you can use the `-r` flag to override the remote.
