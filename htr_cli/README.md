# htr_cli

The CLI usage of the HTML to React transform

## Usage

Transform props to React.

```sh

htr transform --html '<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">'

```

Transform to React component.

```sh

htr transform --html '<div class="something" for="mystuff" tabindex="2" style="color: white; background-color: black">' --component-name 'Main'

```
