# Design

## Summary of design structures

Every algorithm can be described with four logic structures, and these can be used as basic building blocks in small and in very large and complex algorithms. When using the structures in a disciplined manner, the algorithm described in the pseudo-code is called a structured algorithm, and the design process is sometimes called structured design.

Using simple design concepts with pseudo-code and flowcharts, an algorithm can be built using the following basic design structures:

- Sequences
- Selections
- Loops

### Sequences

In the sequence structure an action is done after another in a certain order.

![Sequence design structure](https://upload.wikimedia.org/wikipedia/commons/7/75/Flowchart_sequence.jpg)

### Selection

This is where you select or choose between two or more flows. The choice is decided by asking some sort of question. The answer determines the path (or which lines of code) will be executed.

![Selection design structure](https://i.ytimg.com/vi/LLpLqw5gpl0/maxresdefault.jpg)

### Loops/iterations

They allow some code (one to many lines) to be executed (or repeated) several times. The code might not be executed at all (repeat it zero times), executed a fixed number of times or executed indefinitely until some condition has been met. Also known as looping because the flowcharting shows the flow looping back to repeat the task.

![While loop](https://www.rff.com/flowchart_structure_loop.png)
![For loop](https://study.com/cimages/videopreview/n4btwblifr.jpg)

## Type scale generators

Powered by Google Fonts, the type scale generator is a tool for creating type scales and corresponding code.

The type scale is a combination of thirteen styles that are supported by the type system. It contains reusable categories of text, each with an intended application and meaning.

### Usage

#### Platforms

##### Web (.SCSS)

```
$mdc-typography-styles-headline1: (
  font-family: unquote("Roboto"),
  font-size: 96
);
```

##### Android (.xml)

Add fonts via Android Studio using [Downloadable Fonts](https://developer.android.com/guide/topics/ui/look-and-feel/downloadable-fonts#via-android-studio).

```
<resources>
  <style name="TextAppearance.MdcTypographyStyles.Headline1" parent="TextAppearance.MaterialComponents.Headline1">
      <item name="fontFamily">@font/roboto</item>
      <item name="android:fontFamily">@font/roboto</item>
      <item name="android:textSize">96sp</item>
  </style>
</resources>
```

##### Flutter (.dart)

Look at [flutter.dev](https://docs.flutter.dev/cookbook/design/fonts) for instructions on how to add these fonts to your project.

```

TextTheme(
  headline1: GoogleFonts.roboto(
    fontSize: 96,
    fontWeight: FontWeight.w300,
    letterSpacing: -1.5
  )
)

```

## References

- [Design structures](https://flylib.com/books/en/2.517.1.55/1/)
- [Structured Programming](https://press.rebus.community/programmingfundamentals/chapter/structured-programming/)
- [The type system in Material design](https://material.io/design/typography/the-type-system.html#type-scale)
