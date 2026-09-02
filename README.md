# finder-rs

Bookmark places and commands. Organize them by context. Expand them directly into your shell.

## About

The concept of bookmarks has been around for centuries.

We used to leave them in books. Today, we also use them in web browsers.
`finder-rs` brings the same idea to the shell.

When working on complex projects, we often need to remember long paths and commands.
`finder-rs` lets you give them simple names and recall them whenever you need them.

Bookmarks are explicit and organized into collections, so the same name can refer to different things in different project contexts.

With shell integration, a bookmark can be expanded directly into your command line. You can inspect it, modify it, combine it with other commands, and execute it when you're ready.

## `zoxide` and `finder-rs`

`finder-rs` is not a replacement for `zoxide`. They solve different problems and work well together.

> `zoxide` remembers where you have been.
> `finder-rs` remembers what you have named.

* **Explicit, not learned** — `zoxide` learns from your directory history, while `finder-rs` uses explicitly defined bookmarks.

* **Collections as context** — the same bookmark name can refer to different places or commands depending on the active collection.

* **Places and commands** — `finder-rs` bookmarks not only directories, but also commands you want to recall.

* **Shell expansion** — bookmarks can be expanded directly into the current command line, where you can inspect, edit, combine, or execute them.

* **Windows, Linux and macOS** — `finder-rs` is designed to work across all three platforms.

Use `zoxide` to navigate based on where you have been. Use `finder-rs` for things you have deliberately named.

