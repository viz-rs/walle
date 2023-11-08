# walle

A disk space recovery tool.

Helps us to regularly clean up projects that are not frequently accessed, 
automatically cleans up caches generated during development, etc.
For examples, `node_modules` in nodejs, `target` in rust, `vender` in php. 

Of course, you can also choose to delete the project or package it for backup.

## Getting Started

* Scan a project and add to tracking list

```
walle project scan
walle project scan --level 2
walle project scan --level 2 --path .
walle project add --
```
...

## License

This project is licensed under the [MIT license](LICENSE).

## Author

- [@fundon@fosstodon.org](https://fosstodon.org/@fundon)

- [@\_fundon](https://twitter.com/_fundon)
