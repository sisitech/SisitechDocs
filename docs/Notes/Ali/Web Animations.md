## Web Animations

### Introduction

Web animations are

- Angular Animations
- Animating with SVGs
- CSS Animations
- Animations on After Effects


### Angular Animations

Introducing animation into our angular animations using a 'CAL' stack which represents CSS animations, Angular animations and Lottie animations.

### Anatomy of an angular animation

1. Starts with a trigger name 
2. A transition pair
3. Animation sequence

``` javascript
export const fadeOutAnimation = 

    trigger('fadeOut', [
        transition('* => void', [
            style({position:'absolute'}),
            animate('0.5s', style({opacity: 0.3})),
            animate('0.5s', style({opacity: 0}))
        ])
    ])
```

We declare our animations inside our component i.e.

``` ts
@component({
    selector: SOMETHING,
    templateUrl: SOMETHING,
    animations: [
        fadeOutAnimation
    ]
})
```

add into the HTML element we are interested in using the ``alias trigger name @`` syntax. 

``` html

<li *ngFor="let item of items" class = "slide-in" @fadeOut>{{item.name}}</li>

```




### Interactive Web animations with SVG

SVG's are scalable vector graphics image formats that are scalable without affecting the resolution of an image file. SVG is a lot like HTML, it is an alternate universe version of HTML focused on graphics instead of documents.

SVG has a DOM and therefore we can manipulate graphics with code. 

SVG is built for drawing in a way that HTML and CSS are not. 

SVG allows for seamless handover and collaboration between designers and developers (perfect for a designer and a coder). The world of SVG is your playground. 

#### Tools to create SVG code

| Tool | Subscription |
|:------ | :------ |
| Adobe Illustrator | Paid |
| Inkscape | Free (Recommended) |
| Figma | Free |
| Freepik | Resource for free SVGs online |

!!! note 
        Use layers / groups in Adobe illustrator to arrange your vector graphics (i.e. body, hands, legs) etc. 
        The artboard is your viewport.

#### Clean and Optimize your code

Clean up your generated SVG code graphic with a tool like SVGOMG and this will clean up your code by removing doctypes, removing XML instructions, remove comments & metadata, merge styles etc. 

Group your elements i.e. 

``` html
<svg class = "neopet">
    <g id = "wings"></g>
    <g id = "body"></g>
    <g id = "head"></g>
</svg>
```

SVGs have an implicit drawing order therefore z-index doesn't work. (Head will be shown on top of wings)

#### SVG vs CSS

- In comparison to SVG animations, CSS animations start to get messy at scale and for complex animation. It is difficult to chain animations therefore we have to fake chain with delays (add durations to every chained animations). 

- Transforms applied to SVG elements can behave differently in different browsers (transforms are fundamental). The issue os how ``transform-origin`` is measured. ``transform-box`` has possibility to fix this problem however it is still incompatible in other browsers. 

### Javascript Animation Libraries

| Library | Transforms | Timelines | File Size | Licensing | 
|:------ | :------ | :------ | :------ | :------ |
| Anime | ✅ | ✅ | 6kb | MIT Free License |
| Greensock * | ✅ | ✅ | 18kb| | Freemium + Paid |
| Velocity.js | ✅ | doesn't have a native timeline| 15kb| MIT Free License |

#### Greensock Features

- DrawSVG Plugin (Drawing SVG path animations)
- MorphSVG Plugin (Morphing between different SVG paths)
- MotionPathPlugin (Perform animations on the browser using codepen instead of Illustrator)

#### Tweening

A tween is the name of a single movement in an animation i.e. a movement in space from point A to point B or a movement of state (one color to another) and the ease of tweening is what gives your animation to life. 

Timelines act like container where you can keep your tweens and help with organisations. 

The best thing about web animation is to do with the user experience. Interacting on the web brings a level of playfulness, fun and creativity that's difficult to get from traditional animations i.e. web audio API. 



### References
- [Interactive web animation with SVG (DevFest 2019)](https://www.youtube.com/watch?v=ZKrjsux7C38&t=1523s&ab_channel=GoogleDeveloperGroups)
- [Freepik](https://www.freepik.com/)
- [Figma](https://www.figma.com/)
- [Inkscape](https://inkscape.org/)
- [SVGOMG](https://jakearchibald.github.io/svgomg/)
- [Web Audio API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API)
