> [!IMPORTANT]
> The tool has been deprecated in favor of [ejf-utils-2], which is a TypeScript rewrite of the same tool using the same underlying library, but adding more features such as support for composite fonts (adding two or more fonts into the same .ejf file).

# EJF Generator

Generates .ejf font files for the MicroUI embedded platform. For more information about how fonts are handled at MicroUI level, see [Fonts on MicroEJ Developers](https://docs.microej.com/en/latest/ApplicationDeveloperGuide/UI/MicroUI/fonts.html).

By default fonts are created in MicroEJ using the built-in font editor.

## Configuration format

As described in [Declarative approach](#declarative-approach), the font generation in controlled via a configuration file, called a _manifest_.

The manifest is in the [TOML](https://toml.io/en/) format and an example can be found in [`input.toml.example`](./input.toml.example).

A manifest can contain one or more fonts to be generated, delimitated by a `[[font]]` section.

Each font can have the following options:

<dl>
<dt><code>char_range</code></dt>
<dd>The Unicode range of characters to generate. The range is similar to the range provided to the <code>.fonts.list</code> file in the MicroEJ SDK stack. A single character is represented by its hex code (e.g. 0x41 for the character <code>A</code>). A range of characters can be added by using <code>-</code>, e.g. <code>0x60-0x80</code> to embed the characters starting from 0x60 (inclusive) up to 0x80 (exclusive).</dd>
<dt><code>input</code></dt>
<dd>The absolute or relative path to the .ttf font to be used for generating the fonts.</dd>
<dt><code>output</code></dt>
<dd>The absolute or relative path to the .ejf font that will be created by the font generator.</dd>
<dt><code>size</code></dt>
<dd>The size of the font to generate. The font size is passed to the freetype library directly as the <code>char_width</code> to the <a href="https://freetype.org/freetype2/docs/reference/ft2-sizing_and_scaling.html#ft_set_char_size"><code>FT_Set_Char_Size</code></a> method. The definition for the value is "The nominal width, in 26.6 fractional points.".</dd>
<dt><code>skip_control_characters</code></dt>
<dd>Set to <code>true</code> in order to not embed control characters, that is characters that are not meant to be displayed. This can help reduce the amount of unwanted characters in the font, that only increase the memory consumption of the file. These characters are determined to the <a href="https://www.unicode.org/versions/latest/">Unicode Standard</a>, defined as the code points with the general category of <code>Cc</code>.</dd>
<dt><code>add_null_character</code></dt>
<dd>The MicroUI font engine always uses the first character in a font if it cannot find a particular character. This can sometimes be inconvienient and it is preferable to not display any character at all. To do so, set this value to <code>true</code> which will generate a NULL character (0x00) with a fixed width of 1px.</dd>
<dt><code>dpi</code></dt>
<dd>The DPI (dots per inch) value to pass to the freetype library as the <code>vert_resolution</code> when calling the <code>FT_Set_Char_Size</code> method.</dd>
</dl>

## Advantages over the original EJF generator

### Declarative approach

The main highlight of the EJF generator is that font declaration is declarative. For example, when adding a new character in the original font generator from MicroEJ, the process is as follows:

1. Open the EJF file in MicroEJ SDK.
2. Select Import, and select "system font".
3. Select the right font and font variation that corresponds to the font that is being modified.
4. Select the right "Size (pt)" for the font that is being modified.
5. Select the right rendering options (bold, italic, antialiasing).

This is an imperative approach as the user has to take the actions necessary to update the font. Whereas in the declarative approach all needs to be done is to add the desired character range to be imported (say for example `0x60` to `0x80`):

```diff
-char_range="0x0, 0x40-0x50"
+char_range="0x0, 0x40-0x50,0x60-0x80"
```

And then regenerate the font.

### Platform-independent font generation

Font generation in MicroEJ uses the native stack in order to render the font. This can lead to issues where a font generated on a Windows platform looks different than a Linux-based generation, for example.

The EJF generator builds its fonts using the [freetype](http://freetype.org/) library. This means that the generated font is the same regardless of the platform it is built on. 

### Reproducible by default

When building the fonts, the generator will rebuild all the fonts in a given manifest. However, it takes extra steps to ensure that the build files are always the same if their declaration did not change (including resetting the timestamp in the .ejf archive).

Whenever a change is made, only the .ejf that had the change will appear as modified in Git. This makes it easy to distinguish the scope of any regeneration of fonts.

## Disadvantages over the original EJF generator

### No support for extra characters

The MicroEJ generator allows adding glyphs by adding them as additional characters based on another font or an image. This is (currently) not possible with the EJF generator, as it will only import characters that are present in the font.

### No support for adjustable font characteristics

In MicroEJ SDK, it's possible to adjust:

* At character level:
  * left and right margins.
  * size
* At font level:
  * height
  * baseline
  * space size

All these fields are not adjustable in the EJF generator as they will be determined directly from the font. This has an advantage in the sense that it leaves little to no space for human error, but sometimes it can be desirable to change some of these fields. 
