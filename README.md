# mdbook-image-size

A mdbook preprocessor which support image size syntax

## width & height

From

```md
![the alt](path/to/your/image "the title" =500x400)
```

To

```html
<img src="path/to/your/image" alt="the alt" title="the title" width="500" height="400">
```

## width only

From

```md
![the alt](path/to/your/image "the title" =500x)
```

To

```html
<img src="path/to/your/image" alt="the alt" title="the title" width="500"> 
```

## height only

From

```md
![the alt](path/to/your/image "the title" =x400)
```

To

```html
<img src="path/to/your/image" alt="the alt" title="the title" height="400">
```
TIP: No alt and title is ok.
