# MD-Stitch

A rust util to merge multiple markdown files into one.

## Stitching markdown files

`mdstitch` takes one argument which is the root (markdown) file. It looks for `@mdstitch[path/to/file]` instructions and
replaces that file content in its place.

Example: `mdstitch --root path/to/markdown.md --output readme.md`

path/to/markdown.md:
```
# Title

....

@mdstitch["other/path/other.md"]
@mdstitch[another/path/another.md]

## Conclusion

The turtle was very happy!
```

other/path/other.md:
```
## Chapter 1

A turtle walked slowly in a foggy green wood.

```

another/path/another.md:
```
## Chapter 2

The turtle found a berry and ate it with delight.

```

result.md:
```
# Title

## Chapter 1

A turtle walked slow in the woods.

## Chapter 2

The turtle found a berry and ate it with delight.

## Conclusion

The turtle was very happy!
```


