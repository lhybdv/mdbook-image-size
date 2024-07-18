# mdbook-image-size

A mdbook preprocessor which support image size syntax

## size

### width & height

From

```md
![the alt](path/to/your/image "the title" =500x400)
```

To

```html
<p><img src="path/to/your/image" alt="the alt" title="the title" width="500" height="400"></p>
```

### width only

From

```md
![the alt](path/to/your/image "the title" =500x)
```

To

```html
<p><img src="path/to/your/image" alt="the alt" title="the title" width="500"></p> 
```

### height only

From

```md
![the alt](path/to/your/image "the title" =x400)
```

To

```html
<p><img src="path/to/your/image" alt="the alt" title="the title" height="400"></p>
```

> [!TIP]
> No alt or title is ok.

## align

### left

left is default

### center

From

```md
![the alt](path/to/your/image "the title" =500x400 center)
```

To

```html
<p style="text-align:center"><img src="path/to/your/image" alt="the alt" title="the title" width="500" height="400"></p>
```

### right

From

```md
![the alt](path/to/your/image "the title" =500x400 right)
```

To

```html
<p style="text-align:right"><img src="path/to/your/image" alt="the alt" title="the title" width="500" height="400"></p>
```

## Installation

```sh
cargo install mdbook-image-size
```

add it as a preprocessor in book.toml

```toml
[preprocessor.image-size]
command = "mdbook-image-size"
```

