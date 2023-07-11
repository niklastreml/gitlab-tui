Sure! Here's an updated version of the README with emojis:

# Gitlab TUI 🚀

A TUI app for interacting with GitLab issues and merge requests.

![image](https://github.com/NiklasTreml/gitlab-tui/assets/27763017/7efaabd3-dd68-4cb8-9e6f-9833a1916b81)

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
