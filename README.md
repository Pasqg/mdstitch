# mdstitch

A rust cli util to merge multiple markdown files into one.

## Stitching markdown files

`mdstitch` takes one argument which is the root (markdown) file. It looks for `@mdstitch[path/to/file]` instructions and
replaces that file content in its place.

Example: `mdstitch --root path/to/markdown.md --output readme.md`

path/to/markdown.md:

```
# Title

....

@mdstitch[relative/path/to/other.md]

@mdstitch[another/relative/path/to/another.md]

## Conclusion

The turtle was very happy!
```

relative/path/to/other.md:

```
## Chapter 1

A turtle walked slowly in a foggy green wood.

```

another/relative/path/to/another.md:

```
## Chapter 2

The turtle found a berry and ate it with delight.

```

result.md:

```
# Title

....

## Chapter 1

A turtle walked slowly in a foggy green wood.

## Chapter 2

The turtle found a berry and ate it with delight.

## Conclusion

The turtle was very happy!

```

### Nested stitching

Nested stitching is also supported. Mdstitch will evaluate the hierarchy depth-first, with the implicit assumption that
there are no cycles.

Currently, cycles are not explicitly handled. This will result in errors running mdstitch. Future plans include
detecting loops and throwing better warnings/errors.

### Stitching directive

The stitching directive is by default:

`@mdstitch[/path/to/file]`

In case your markdown should contain
already strings such as `@mdstitch[...]`, or you just don't like to type that, the stitching directive can be customized
with the flag `-d/--directive <PATTERN>`.