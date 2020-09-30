# rust-image-write-app
Write some text on a url based image with only requests. This is useful for url image markdowns.

# Demo
The URL must be formatted like this:
`https://image-write-app.herokuapp.com/?x=<x>&y=<y>&size=<fontsize>&text=<text>&url=<url>`

**<x>**: X position of the text
**<y>**: Y position of the text
**\<fontsize>**: Font size of the text
**\<text>**: Text itself
**\<url>**: Base Image URL

All parameters must be URL encoded. (You can use [a URL Encoder](https://www.urlencoder.org/))

The URL you provide has to return an image on request. (And yes you can provide another image-write-app url to write multiple texts.)

# Example
![](https://image-write-app.herokuapp.com/?x=640&y=420&size=45.0&text=MegaCrafter&url=https://www.pngitem.com/pimgs/m/346-3468947_images-rust-lang-ar21-rust-programming-language-logo.png)

# Nested Example
![](http://image-write-app.herokuapp.com/?x=340&y=420&size=45.0&text=MegaCrafter&url=https%3A%2F%2Fimage-write-app.herokuapp.com%2F%3Fx%3D640%26y%3D420%26size%3D45.0%26text%3DMegaCrafter%26url%3Dhttps%3A%2F%2Fwww.pngitem.com%2Fpimgs%2Fm%2F346-3468947_images-rust-lang-ar21-rust-programming-language-logo.png)

# TODO
- Specifying colors for the text
- Filters on the base image
- Color-only images without url
- Gradient-only images without url
- Specifying fonts for the text
- Drawing geometry other than text 
