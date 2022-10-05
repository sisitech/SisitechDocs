# Sisitech Carousel Library

[Sisitech Carousel](https://github.com/sisitech/sisitech-carousel) is an angular library to easily create and control slideshows for different html components.

![lib-carousel](../images/multi-cmp-car.png)

## Installation
```
npm install @sisitech/carousel@0.0.3
```

## Usage

Import the library

```typescript hl_lines="6" title="app.module.ts" linenums="1"
import { CarouselModule } from '@sisitech/carousel'
...

imports: [
    ...
    CarouselModule
    ...
  ],
```

## About

The sisitech carousel library has the following properties :

| Input | Description|
| :----------- |:----------- |
| **[sliderOptions]** | Takes an array of items that each slide will have. It's necessary to prepare the different components for the data given by the sliderItems array.  |
| **[sliderItems]** | <p>Takes an array of slide items</p><p>The array takes the following items :</p><ol><li>id: string - used to uniquely identify each slide</li><li>link: string -the target page for when the slide is clicked</li><li>img: string - the header image of the slide</li><li>alt: string - the alternative text in case the image isn't rendered</li><li>title: string - the slider title text</li><li>desc: string - the slider description text</li><li>class: string - the class that can be used to uniquely style the different parts of each slide</li><li>linktxt: string - the text that describes the call to action for each slide</li></ol> |
| **[slidesToShow]** | Takes a number and determines the number of slides that will be in view. <br> The default value is 3. |
| **[autoSlide]** | Takes a bolean value and makes the slides move automatically. <br> The default value is false.|
| **[controls]** | Takes a bolean value and toggles between shwoing and hiding the previous and next buttons <br> The default value is true. |
| **[indicators]** | Takes a bolean value and toggles between shwoing and hiding the indicator dots <br> The default value is true. |
| **[slideInterval]** | Takes a number and determines the speed at which the slides move on autoSlide <br> The default value is 4000 (4s).  |

!!! note
    The inputs [sliderItems] and [sliderOptions] are compulsory inputs.

## Example

### Single Component Carousel

The library can be used to create a slideshow for single components, e.g. images only, as shown below.


```html title="app.component.html" linenums="1"

<lib-carousel id="partnersTwo"
  [sliderItems]="partnersCarousel" 
  [sliderOptions]="partnersOptions"
  [slidesToShow] = 4>
</lib-carousel>

```

```ts title="app.component.ts" linenums="1"
    partnersOptions = ['id', 'img', 'alt', 'class', 'link'];

  partnersCarousel: any = [
    {
      id: '0',
      link: "https://www.unicef.org/kenya/",
      img: "../../assets/chuo-carousel/unicef.svg",
      alt: "unicef",
      class: "unicef"
    },
    {
      id: '1',
      link: "https://www.wvi.org/somalia",
      img: "../../assets/chuo-carousel/world-vision-01.svg",
      alt: "world vision somalia",
      class: "world-vision"
    },
    {
      id: '2',
      link: "https://kenya.savethechildren.net/",
      img: "../../assets/chuo-carousel/save-the-children-02.svg",
      alt: "save the children",
      class: "save-the-children"
    },
    {
      id: '3',
      link: "https://naconek.ke/",
      img: "../../assets/chuo-carousel/naconek-03.svg",
      alt: "naconeck",
      class: "naconek"
    },
    {
      id: '4',
      link: "https://www.responseinnovationlab.com/somalia",
      img: "../../assets/chuo-carousel/somril-04.svg",
      alt: "response innovation lab somalia",
      class: "ril"
    },
    {
      id: '5',
      link: "https://en.wikipedia.org/wiki/Puntland",
      img: "../../assets/chuo-carousel/puntland.png",
      alt: "puntland state",
      class: "puntland"
    },
    {
      id: '6',
      link: "https://www.education.go.ke/",
      img: "../../assets/chuo-carousel/moe.jpeg",
      alt: "ministry of education",
      class: "ministry-of-education"
    }
  ]
```

!!! caution
    The first id value for the sliderItems needs to be 0 not 1. Otherwise the first slide will not be shown.
    Working to improve this restriction in future releases.

**Result**

![lib-carousel](../images/sng-cmp-car.png)

To create a more automated slideshow, one can remove both the controls and the indicators as well as activate autoslide with a set interval. The code snippet would thus look like this :


```html title="app.component.html" linenums="1"
<lib-carousel id="partnersOne"
  [sliderItems]="partnersCarousel" 
  [sliderOptions]="partnersOptions"
  [slidesToShow] = 5
  [autoSlide]="true" 
  [controls]="false" 
  [indicators]="false"
  [slideInterval]=2000>
</lib-carousel>
```

**Result**

![lib-carousel](../images/sng-cmp-nc.png)


## Multi-Component Carousel

The library can also be used to create a slideshow for more complex html elements, e.g. cards e.t.c, as shown below.

```html title="app.component.html" linenums="1"
  <lib-carousel id="wavvyOne"
  [sliderItems]="wavvyCarousel"
  [sliderOptions]="wavvyOptions"
  [slidesToShow] = 3>
</lib-carousel>
```

```ts title="app.component.ts" linenums="1"
wavvyOptions = ['id', 'img', 'alt', 'title', 'desc', 'link', 'class', 'linktxt'];

 wavvyCarousel = [
    {
      id: '1',
      img: '../../assets/wavvy-carousel/weche-cover.png',
      alt: 'weche',
      desc: "Kenyan record label Gondwana KE is ever feeding our musical atmosphere with profound and authentic African style music",
      title: "Punk Mbedzi, Euggy & Akoth Jumadi",
      link: 'https://youtu.be/Re-bOsZyBiU',
      class: 'weche',
      linktxt: 'WATCH VIDEO'
    },
    {
      id: '2',
      img: '../../assets/wavvy-carousel/hutia-cover.png',
      alt: 'hutia',
      desc: "Rising Deejays Ally Fresh & Mura drop their hit single & music video Hutia. An afrocentric laid back single perfect for setting vibrations.",
      title: "Ally Fresh & DJ Mura K.E",
      link: 'https://youtu.be/Re-bOsZyBiU',
      class: 'hutia',
      linktxt: 'WATCH VIDEO'
    },
    {
      id: '3',
      img: '../../assets/wavvy-carousel/hutia-remix-cover.png',
      alt: 'hutia remix',
      desc: "Ally Fresh & DJ Mura K.E thank their fans for 8,000 views in two months. A fresh remix EP just released on Wavvy exclusively",
      title: "Various Artists",
      link: 'https://youtu.be/Re-bOsZyBiU',
      class: 'hutia-remix',
      linktxt: 'WATCH VIDEO'
    },
    {
      id: '4',
      img: '../../assets/wavvy-carousel/weche-cover.png',
      alt: 'weche',
      desc: "Kenyan record label Gondwana KE is ever feeding our musical atmosphere with profound and authentic African style music",
      title: "Punk Mbedzi, Euggy & Akoth Jumadi",
      link: 'https://youtu.be/Re-bOsZyBiU',
      class: 'weche',
      linktxt: 'WATCH VIDEO'
    },
    {
      id: '5',
      img: '../../assets/wavvy-carousel/hutia-cover.png',
      alt: 'hutia',
      desc: "Rising Deejays Ally Fresh & Mura drop their hit single & music video Hutia. An afrocentric laid back single perfect for setting vibrations.",
      title: "Ally Fresh & DJ Mura K.E",
      link: 'https://youtu.be/Re-bOsZyBiU',
      class: 'hutia',
      linktxt: 'WATCH VIDEO'
    },
    {
      id: '6',
      img: '../../assets/wavvy-carousel/hutia-remix-cover.png',
      alt: 'hutia remix',
      desc: "Ally Fresh & DJ Mura K.E thank their fans for 8,000 views in two months. A fresh remix EP just released on Wavvy exclusively",
      title: "Various Artists",
      link: 'https://youtu.be/Re-bOsZyBiU',
      class: 'hutia-remix',
      linktxt: 'WATCH VIDEO'
    },
  ]
```

**Result**

![lib-carousel](../images/multi-cmp-car.png)

!!! note
    Selecting an image using the indicator causes it to appear as the first image on the carousel.

## Styling

It's advised to add an id attribute to each carousel to aid in targeting the styling to a specific carousel in the event that your page has more than one carousel.

```html hl_lines="1" title="app.component.html" linenums="1"
  <lib-carousel id="wavvyOne"
  [sliderItems]="wavvyCarousel"
  [sliderOptions]="wavvyOptions"
  [slidesToShow] = 3>
</lib-carousel>
```

!!! note
    All the custom styling for the carousel should be put in the global styles.scss file.

```scss
#wavvyOne {
    .slider-img{ //styles the header image for all the slides
        max-width: 90%;
        min-width: 45%;
    }
    .carousel-btn { // styles the next and previous buttons
        color: white;
    }
    .dot { // styles the indicator dots
        background-color: white;
    }
    .slider-meta { // styles the all the text in the slider 
        color: white;
        font-weight: 300;
        font-family: 'Lato', sans-serif;
    }
    .slider-title { // styles the slider title
        font-weight: 400;
    }
    .slider-desc{ //styles the slider description
        font-weight: 300;
    }
    // the classes below are used to style each slide individually
    // these are the classes we passed in the sliderItems array above
    .hutia {
        img{
            width: 70%;
            margin-top: 10px;
            margin-left: 15px;
        }
        a, .slider-title {
            color: $hutia;
        }
    }
    .weche {
        a, .slider-title {
            color: $weche;
        }
    }
    .hutia-remix {
        a, .slider-title {
            color: $hutia-remix;
        }
    }
}
```    

## Responsivness

| Screen Size | Number of Slides in View|
| :----------- |:----------- |
| 320px - 400px | one |
| 401px to 600px | three |
| 601px and above | As determined by user <br> The default is 4 |
