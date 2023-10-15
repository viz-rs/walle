# walle

A disk space recovery tool.

Helps us to regularly clean up items that are not frequently accessed, 
automatically cleans up caches generated during development, etc.
For example, `node_modules` in nodejs, `target` in rust, `vender` in php. 

## Getting Started

* Scan a project and add to tracking list

```
walle add .
```

* Remove a project from tracking list

```
walle remove --name [project]
```

* Show tracking list

```
walle list
```

...

## License

This project is licensed under the [MIT license](LICENSE).

## Author

- [@fundon@fosstodon.org](https://fosstodon.org/@fundon)

- [@\_fundon](https://twitter.com/_fundon)
