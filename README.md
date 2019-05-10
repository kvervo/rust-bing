# RUST BING

A MacOS application written in Rust that would set the wallpaper image using the Bing image of the day.

## MVP

```text
*When* I get tired of my current wallpaper *I want to* update it easily using bing images (because they are cool) automatically *So I can* skip the need to do it manually
```

## Algorythm

(1) Get image from Bing API:
<http://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US>
where:

* format: JSON | XML
* idx: [0-7] number of days to go back. Maximum number of days you can go back index is 7
* n: [0-8] number of result images. Maximum number of images you can retrieve is 8.
* mkt: region (en-US, ru-RU, etc)

(2) Parse response
Get attributes:

* url
  Used to download wallpaper image
* startdate
  Used in filename generation for local copy of wallpaper image

Example response:

```json
{
  "images": [
    {
      "startdate": "20190509",
      "fullstartdate": "201905090700",
      "enddate": "20190510",
      "url": "/th?id=OHR.SerengetiZebra_EN-US5631881768_1920x1080.jpg&rf=LaDigue_1920x1080.jpg&pid=hp",
      "urlbase": "/th?id=OHR.SerengetiZebra_EN-US5631881768",
      "copyright": "Zebras in Serengeti National Park, Tanzania (© pchoui/Getty Images)",
      "copyrightlink": "http://www.bing.com/search?q=zebra+(animal)&form=hpcapt&filters=HpDate:%2220190509_0700%22",
      "title": "The herd heads north",
      "quiz": "/search?q=Bing+homepage+quiz&filters=WQOskey:%22HPQuiz_20190509_SerengetiZebra%22&FORM=HPQUIZ",
      "wp": true,
      "hsh": "f5dd604c097a616cde21ecd62e156d97",
      "drk": 1,
      "top": 1,
      "bot": 1,
      "hs": []
    }
  ],
  "tooltips": {
    "loading": "Loading...",
    "previous": "Previous image",
    "next": "Next image",
    "walle": "This image is not available to download as wallpaper.",
    "walls": "Download this image. Use of this image is restricted to wallpaper only."
  }
}
```

(3) Download Image and save locally
Image should be saved to `~/Wallpapers` folder.
Image filename format should be as follows:

```text
bing_{startdate}.jpg
```

(4) Use local file to set background wallpaper
